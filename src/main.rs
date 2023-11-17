use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

mod misc;
mod parser;
mod tokenizer;
mod symbols;

use misc::Parameters;
use parser::Parser;
use tokenizer::Tokenizer;

fn main() {
    let args = env::args();
    let params = Parameters::from(args);

    //TODO: better error message
    let mut file = File::open(params.file_path).expect("Failed to open file!");

    let mut file_content = String::new();

    file.read_to_string(&mut file_content)
        .expect("Failed to read file into string");

    let tokenizer = Tokenizer::new(&file_content);
    let mut parser = Parser::new();

    parser.parse(tokenizer);
}
