use std::{fs::File, io::Write, process::Command};

use compiler_lib::{Compiler, Parser};

fn run_test(rkt: impl ToString, expected: i32) {
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

    Command::new("nasm")
        .args(["-f", "elf64", "a.asm"])
        .status()
        .unwrap();
    Command::new("gcc")
        .args(["-no-pie", "a.o"])
        .status()
        .unwrap();
    let out = Command::new("./a.out").status().unwrap();
    assert_eq!(
        out.code(),
        Some(expected),
        "{}: expected {}, got {}",
        rkt.to_string(),
        expected,
        out.code().unwrap()
    );
}

#[test]
fn arithmetic() {
    run_test("(+ 1 (* 2 3))", 7);
    run_test("(+ 40 2)", 42);
    run_test("(- 2 1)", 1);
    run_test("(* 2 21)", 42);
    run_test("(/ 5 2)", 2);
    run_test("(+ 1 (* 2 (- 3 4)))", -1);
}
