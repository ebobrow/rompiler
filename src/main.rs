use std::{
    env,
    fs::{self, File},
    io::{self, Write},
    process::Command,
};

use compiler_lib::{Compiler, Lexer, Parser};

fn main() {
    let args: Vec<_> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).unwrap();
    let e = Parser::parse(Lexer::lex(contents));
    let mut file = File::create("a.asm").unwrap();
    Compiler::default().compile_to_file(e, &mut file);

    Command::new("nasm")
        .args(["-f", "elf64", "a.asm", "-o", "a.o"])
        .output()
        .unwrap();

    let output = Command::new("gcc")
        .args(["-no-pie", "a.o"])
        .output()
        .unwrap();
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}
