use std::collections::HashMap;

use crate::{expect_lexem_type, get_value_from_number_token, Lexem, LexemType};

use super::pseudo_instructions::PseudoInstructions;

#[derive(Debug, Clone)]
pub enum Token{
    Label{
        name: Lexem
    },
    Instruction{
        name: Lexem,
        args: Vec<Lexem>
    }
}

impl std::fmt::Display for Token{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        match self{
            Token::Instruction { .. } => {write!(f, "Instruction")},
            Token::Label { .. } => {write!(f, "Label")}
        }
    }
}

pub struct Parser{
    source_filename: String,
    cursor: usize,
    lexems: Vec<Lexem>,
    pub tokens: Vec<Token>
}

impl Parser{
    pub fn new() -> Parser{
        Parser{
            source_filename: String::new(),
            cursor: 0,
            lexems: Vec::new(),
            tokens: Vec::new()
        }
    }


    fn peek_lexem(self: &Self) -> Option<Lexem>{

        if self.cursor >= self.lexems.len(){
            return None;
        }

        Some(self.lexems[self.cursor].clone())
    }

    fn chop_newline(self: &mut Self){
        while self.peek_lexem().unwrap().ttype == LexemType::NewLine{
            self.chop_lexem();
            if self.cursor >= self.lexems.len(){
                return;
            }
        }
    }

    fn chop_lexem(self: &mut Self) -> Lexem{
        let lexem = self.peek_lexem().unwrap();
        self.cursor += 1;
        return lexem;
    }

    fn parse_lexem_label(self: &mut Self) -> bool{
        
        let initial_cursor = self.cursor;
        
        self.chop_newline();

        if self.cursor >= self.lexems.len(){
            self.cursor = initial_cursor;
            return false;
        }


        if self.peek_lexem().unwrap().ttype != LexemType::Ident{
            self.cursor = initial_cursor;
            return false;
        }

        let label_name = self.chop_lexem();

        if self.cursor >= self.lexems.len(){
            self.cursor = initial_cursor;
            return false;
        }

        if self.peek_lexem().unwrap().value != ":"{
            self.cursor = initial_cursor;
            return false;
        }

        self.chop_lexem();

        self.tokens.push(Token::Label { name: label_name });

        return true;
    }

    fn parse_args(self: &mut Self) -> Option<Vec<Lexem>>{

        let mut args: Vec<Lexem> = Vec::new();

        let arg_types = &[LexemType::Number{radix: 0} ,LexemType::Ident, LexemType::Register];

        if self.cursor >= self.lexems.len(){
            return  Some(Vec::new());
        }

        if self.peek_lexem().unwrap().ttype == LexemType::NewLine{
            return Some(Vec::new());
        }

        if !expect_lexem_type(&self.peek_lexem().unwrap(), arg_types) {
            let lexem = self.peek_lexem().unwrap();
            println!("{}:{}:{} Expected arg got {}", self.source_filename, lexem.row, lexem.col, lexem.ttype);
            std::process::exit(1);
        }

        args.push(self.chop_lexem());

        if self.cursor >= self.lexems.len(){
            return Some(args);
        }

        while self.cursor < self.lexems.len() && self.peek_lexem().unwrap().value == ","{
            let x = self.chop_lexem();

            if self.cursor >= self.lexems.len(){
                println!("{}:{}:{} Expected arg got end of file", self.source_filename, x.row, x.col+1);
                std::process::exit(1);
            }

            let arg = self.chop_lexem();

            if !expect_lexem_type(&arg, arg_types){
                println!("{}:{}:{} Expected arg got {}", self.source_filename, arg.row, arg.col, arg.ttype);
                std::process::exit(1);
            }

            args.push(arg);

            if self.cursor < self.lexems.len() && self.peek_lexem().unwrap().ttype == LexemType::NewLine{
                break;
            }

        }

        return Some(args);
    }

    fn parse_lexem_instruction(self: &mut Self) -> bool{

        
        let initial_cursor = self.cursor;
        
        self.chop_newline();

        if self.cursor >= self.lexems.len(){
            self.cursor = initial_cursor;
            return false;
        }
        
        if self.peek_lexem().unwrap().ttype != LexemType::Ident{
            self.cursor = initial_cursor;
            return false;
        }

        let name = self.chop_lexem();

        let args = match self.parse_args(){
            Some(a) => a,
            None => {
                self.cursor = initial_cursor;
                return false;
            }
        };

        self.tokens.push(Token::Instruction { name, args });


        return true;
    }

    fn parse_token(self: &mut Self){
        if self.parse_lexem_label(){return}
        
        if self.parse_lexem_instruction(){return}
        
        self.chop_newline();

        if self.cursor >= self.lexems.len(){
            return;
        }

        let lexem = self.peek_lexem().unwrap();
        println!("{}:{}:{} got unexpected token {}", self.source_filename, lexem.row, lexem.col, lexem.value);
        // dbg!(&self.tokens);
        std::process::exit(1);
    }

    pub fn first_stage_parse<'a>(self: &mut Self, source_filename: &'a str, lexems: &Vec<Lexem>){
        self.source_filename = source_filename.to_string();
        self.lexems = lexems.clone();
        self.cursor = 0;

        self.tokens.clear();
        
        while self.cursor < self.lexems.len(){
            self.parse_token()
        }
    }

    pub fn parse<'a>(self: &mut Self, source_filename: &'a str, lexems: &Vec<Lexem>){
        
        self.first_stage_parse(source_filename, lexems);

        let pseudo_instructions = PseudoInstructions::initialize();

        let mut after_pseudo: Vec<Token> = Vec::new();

        for token in self.tokens.iter_mut(){
            match token{
                Token::Instruction { name, args } =>{
                    if pseudo_instructions.keys().collect::<Vec<&String>>().contains(&&name.value){
                        // dbg!(args);
                        let mut arg_hashmap: HashMap<String, Lexem> = HashMap::new();

                        let pseudo = match pseudo_instructions.get(name.value.as_str()){
                            Some(a) => a,
                            None => {
                                println!("{}:{}:{} Pasrser pseudo_instructions: Impossible Error", self.source_filename, name.row, name.col);
                                std::process::exit(1);
                            }
                        }.clone();

                        if args.len() != pseudo.0.len(){
                            println!("{}:{}:{} Expects {} ammount of args got {}", self.source_filename, name.row, name.col, pseudo.0.len(), args.len());
                        }

                        for (i, arg) in pseudo.0.iter().enumerate(){
                            arg_hashmap.insert(arg.clone(), args[i].clone());
                        }

                        let pseudo_name = name;

                        for token in pseudo.1{
                            match token{
                                Token::Instruction { name, args } => {
                                    let mut new_args: Vec<Lexem> = Vec::new();
                                    for arg in args{
                                        let arg_name = arg.value.clone();
                                        let new_arg = match arg_hashmap.get(&arg_name){
                                            Some(a) => a.clone(),
                                            None => Lexem::new(arg_name, arg.ttype, pseudo_name.row, pseudo_name.col)
                                        };
                                        new_args.push(new_arg);
                                    }
                                    after_pseudo.push(Token::Instruction { name, args: new_args });
                                }
                                Token::Label { name } => {
                                    println!("{}:{}:{} Currently labels are not possible inside pseudo instruction: {}", "PSEUDO_INSTRUCTION: ".to_string()+pseudo_name.value.as_str(), pseudo_name.row, pseudo_name.col, name.value);
                                }
                            }
                        }


                    }else{
                        after_pseudo.push(Token::Instruction {  name: name.clone(), args: args.clone() });
                    }
                },
                Token::Label { name } => {
                    after_pseudo.push(Token::Label { name: name.clone() });
                }
            }
        }
        self.tokens = after_pseudo;

        let mut origin: usize = 0;
        self.cursor = 0;

        let mut cleaned_tokens: Vec<Token> = Vec::new();

        let mut labels: HashMap<String, usize> = HashMap::new();

        for token in self.tokens.iter(){
            match token{
                Token::Instruction { name, args } => {

                    if name.value.to_lowercase() == "org"{
                        if args.len() != 1{
                            println!("{}:{}:{} you need to provide addr", self.source_filename, name.row, name.col);
                            std::process::exit(1);
                        }

                        let arg = args[0].clone();
                        if !matches!(arg.ttype, LexemType::Number { .. }){
                            println!("{}:{}:{} Expected number got {}", self.source_filename, arg.row, arg.col, arg.ttype);
                            std::process::exit(1);
                        }

                        origin = get_value_from_number_token(self.source_filename.as_str(), &arg);
                        self.cursor = 0;
                        continue;
                    }

                    let name = name.clone();
                    let args = args.clone();

                    if name.value.to_lowercase() == "dw"{
                        
                        let mut to_add = 0;

                        for arg in args.iter(){
                            match arg.ttype{
                                LexemType::Ident => to_add += 1,
                                LexemType::String => to_add += arg.value.len()*2,
                                LexemType::Number { .. } => to_add += 1,
                                _ => {
                                    println!("{}:{}:{} Unexpected token {}", self.source_filename, arg.row, arg.col, arg.ttype);
                                }
                            }
                        }

                        cleaned_tokens.push(Token::Instruction { name, args });
                        self.cursor += to_add;
                        continue;
                    }

                    if name.value.to_lowercase() == "dd"{
                        
                        let mut to_add = 0;

                        for arg in args.iter(){
                            match arg.ttype{
                                LexemType::Ident => to_add += 2,
                                LexemType::String => to_add += arg.value.len()*4,
                                LexemType::Number { .. } => to_add += 2,
                                _ => {
                                    println!("{}:{}:{} Unexpected token {}", self.source_filename, arg.row, arg.col, arg.ttype);
                                }
                            }
                        }

                        cleaned_tokens.push(Token::Instruction { name, args });
                        self.cursor += to_add;
                        continue;
                    }

                    if name.value.to_lowercase() == "dq"{
                        
                        let mut to_add = 0;

                        for arg in args.iter(){
                            match arg.ttype{
                                LexemType::Ident => to_add += 4,
                                LexemType::String => to_add += arg.value.len()*8,
                                LexemType::Number { .. } => to_add += 4,
                                _ => {
                                    println!("{}:{}:{} Unexpected token {}", self.source_filename, arg.row, arg.col, arg.ttype);
                                }
                            }
                        }

                        cleaned_tokens.push(Token::Instruction { name, args });
                        self.cursor += to_add;
                        continue;
                    }

                    cleaned_tokens.push(Token::Instruction { name, args });
                    self.cursor += 1;

                }
                Token::Label { name } => {
                    labels.insert(name.value.clone(), origin+self.cursor);
                }
            }
        }

        for arg in cleaned_tokens.iter_mut(){
            match arg{
                Token::Instruction { name: _, args } =>{
                    for arg in args{
                        if arg.ttype == LexemType::Ident{
                            let fix_addr = match labels.get(&arg.value){
                                Some(x) => x,
                                None => {
                                    println!("{}:{}:{} use of undeclared label {}", self.source_filename, arg.row, arg.col, arg.value);
                                    std::process::exit(1);
                                }
                            };
                             
                            *arg = Lexem::new(format!("{}",fix_addr),LexemType::Number { radix: 10 },arg.row,arg.col);

                        }
                    }
                }
                Token::Label { .. } => {
                    println!("Internal error labels shouldve been removed in this stage");
                    std::process::exit(1);
                }
            }
        }

        self.tokens = cleaned_tokens;

    }
}
