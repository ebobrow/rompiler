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
    let (consts, lines) = Compiler::new().compile(e);

    let mut file = File::create("a.asm").unwrap();
    file.write_all(b"global main\n").unwrap();
    file.write_all(
        br#"section .text
"#,
    )
    .unwrap();

    // custom heap (there's gotta be a better way)
    file.write_all(b"main:\n").unwrap();
    for line in lines {
        file.write_all(line.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }
    file.write_all(b"ret\n").unwrap();

    file.write_all(b"section .data\n").unwrap();
    for (name, val) in consts {
        file.write_all(format!("{name}: dd {val:?}\n").as_bytes())
            .unwrap();
    }

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
