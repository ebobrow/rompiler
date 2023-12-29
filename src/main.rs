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
    let lines = Compiler::compile(e);

    let mut file = File::create("a.asm").unwrap();
    file.write_all(b"global main\n").unwrap();
    file.write_all(
        br#"section .text
"#,
    )
    .unwrap();

    // custom heap (there's gotta be a better way)
    file.write_all(
        br#"main:
push rbp
mov rbp, rsp
sub rbp, 8
sub rsp, 88
"#,
    )
    .unwrap();
    for line in lines {
        file.write_all(line.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }
    file.write_all(
        br#"add rsp, 88
pop rbp
ret
"#,
    )
    .unwrap();

    for stdlib in fs::read_dir("src/stdlib").unwrap() {
        file.write_all(
            format!("%include '{}'\n", stdlib.unwrap().path().to_str().unwrap()).as_bytes(),
        )
        .unwrap();
    }

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
