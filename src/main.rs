use std::env;
use std::fs::File;
use std::io::prelude::*;

mod misc;
mod parser;
mod symbols;
mod tokenizer;
mod declarations;
mod assembly;

use assembly::Assembler;
use misc::Parameters;
use parser::Parser;
use symbols::CodeScope;
use tokenizer::Tokenizer;

fn main() {
    let args = env::args();
    let params = Parameters::from(args);

    let global_scope = CodeScope::global();

    //TODO: better error message
    let mut file = File::open(params.file_path).expect("Failed to open file!");

    let mut file_content = String::new();

    file.read_to_string(&mut file_content).expect("Failed to read file into string");

    let tokenizer  = Tokenizer::new(&file_content);
    let mut parser = Parser::new(global_scope);

    let mut assembly = Assembler::new("out");
    let ast = parser.parse(tokenizer);

    assembly.from_ast(&ast);
    assembly.build();
}
