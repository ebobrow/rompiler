use std::{env, fs};

use compiler::Compiler;
use parser::Parser;

mod compiler;
mod parser;

fn main() {
    let args: Vec<_> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).unwrap();
    let e = Parser::parse(contents);
    let lines = Compiler::compile(e);
    println!("{:#?}", lines);
}
