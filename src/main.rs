use std::io::Write;
use std::path::Path;
use std::{fs::File, io::Read};

mod components;
use components::lexer::*;
use components::parser::*;
use components::codegen::*;
use components::common::*;
use components::instruction_lexer::*;

/*

    Kaktusiku mój drogi edytuj common.rs spoczko :)

*/


fn main() {

    let mut instruction_lexer: InstructionsLexer = InstructionsLexer::new();

    instruction_lexer.lex_instructions();

    let mut args = std::env::args();

    let filename = args.next().unwrap();

    let source_filename = match args.next(){
        Some(n) => {n},
        None => {
            println!("{}: Source Filename wasn't provided", filename);
            std::process::exit(1);
        }
    };

    let path = Path::new(&source_filename);

    let mut file = File::open(path).unwrap();

    let mut content = String::new();

    file.read_to_string(&mut content).unwrap();

    let mut lexer: Lexer = Lexer::new(&source_filename, &content);

    lexer.lex();

    // dbg!(&lexer.lexems);

    let mut parser: Parser = Parser::new(&source_filename, &lexer.lexems);

    parser.parse();

    // dbg!(&parser.tokens);

    let mut codegen: CodeGen = CodeGen::new(&source_filename,&parser.tokens, &instruction_lexer.instructions);

    codegen.gen();

    // println!("{:?}", path.extension());

    let output_str = match path.extension(){
        Some(a) => path.to_str().unwrap().replace(a.to_str().unwrap(),"zirc16"),
        None => path.to_str().unwrap().to_string() + ".zirc16"
    };

    let mut file = File::create(&output_str).unwrap();

    let _ =file.write(&codegen.bytes);

    println!("Assembled file: {} ({} bytes)", output_str, codegen.bytes.len());
    

}
