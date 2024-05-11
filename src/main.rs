/*
 made by:
  ___ _ _    _ ___      ___ 
 | __/ | |  / | _ \__ _|_  )
 | _|| | |__| |  _/\ V // / 
 |_| |_|____|_|_|   \_//___|
*/

use std::io::Write;
use std::{fs::File, io::Read};

mod components;
use components::lexer::*;
use components::parser::*;
use components::codegen::*;
use components::common::*;
use components::instruction_lexer::*;
use components::args::*;

/*

    Kaktusiku mój drogi edytuj common.rs spoczko :)

*/


fn main() {

    
    // let pseudo_instructions = PseudoInstructions::initialize();
    
    // dbg!(&pseudo_instructions);

    let mut instruction_lexer: InstructionsLexer = InstructionsLexer::new();

    instruction_lexer.lex_instructions();

    let args: Args = Args::parse();

    let mut file = File::open(&args.input).unwrap();

    let mut content = String::new();

    file.read_to_string(&mut content).unwrap();

    let mut lexer: Lexer = Lexer::new();

    lexer.lex(&args.input.as_os_str().to_string_lossy(), &content);

    // dbg!(&lexer.lexems);
    
    let mut parser: Parser = Parser::new();
    
    parser.parse(&lexer.lexems);
    
    // dbg!(&parser.tokens);
    
    let mut codegen: CodeGen = CodeGen::new(&parser.tokens, &instruction_lexer.instructions);

    codegen.gen();

    let mut file = File::create(&args.output.as_ref().unwrap()).unwrap();

    let _ = file.write(&codegen.bytes);

    println!("Assembled file: {} ({} bytes)", &args.output.unwrap().display(), codegen.bytes.len());
}
