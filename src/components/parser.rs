use crate::{expect_lexem_type, Lexem, LexemType};

#[derive(Debug)]
pub enum Token{
    Label{
        name: Lexem
    },
    Instruction{
        name: Lexem,
        args: Vec<Lexem>
    }
}

pub struct Parser<'a>{
    source_filename: &'a str,
    cursor: usize,
    lexems: &'a[Lexem],
    pub tokens: Vec<Token>
}

impl Parser<'_>{
    pub fn new<'a>(source_filename: &'a str, lexems: &'a[Lexem]) -> Parser<'a>{
        Parser{
            source_filename,
            cursor: 0,
            lexems,
            tokens: Vec::new()
        }
    }


    fn peek_lexem(self: &Self) -> Option<Lexem>{

        if self.cursor >= self.lexems.len(){
            return None;
        }

        Some(self.lexems[self.cursor].clone())
    }

    fn chop_lexem(self: &mut Self) -> Lexem{
        let lexem = self.peek_lexem().unwrap();
        self.cursor += 1;
        return lexem;
    }

    fn parse_lexem_label(self: &mut Self) -> bool{

        let initial_cursor = self.cursor;

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

        if !expect_lexem_type(&self.peek_lexem().unwrap(), arg_types) {
            // return None;
            let lexem = self.peek_lexem().unwrap();
            println!("{}:{}:{} Expected arg got {:?}", self.source_filename, lexem.row, lexem.col, lexem.ttype);
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
                println!("{}:{}:{} Expected arg got {:?}", self.source_filename, arg.row, arg.col, arg.ttype);
                std::process::exit(1);
            }

            args.push(arg);

        }

        return Some(args);
    }

    fn parse_lexem_instruction(self: &mut Self) -> bool{

        let initial_cursor = self.cursor;

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
        

        let lexem = self.peek_lexem().unwrap();
        println!("{}:{}:{} got unexpected token {}", self.source_filename, lexem.row, lexem.col, lexem.value);
        // dbg!(&self.tokens);
        std::process::exit(1);
    }

    pub fn parse(self: &mut Self){
        self.cursor = 0;
        while self.cursor < self.lexems.len(){
            self.parse_token()
        }
    }
}
