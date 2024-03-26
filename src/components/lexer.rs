use super::common::REGISTERS;


pub const SINGLE_LEXEMS: &[char] = &[',',':'];

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LexemType{
    Ident,
    Register,
    Single,
    Number{
        radix: usize
    },
    String,
    NewLine
}

#[derive(Debug, Clone)]
pub struct Lexem{
    pub value: String,
    pub ttype: LexemType,
    pub row: usize,
    pub col: usize
}

impl Lexem{
    fn new(value: String, ttype: LexemType, row: usize, col: usize) -> Lexem{
        Lexem { value, ttype, row, col}
    }
}

pub struct Lexer<'a>{
    content: &'a str,
    source_filename: &'a str,
    cursor: usize,
    row: usize,
    col: usize,
    pub lexems: Vec<Lexem>,
}

pub fn expect_lexem_type(lexem: &Lexem, expected: &[LexemType]) -> bool{
    
    for _other in expected{
        if matches!(lexem.ttype, _other){
            return true;
        }
    }

    false
}

impl Lexer<'_>{
    pub fn new<'a>(source_filename: &'a str, content: &'a str) -> Lexer<'a>{
        Lexer{
            content,
            source_filename,
            cursor: 0,
            row: 1,
            col: 1,
            lexems: Vec::new()
        }
    }

    fn peek(self: &Self) -> Option<char> {
        return self.content.chars().nth(self.cursor)
    }

    fn chop(self: &mut Self) -> char{
        let ch = self.peek().unwrap();
        self.cursor += 1;
        self.col += 1;
        if ch == '\n'{
            self.row += 1;
            self.col = 1;
        }
        return ch;
    }

    fn seek_whitespace(self: &mut Self){
        while self.cursor < self.content.len() && self.peek().unwrap().is_whitespace() && self.peek().unwrap() != '\n'{
            self.chop();
        }
    }

    fn chop_single(self: &mut Self) -> bool{
        if self.peek().unwrap() == '\n'{
            let row = self.row;
            let col = self.col;
            let ch = self.chop();
            self.lexems.push(Lexem::new(ch.to_string(), LexemType::NewLine, row, col));
            return true;
        }
        if SINGLE_LEXEMS.contains(&self.peek().unwrap()) {
            let row = self.row;
            let col = self.col;
            let ch = self.chop();
            self.lexems.push(Lexem::new(ch.to_string(), LexemType::Single,row, col));
            return true;
        }
        return false;
    }

    fn chop_word(self: &mut Self) -> bool{
        let mut lexem: String = String::new();

        let row = self.row;
        let col = self.col;

        while self.cursor < self.content.len() && self.peek().unwrap().is_alphanumeric(){

            lexem += self.chop().to_string().as_str();
        }

        if lexem.is_empty(){
            return false;
        }

        
        if lexem.starts_with("0x"){
            for (i, ch) in lexem.chars().skip(2).enumerate(){
                if !ch.is_ascii_hexdigit(){
                    println!("{}:{}:{} Expected hexlit got {}", self.source_filename, row, col+i+2, ch);
                    std::process::exit(1);
                }
                
            }
            self.lexems.push(Lexem::new(lexem.chars().skip(2).collect(), LexemType::Number { radix: 16 }, row,col));
            return true;
        }
        
        if lexem.starts_with("0b"){
            for (i, ch) in lexem.chars().skip(2).enumerate(){
                if ch != '0' && ch != '1' {
                    println!("{}:{}:{} Expected binlit got {}", self.source_filename, row, col+i+2, ch);
                    std::process::exit(1);
                }
                
            }
            self.lexems.push(Lexem::new(lexem.chars().skip(2).collect(), LexemType::Number { radix: 2 }, row,col));
            return true;
        }
        
        if lexem.chars().nth(0).unwrap().is_numeric(){
            for (i, ch) in lexem.chars().enumerate(){
                if !ch.is_numeric(){
                    println!("{}:{}:{} Expected number got {}", self.source_filename, row, col+i, ch);
                    std::process::exit(1);
                }

            }
            self.lexems.push(Lexem::new(lexem, LexemType::Number { radix: 10 }, row,col));
            return true;
        }


        if REGISTERS.contains(&lexem.to_lowercase().as_str()){
            self.lexems.push(Lexem::new(lexem.to_lowercase(), LexemType::Register, row,col));
            return true;
        }

        self.lexems.push(Lexem::new(lexem, LexemType::Ident, row,col));

        return true;
    }

    fn chop_string(self: &mut Self) -> bool{
        let mut lexem: String = String::new();

        let row = self.row;
        let col = self.col;

        let initial_cursor = self.cursor;

        if self.cursor >= self.content.len(){
            return false;
        }

        if self.peek().unwrap() != '\"' && self.peek().unwrap() != '\''{
            self.cursor = initial_cursor;
            return false;
        }

        self.chop();

        let mut value = String::new();

        while self.peek().unwrap() != '\"' && self.peek().unwrap() != '\''{
            if self.cursor >= self.content.len(){
                println!("{}:{}:{} Expected \" got end of file", self.source_filename, self.row, self.col);
                std::process::exit(1);
            }

            if self.peek().unwrap() == '\\'{
                self.chop();
                if self.cursor >= self.content.len(){
                    println!("{}:{}:{} Expected something got end of file", self.source_filename, self.row, self.col);
                    std::process::exit(1);
                }
                match self.chop(){
                    'n' => value += "\n",
                    '0' => value += "\0",
                    '\\' => value += "\\",
                    '\"' => value += "\"",
                    '\'' => value += "\'",
                     a  => {
                        println!("{}:{}:{} Unexpected character {}", self.source_filename, self.row, self.col, a);
                     }
                };

                continue;

            }

            value += self.chop().to_string().as_str();
        }

        self.chop();

        self.lexems.push(Lexem { value, ttype: LexemType::String, row, col });

        return true;
    }

    fn chop_lexem(self: &mut Self){

        self.seek_whitespace();

        if self.cursor >= self.content.len() {return}

        if self.chop_single() {return}

        if self.chop_string() {return}

        if self.chop_word() {return}

        
        println!("{}:{}:{} unexpected character: \"{}\" at {}", self.source_filename, self.row, self.col, self.peek().unwrap(), self.cursor);
        std::process::exit(1);

    }

    pub fn lex<'a>(self: &mut Self){
        self.cursor = 0;
        while self.cursor < self.content.len(){
            self.chop_lexem();
        }
    }
}