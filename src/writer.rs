use std::{
    fs::{self, File},
    io::Write,
};

use crate::Compiler;

pub trait Writer {
    fn to_file(&self, file: &mut File);
}

impl Writer for Compiler {
    fn to_file(&self, file: &mut File) {
        file.write_all(
            br#"global main
section .text
"#,
        )
        .unwrap();

        for line in self.fns.iter() {
            file.write_all(line.as_bytes()).unwrap();
            file.write_all(b"\n").unwrap();
        }

        file.write_all(
            br#"main:
call currip
mov rbx, rax
"#,
        )
        .unwrap();
        for line in &self.lines {
            file.write_all(line.as_bytes()).unwrap();
            file.write_all(b"\n").unwrap();
        }
        file.write_all(
            br#"ret
currip:
    mov eax, [rsp]
    ret
section .data
"#,
        )
        .unwrap();
        for (name, val) in &self.consts {
            if !val.is_nan() {
                file.write_all(format!("{name}: dd {val:?}\n").as_bytes())
                    .unwrap();
            }
        }

        for stdlib in fs::read_dir("src/stdlib").unwrap() {
            file.write_all(
                format!("%include '{}'\n", stdlib.unwrap().path().to_str().unwrap()).as_bytes(),
            )
            .unwrap();
        }
    }
}
