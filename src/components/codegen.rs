use std::collections::HashMap;

use crate::{InstructionPart, Lexem, LexemType, Token, REGISTERS_TO_VAL};

#[derive(Debug)]
pub struct CodeGen<'a>{
    source_filename: &'a str,
    origin: usize,
    cursor: usize,
    tokens: &'a[Token],
    instructions: &'a HashMap<&'static str,Vec<InstructionPart>>,
    fix_addr: HashMap<String,usize>,
    str_instructions: Vec<String>,
    to_fix: Vec<(Lexem, usize, usize)>,
    pub bytes: Vec<u8>
}

fn get_value_from_number_token<'a>(filename: &'a str, lexem: &Lexem) -> usize{
    match lexem.ttype{
        LexemType::Number { radix } => {usize::from_str_radix(&lexem.value, radix as u32).unwrap()}
        _ => {
            println!("{}:{}:{} expected number", filename, lexem.row, lexem.col);
            std::process::exit(1);
        }
    }
}

impl CodeGen<'_>{
    pub fn new<'a>(source_filename: &'a str, tokens: &'a[Token], instructions: &'a HashMap<&'static str,Vec<InstructionPart>>) -> CodeGen<'a>{
        CodeGen{
            source_filename,
            origin: 0,
            cursor: 0,
            tokens,
            instructions,
            fix_addr: HashMap::new(),
            to_fix: Vec::new(),
            str_instructions: Vec::new(),
            bytes: Vec::new()
        }
    }

    pub fn str_to_bytes(self: &Self, str: &String) -> [u8; 2]{
        let mut ret: usize = 0;

        for (i,ch) in str.chars().enumerate(){
            match ch{
                '1' => {
                    ret |= 1 << str.len()-i-1;
                },
                '0' => {
                    ret &= !(1 << str.len()-i-1);
                },
                _ => {
                    println!("Converting Instructions to Bytes: Expected 1 or 0 got {}", ch);
                    std::process::exit(1);
                }
            }
        }

        [((ret >> 8 ) & 0xFF) as u8, (ret & 0xFF) as u8 ]
    }

    pub fn gen(self: &mut Self){

        let mut fix_index: usize = 0;

        for token in self.tokens.iter(){
            match token{
                Token::Instruction { name, args } => {
                    match name.value.as_str(){
                        "org" => {
                            if args.len() != 1{
                                println!("{}:{}:{} expected addr", self.source_filename, name.row, name.col);
                                std::process::exit(1);
                            }

                            self.cursor = 0;

                            self.origin = get_value_from_number_token(self.source_filename, &args[0]);

                        }

                        _ => {
                            let instruction = match self.instructions.get(&name.value.as_str()){
                                Some(a) => a,
                                None => {
                                    println!("{}:{}:{} Unknown instruction {}", self.source_filename, name.row, name.col, name.value);
                                    std::process::exit(1);
                                }
                            }.as_slice();

                            let mut args = args.clone();

                            let mut bits_str = String::new();

                            
                            for part in instruction{
                                match part{
                                    InstructionPart::Const { val } => {
                                        bits_str+=val;
                                    },
                                    InstructionPart::Register { size } => {
                                        if args.len() == 0{
                                            println!("{}:{}:{} Expected Register", self.source_filename, name.row, name.col+name.value.len());
                                            std::process::exit(1);
                                        }
                                        let arg = args.remove(0);
                                        if !matches!(arg.ttype, LexemType::Register){
                                            println!("{}:{}:{} Expected Register got {:?}", self.source_filename, arg.row, arg.col, arg.ttype);
                                            std::process::exit(1);
                                        }
                                        
                                        let val = *REGISTERS_TO_VAL.get(arg.value.as_str()).unwrap();
                                        
                                        let val = format!("{:b}", val);
                                        
                                        bits_str+="0".repeat(*size - val.len()).as_str();
                                        
                                        bits_str += val.as_str();
                                    }
                                    
                                    InstructionPart::Imm { size } => {
                                        if args.len() == 0{
                                            println!("{}:{}:{} Expected Immediate", self.source_filename, name.row, name.col+name.value.len());
                                            std::process::exit(1);
                                        }
                                        let arg = args.remove(0);
                                        
                                        if arg.ttype == LexemType::Ident{
                                            bits_str += "F".repeat(*size).as_str();
                                            self.to_fix.push((arg.clone(), *size,fix_index));
                                            continue;
                                        }

                                        let val = get_value_from_number_token(self.source_filename, &arg);
                                        
                                        
                                        let val = format!("{:b}", val);
                                        
                                        if val.len() > *size{
                                            println!("{}:{}:{} Number is too big {}", self.source_filename, arg.row, arg.col, arg.value);
                                            std::process::exit(1);
                                        }

                                        bits_str+="0".repeat(*size - val.len()).as_str();
                                        
                                        bits_str += val.as_str();
                                    }

                                    InstructionPart::Extra { size } => {
                                        if args.len() == 0{
                                            bits_str += "0".repeat(*size).as_str();
                                            continue;
                                        }
                                        
                                        let arg = args.remove(0);
                                        
                                        let val = get_value_from_number_token(self.source_filename, &arg);
                                        
                                        
                                        let val = format!("{:b}", val);
                                        
                                        if val.len() > *size{
                                            println!("{}:{}:{} Number is too big {}", self.source_filename, arg.row, arg.col, arg.value);
                                            std::process::exit(1);
                                        }
                                        
                                        bits_str+="0".repeat(*size - val.len()).as_str();
                                        bits_str += val.as_str();
                                    }
                                    
                                    _ => todo!()
                                }
                            }

                            self.str_instructions.push(bits_str);
                            
                            self.cursor += 1;
                            fix_index += 1;
                        }
                    }

                },

                Token::Label { name } => {
                    self.fix_addr.insert(name.value.clone(), self.origin+self.cursor);
                }

            }

        }

        // now patches

        for (fix, size,  i) in self.to_fix.iter(){
            let i = *i;
            let size = *size;

            let addr = match self.fix_addr.get(&fix.value){
                Some(x) => *x,
                None => {
                    println!("{}:{}:{} Use of undeclared label {}", self.source_filename, fix.row, fix.col, fix.value);
                    std::process::exit(1);
                }
            };

            let addr_str = format!("{:b}",addr);

            if addr_str.len() > size {
                println!("{}:{}:{} Addr of {} is too big: {:x}", self.source_filename, fix.row, fix.col, fix.value, addr);
                std::process::exit(1);
            }

            let addr = "0".repeat(size-addr_str.len()) + addr_str.as_str();

            self.str_instructions[i] = self.str_instructions[i].replace("F".repeat(size).as_str(), &addr);

        }

        for instruction in self.str_instructions.iter(){
            let instruction = self.str_to_bytes(instruction);
            for byte in instruction{
                self.bytes.push(byte);
            }
        }
    }
}

