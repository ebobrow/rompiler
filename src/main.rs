use std::{
    env,
    fs::{self, File},
    io::Write,
};

use compiler_lib::{Compiler, Parser};

fn main() {
    let args: Vec<_> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).unwrap();
    let e = Parser::parse(contents);
    let lines = Compiler::compile(e);

    let mut file = File::create("a.asm").unwrap();
    file.write_all(
        br#"global main
section .text
main:"#,
    )
    .unwrap();
    for line in lines {
        file.write_all(line.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }
    file.write_all(b"ret").unwrap();
}
