use std::{
    fs::{self, File},
    io::{self, Write},
    process::Command,
};

use compiler_lib::{Compiler, Parser};

/// Do not touch this function it is awful
fn run_tests(tests: &[(impl ToString, i64)]) {
    fs::create_dir_all("target/tests").unwrap();
    let mut asmfile = File::create("target/tests/a.asm").unwrap();
    let mut cfile = File::create("target/tests/test.c").unwrap();
    cfile
        .write_all(b"#include <stdio.h>\n#include <inttypes.h>\n")
        .unwrap();

    for i in 0..(tests.len()) {
        asmfile
            .write_all(format!("global f{i}\n").as_bytes())
            .unwrap();
        cfile
            .write_all(format!("int64_t f{i}();").as_bytes())
            .unwrap();
    }

    asmfile.write_all(b"section .text\n").unwrap();
    cfile
        .write_all(b"int main() {int all_pass = 0;int64_t out;")
        .unwrap();

    for (i, (rkt, expected)) in tests.iter().enumerate() {
        let e = Parser::parse(rkt.to_string());
        let lines = Compiler::compile(e);
        asmfile.write_all(format!("f{i}:\n").as_bytes()).unwrap();
        for line in lines {
            asmfile.write_all(line.as_bytes()).unwrap();
            asmfile.write_all(b"\n").unwrap();
        }
        asmfile.write_all(b"ret\n").unwrap();

        cfile.write_all(format!("out=f{i}();").as_bytes()).unwrap();
        cfile
            .write_all(
                format!(
                    "if (out != {expected}) {{printf(\"{rkt}: expected {expected}, got %d\\n\", out);all_pass=1;}}",
                    rkt = rkt.to_string(),
                    expected = expected.to_string()
                )
                .as_bytes(),
            )
            .unwrap();
    }
    cfile.write_all(b"return all_pass;}").unwrap();

    Command::new("nasm")
        .args([
            "-f",
            "elf64",
            "target/tests/a.asm",
            "-o",
            "target/tests/a.o",
        ])
        .output()
        .unwrap();
    Command::new("gcc")
        .args([
            "-no-pie",
            "target/tests/a.o",
            "target/tests/test.c",
            "-o",
            "target/tests/a.out",
        ])
        .output()
        .unwrap();
    let output = Command::new("./target/tests/a.out").output().unwrap();
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
    assert_eq!(output.status.code().unwrap(), 0);
}

#[test]
fn arithmetic() {
    run_tests(&[
        ("(+ 1 (* 2 3))", 7),
        ("(+ 40 2)", 42),
        ("(- 2 1)", 1),
        ("(- 1 2)", -1),
        ("(* 2 21)", 42),
        ("(/ 5 2)", 2),
        ("(+ 1 (* 2 (- 3 4)))", -1),
    ]);
}
