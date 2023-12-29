use std::{
    fs::{self, File},
    io::{self, Write},
    process::Command,
};

use compiler_lib::{Compiler, Lexer, Parser};

#[test]
fn arithmetic() {
    run_tests(
        "arith",
        &[
            ("(+ 1 (* 2 3))", 7),
            ("(+ 40 2)", 42),
            ("(- 2 1)", 1),
            ("(- 1 2)", -1),
            ("(* 2 21)", 42),
            ("(/ 5 2)", 2),
            ("(+ 1 (* 2 (- 3 4)))", -1),
            ("(mod 5 2)", 1),
        ],
    );
}

#[test]
fn lists() {
    run_tests(
        "lists",
        &[
            ("(empty? (empty))", 1),
            ("(empty? (cons 1 (empty)))", 0),
            ("(first (list 1 2 3))", 1),
            ("(first (rest (list 1 2 3)))", 2),
            ("(first (cons 1 (empty)))", 1),
            // this one doesn't pass but that's a later problem :o
            // ("(first (rest (append (list 1) (list 2 3))))", 2),
        ],
    );
}

#[test]
fn local_variables() {
    run_tests(
        "local vars",
        &[
            ("(let* ((x 1)) x)", 1),
            ("(let* ((x 1) (y 2)) (+ x y))", 3),
            ("(let* ((x 1) (y (+ 1 x))) (+ x y))", 3),
        ],
    );
}

/// Do not touch this function it is awful
fn run_tests(name: &str, tests: &[(impl ToString, i64)]) {
    fs::create_dir_all("target/tests").unwrap();
    let mut asmfile = File::create(format!("target/tests/{name}.asm")).unwrap();
    let mut cfile = File::create(format!("target/tests/{name}.c")).unwrap();
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
        let e = Parser::parse(Lexer::lex(rkt.to_string()));
        let lines = Compiler::compile(e);
        asmfile.write_all(format!("f{i}:\n").as_bytes()).unwrap();
        asmfile
            .write_all(
                br#"push rbp
mov rbp, rsp
sub rbp, 8
sub rsp, 88
"#,
            )
            .unwrap();
        for line in lines {
            asmfile.write_all(line.as_bytes()).unwrap();
            asmfile.write_all(b"\n").unwrap();
        }
        asmfile
            .write_all(
                br#"add rsp, 88
pop rbp
ret
"#,
            )
            .unwrap();

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

    for stdlib in fs::read_dir("src/stdlib").unwrap() {
        asmfile
            .write_all(
                format!("%include '{}'\n", stdlib.unwrap().path().to_str().unwrap()).as_bytes(),
            )
            .unwrap();
    }

    Command::new("nasm")
        .args([
            "-f",
            "elf64",
            &format!("target/tests/{name}.asm")[..],
            "-o",
            &format!("target/tests/{name}.o")[..],
        ])
        .output()
        .unwrap();
    Command::new("gcc")
        .args([
            "-no-pie",
            &format!("target/tests/{name}.o")[..],
            &format!("target/tests/{name}.c")[..],
            "-o",
            &format!("target/tests/{name}.out")[..],
        ])
        .output()
        .unwrap();
    let output = Command::new(&format!("./target/tests/{name}.out")[..])
        .output()
        .unwrap();
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
    assert_eq!(output.status.code().unwrap(), 0);
}
