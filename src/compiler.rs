use std::collections::HashSet;

use crate::parser::{Expr, Token};

type Reg = usize;

#[derive(Default)]
pub struct Compiler {
    lines: Vec<String>,
    preserve: HashSet<Reg>,
}

impl Compiler {
    pub fn compile(e: Expr) -> Vec<String> {
        let mut compiler = Compiler::default();
        compiler.compile_tok(&Token::Expr(e), 1);
        compiler.lines
    }

    fn l(&mut self, line: impl ToString) {
        self.lines.push(line.to_string());
    }

    fn compile_tok(&mut self, t: &Token, out: Reg) {
        match t {
            Token::Expr(e) => match &e.op[..] {
                "+" | "-" | "*" | "quotient" | "%" => {
                    assert_eq!(e.params.len(), 2);
                    self.compile_arithmetic(&e.op[..], &e.params[0], &e.params[1], out)
                }
                _ => unimplemented!(),
            },
            Token::Const(c) => self.compile_constant(c, out),
        }
    }

    fn compile_constant(&mut self, c: &String, out: Reg) {
        let val = match &c[..] {
            "#t" => "1",
            "#f" => "0",
            _ => &c,
        };
        self.l(format!("setn r{out} {val}"));
    }

    fn compile_arithmetic(&mut self, op: &str, p1: &Token, p2: &Token, out: Reg) {
        let reg1 = self.next_reg();
        self.compile_tok(p1, reg1);
        self.preserve.insert(reg1);

        let reg2 = self.next_reg();
        self.compile_tok(p2, reg2);
        self.preserve.insert(reg2);

        let instruction = match op {
            "+" => "add",
            "-" => "sub",
            "*" => "mul",
            "quotient" => "div",
            "%" => "mod",
            _ => unreachable!(),
        };
        self.l(format!("{instruction} r{out} r{reg1} r{reg2}"));

        self.preserve.remove(&reg1);
        self.preserve.remove(&reg2);
    }

    fn next_reg(&self) -> Reg {
        *HashSet::from_iter(1..=12)
            .difference(&self.preserve)
            .next()
            .unwrap()
    }
}
