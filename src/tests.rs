use std::{env, fs::File, io::Write};

use compiler_lib::{Compiler, Parser};

fn main() {
    let args: Vec<_> = env::args().collect();
    let rkt = &args[1];
    let e = Parser::parse(rkt.to_string());
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
        file.write(b"\n").unwrap();
    }
    file.write_all(b"ret").unwrap();
}
