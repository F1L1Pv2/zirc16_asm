use std::collections::HashMap;

use crate::{InstructionPart, Lexem, LexemType, Token, REGISTERS_TO_VAL};

#[derive(Debug)]
pub struct CodeGen<'a>{
    source_filename: &'a str,
    tokens: &'a[Token],
    instruction_set: &'a HashMap<&'static str,Vec<InstructionPart>>,
    pub bytes: Vec<u8>
}

pub fn get_value_from_number_token<'a>(filename: &'a str, lexem: &Lexem) -> usize{
    match lexem.ttype{
        LexemType::Number { radix } => {usize::from_str_radix(&lexem.value, radix as u32).unwrap()}
        _ => {
            println!("{}:{}:{} Expected number got {}", filename, lexem.row, lexem.col,lexem.ttype);
            std::process::exit(1);
        }
    }
}

impl CodeGen<'_>{
    pub fn new<'a>(source_filename: &'a str, tokens: &'a[Token], instruction_set: &'a HashMap<&'static str,Vec<InstructionPart>>) -> CodeGen<'a>{
        CodeGen{
            source_filename,
            tokens,
            instruction_set,
            bytes: Vec::new()
        }
    }

    pub fn str_to_bytes(self: &Self, str: &String) -> [u8; 2]{
        let ret: usize = usize::from_str_radix(str, 2).unwrap();
        (ret as u16).to_be_bytes()
    }

    pub fn gen(self: &mut Self){

        for token in self.tokens.iter(){
            match token{
                Token::Instruction { name, args } => {
                    match name.value.as_str(){
                        "org" => {
                            println!("Org: Error in parser");
                            std::process::exit(1);
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

                            for arg in args{
                                match arg.ttype{
                                    LexemType::Number { radix } => {
                                        let b = ((usize::from_str_radix(&arg.value, radix as u32).unwrap() & 0xFFFF ) as u16).to_be_bytes();
                                        for b in b{
                                            self.bytes.push(b);
                                        }
                                    },
                                    LexemType::String => {
                                        for ch in arg.value.chars(){
                                            self.bytes.push(0);
                                            self.bytes.push(ch as u8);
                                        }
                                    },
                                    _ => {
                                        println!("{}:{}:{} Unexpected lexem {}", self.source_filename, arg.row, arg.col, arg.ttype);
                                        std::process::exit(1);
                                    }
                                }
                            }

                        }

                        "dd" => {
                            if args.len() == 0{
                                println!("{}:{}:{} No data was provided", self.source_filename, name.row, name.col+name.value.len());
                                std::process::exit(1);
                            }
    
                            for arg in args{
                                match arg.ttype{
                                    LexemType::Number { radix } => {
                                        let b = ((usize::from_str_radix(&arg.value, radix as u32).unwrap() & 0xFFFFFFFF ) as u32).to_be_bytes();
                                        for b in b{
                                            self.bytes.push(b);
                                        }
                                    },
                                    LexemType::String => {
                                        for ch in arg.value.chars(){
                                            self.bytes.push(0);
                                            self.bytes.push(0);
                                            self.bytes.push(0);
                                            self.bytes.push(ch as u8);
                                        }
                                    },
                                    _ => {
                                        println!("{}:{}:{} Unexpected lexem {}", self.source_filename, arg.row, arg.col, arg.ttype);
                                        std::process::exit(1);
                                    }
                                }
                            }
                        }

                        "dq" => {
                            if args.len() == 0{
                                println!("{}:{}:{} No data was provided", self.source_filename, name.row, name.col+name.value.len());
                                std::process::exit(1);
                            }

                            for arg in args{
                                match arg.ttype{
                                    LexemType::Number { radix } => {
                                        let b = ((usize::from_str_radix(&arg.value, radix as u32).unwrap() & 0xFFFFFFFFFFFFFFFF ) as u32).to_be_bytes();
                                        for b in b{
                                            self.bytes.push(b);
                                        }
                                    },
                                    LexemType::String => {
                                        for ch in arg.value.chars(){
                                            self.bytes.push(0);
                                            self.bytes.push(0);
                                            self.bytes.push(0);
                                            self.bytes.push(0);
                                            self.bytes.push(0);
                                            self.bytes.push(0);
                                            self.bytes.push(0);
                                            self.bytes.push(ch as u8);
                                        }
                                    },
                                    _ => {
                                        println!("{}:{}:{} Unexpected lexem {}", self.source_filename, arg.row, arg.col, arg.ttype);
                                        std::process::exit(1);
                                    }
                                }
                            }

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
                                            println!("{}:{}:{} Expected Register got {}", self.source_filename, arg.row, arg.col, arg.ttype);
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
                                            println!("{}:{}:{} Error in parser", self.source_filename, arg.row, arg.col);
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

                            let bytes = self.str_to_bytes(&bits_str);
                            for byte in bytes{
                                self.bytes.push(byte);
                            }
                        }
                    }

                },

                Token::Label { name } => {
                    println!("{}:{}:{} Error in parser", self.source_filename, name.row, name.col);
                    std::process::exit(1);
                }

            }

        }

    }
}

