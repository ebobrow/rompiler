use std::{
    env,
    fs::{self, File},
    io::Write,
};

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

    let mut file = File::create("a.asm").unwrap();
    file.write_all(
        br#"global main
extern printf
section .data
format: db "%d", 10, 0
section .text
main:"#,
    )
    .unwrap();
    for line in lines {
        file.write_all(line.as_bytes()).unwrap();
        file.write(b"\n").unwrap();
    }
    file.write_all(
        br#"mov rdi, format
xor eax, eax
push rax
call printf
pop rax
mov rax, 0
ret"#,
    )
    .unwrap();
}
