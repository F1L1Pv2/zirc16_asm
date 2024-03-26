use std::collections::HashMap;

use crate::{InstructionPart, Lexem, LexemType, Token, REGISTERS_TO_VAL};

#[derive(Debug, Clone)]
enum InstructionTypes {
    str{
        value: String,
    },
    pure_bytes{
        value: Vec<u8>
    }
}

#[derive(Debug)]
pub struct CodeGen<'a>{
    source_filename: &'a str,
    origin: usize,
    cursor: usize,
    tokens: &'a[Token],
    instruction_set: &'a HashMap<&'static str,Vec<InstructionPart>>,
    fix_addr: HashMap<String,usize>,
    instructions: Vec<InstructionTypes>,
    to_fix: Vec<(Lexem, usize, usize)>,
    pub bytes: Vec<u8>
}

fn get_value_from_number_token<'a>(filename: &'a str, lexem: &Lexem) -> usize{
    match lexem.ttype{
        LexemType::Number { radix } => {usize::from_str_radix(&lexem.value, radix as u32).unwrap()}
        _ => {
            println!("{}:{}:{} Expected number got {:?}", filename, lexem.row, lexem.col,lexem.ttype);
            std::process::exit(1);
        }
    }
}

impl CodeGen<'_>{
    pub fn new<'a>(source_filename: &'a str, tokens: &'a[Token], instruction_set: &'a HashMap<&'static str,Vec<InstructionPart>>) -> CodeGen<'a>{
        CodeGen{
            source_filename,
            origin: 0,
            cursor: 0,
            tokens,
            instruction_set,
            fix_addr: HashMap::new(),
            to_fix: Vec::new(),
            instructions: Vec::new(),
            bytes: Vec::new()
        }
    }

    pub fn str_to_bytes(self: &Self, str: &String) -> [u8; 2]{
        let ret: usize = usize::from_str_radix(str, 2).unwrap();
        (ret as u16).to_be_bytes()
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

                        "db" => {
                            if args.len() == 0{
                                println!("{}:{}:{} No data was provided", self.source_filename, name.row, name.col+name.value.len());
                                std::process::exit(1);
                            }
                            
                            todo!()
                        }

                        "dw" => {
                            if args.len() == 0{
                                println!("{}:{}:{} No data was provided", self.source_filename, name.row, name.col+name.value.len());
                                std::process::exit(1);
                            }

                            let mut bytes: Vec<u8> = Vec::new();
                            for arg in args{
                                match arg.ttype{
                                    LexemType::Ident => todo!(),
                                    LexemType::Number { radix } => {
                                        let b = ((usize::from_str_radix(&arg.value, radix as u32).unwrap() & 0xFFFF ) as u16).to_be_bytes();
                                        for b in b{
                                            bytes.push(b);
                                        }
                                    },
                                    LexemType::String => {
                                        for ch in arg.value.chars(){
                                            bytes.push(0);
                                            bytes.push(ch as u8);
                                        }
                                    },
                                    _ => {
                                        println!("{}:{}:{} Unexpected lexem {:?}", self.source_filename, arg.row, arg.col, arg.ttype);
                                        std::process::exit(1);
                                    }
                                }
                            }

                            self.cursor += bytes.len()/2;
                            self.instructions.push(InstructionTypes::pure_bytes { value: bytes });
                        }

                        "dd" => {
                            if args.len() == 0{
                                println!("{}:{}:{} No data was provided", self.source_filename, name.row, name.col+name.value.len());
                                std::process::exit(1);
                            }
    
                            let mut bytes: Vec<u8> = Vec::new();
                            for arg in args{
                                match arg.ttype{
                                    LexemType::Ident => todo!(),
                                    LexemType::Number { radix } => {
                                        let b = ((usize::from_str_radix(&arg.value, radix as u32).unwrap() & 0xFFFFFFFF ) as u32).to_be_bytes();
                                        for b in b{
                                            bytes.push(b);
                                        }
                                    },
                                    LexemType::String => {
                                        for ch in arg.value.chars(){
                                            bytes.push(0);
                                            bytes.push(0);
                                            bytes.push(0);
                                            bytes.push(ch as u8);
                                        }
                                    },
                                    _ => {
                                        println!("{}:{}:{} Unexpected lexem {:?}", self.source_filename, arg.row, arg.col, arg.ttype);
                                        std::process::exit(1);
                                    }
                                }
                            }

                            self.cursor += bytes.len()/2;
                            self.instructions.push(InstructionTypes::pure_bytes { value: bytes });
                        }

                        "dq" => {
                            if args.len() == 0{
                                println!("{}:{}:{} No data was provided", self.source_filename, name.row, name.col+name.value.len());
                                std::process::exit(1);
                            }

                            let mut bytes: Vec<u8> = Vec::new();
                            for arg in args{
                                match arg.ttype{
                                    LexemType::Ident => todo!(),
                                    LexemType::Number { radix } => {
                                        let b = ((usize::from_str_radix(&arg.value, radix as u32).unwrap() & 0xFFFFFFFFFFFFFFFF ) as u32).to_be_bytes();
                                        for b in b{
                                            bytes.push(b);
                                        }
                                    },
                                    LexemType::String => {
                                        for ch in arg.value.chars(){
                                            bytes.push(0);
                                            bytes.push(0);
                                            bytes.push(0);
                                            bytes.push(0);
                                            bytes.push(0);
                                            bytes.push(0);
                                            bytes.push(0);
                                            bytes.push(ch as u8);
                                        }
                                    },
                                    _ => {
                                        println!("{}:{}:{} Unexpected lexem {:?}", self.source_filename, arg.row, arg.col, arg.ttype);
                                        std::process::exit(1);
                                    }
                                }
                            }
                            
                            self.cursor += bytes.len()/2;
                            self.instructions.push(InstructionTypes::pure_bytes { value: bytes });

                        }

                        _ => {
                            let instruction = match self.instruction_set.get(&name.value.as_str()){
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

                            self.instructions.push(InstructionTypes::str { value: bits_str });
                            
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

            match self.instructions[i].clone(){
                InstructionTypes::str { value } => {
                    self.instructions[i] = InstructionTypes::str{ value: value.replace("F".repeat(size).as_str(), &addr)};
                }
                InstructionTypes::pure_bytes { .. } => {
                    println!("CodeGen Err expected Str got Pure Bytes");
                    std::process::exit(1);
                }
            }

            // self.instructions[i] = self.instructions[i].replace("F".repeat(size).as_str(), &addr);

        }

        for instruction in self.instructions.iter(){
            match instruction{
                InstructionTypes::str { value } => {
                    let instruction = self.str_to_bytes(value);
                    for byte in instruction{
                        self.bytes.push(byte);
                    }
                }
                InstructionTypes::pure_bytes { value } => {
                    for byte in value{
                        self.bytes.push(*byte);
                    }
                }
            }
        }
        // dbg!(&self.fix_addr);
        // dbg!(&self.to_fix);
        // dbg!(&self.instructions);
    }
}

