/*
 made by:
  ___ _ _    _ ___      ___ 
 | __/ | |  / | _ \__ _|_  )
 | _|| | |__| |  _/\ V // / 
 |_| |_|____|_|_|   \_//___|
*/

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

    
    // let pseudo_instructions = PseudoInstructions::initialize();
    
    // dbg!(&pseudo_instructions);

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

    let mut lexer: Lexer = Lexer::new();

    lexer.lex(&source_filename, &content);

    // dbg!(&lexer.lexems);
    
    let mut parser: Parser = Parser::new();
    
    parser.parse(&lexer.lexems);
    
    // dbg!(&parser.tokens);
    
    let mut codegen: CodeGen = CodeGen::new(&parser.tokens, &instruction_lexer.instructions);

    codegen.gen();

    let output_str = match path.extension(){
        Some(a) => path.to_str().unwrap().replace(a.to_str().unwrap(),"zirc16"),
        None => path.to_str().unwrap().to_string() + ".zirc16"
    };

    let mut file = File::create(&output_str).unwrap();

    let _ =file.write(&codegen.bytes);

    println!("Assembled file: {} ({} bytes)", output_str, codegen.bytes.len());
    

}
