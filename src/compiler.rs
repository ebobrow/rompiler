use std::collections::HashSet;

use crate::parser::{Expr, Token};

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
enum Reg {
    RAX,
    RBX,
    RCX,
    RDX,
    // RBP,
    RSI,
    RDI,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

// type Reg = usize;

// fn reg_to_string(r: Reg) -> String {
//     match r {
//         1 => "RAX",
//         2 => "RBX",
//         3 => "RCX",
//         4 => "RDX",
//         5 => "RBP",
//         6 => "RSI",

//     }
// }
// struct Reg {
//     n: usize,
//     bits: usize,
// }

// impl ToString for Reg {
//     fn to_string(&self) -> String {
//         match self.n {
//             1 =>
//             _ => match self.bits {
//                 64 => format!("r{}", self.n),
//                 32 => format!("r{}D", self.n),
//                 16 => format!("r{}W", self.n),
//                 8 => format!("r{}B", self.n),
//                 _ => unreachable!(),
//             },
//         }
//     }
// }

pub struct Compiler {
    lines: Vec<String>,
    preserve: HashSet<Reg>,

    /// Convenience variable, should be a constant
    all_regs: HashSet<Reg>,
}

impl Compiler {
    fn new() -> Self {
        Self {
            lines: Vec::new(),
            preserve: HashSet::new(),
            all_regs: HashSet::from([
                Reg::RAX,
                Reg::RBX,
                Reg::RCX,
                Reg::RDX,
                // Reg::RBP,
                Reg::RSI,
                Reg::RDI,
                Reg::R8,
                Reg::R9,
                Reg::R10,
                Reg::R11,
                Reg::R12,
                Reg::R13,
                Reg::R14,
                Reg::R15,
            ]),
        }
    }

    pub fn compile(e: Expr) -> Vec<String> {
        let mut compiler = Compiler::new();
        compiler.compile_tok(&Token::Expr(e), Some(Reg::RAX));
        compiler.lines
    }

    fn l(&mut self, line: impl ToString) {
        self.lines.push(line.to_string());
    }

    fn compile_tok(&mut self, t: &Token, target: Option<Reg>) -> Reg {
        let out = match t {
            Token::Expr(e) => match &e.op[..] {
                "+" => {
                    // TODO: can add an arbitrary amount of numbers
                    assert_eq!(e.params.len(), 2);
                    self.binop("add", &e.params[0], &e.params[1], target)
                }
                "-" => {
                    assert_eq!(e.params.len(), 2);
                    self.binop("sub", &e.params[0], &e.params[1], target)
                }
                "*" => {
                    assert_eq!(e.params.len(), 2);
                    self.binop_in_reg("imul", Reg::RAX, Reg::RAX, &e.params[0], &e.params[1])
                }
                "/" => {
                    assert_eq!(e.params.len(), 2);
                    self.preserve.insert(Reg::RDX); // remainder is stored in rdx
                    self.binop_in_reg("idiv", Reg::RAX, Reg::RAX, &e.params[0], &e.params[1])
                }
                "mod" => {
                    assert_eq!(e.params.len(), 2);
                    self.binop_in_reg("idiv", Reg::RAX, Reg::RDX, &e.params[0], &e.params[1])
                }
                _ => unimplemented!(),
            },
            Token::Const(c) => self.compile_constant(c, target),
        };
        if let Some(target) = target {
            if out != target {
                self.l(format!("mov {target:?}, {out:?}"));
            }
            target
        } else {
            out
        }
    }

    fn compile_constant(&mut self, c: &String, target: Option<Reg>) -> Reg {
        let val = match &c[..] {
            "#t" => "1",
            "#f" => "0",
            _ => c,
        };
        let out = target.unwrap_or_else(|| self.next_reg());
        self.l(format!("mov {out:?}, {val}"));
        out
    }

    fn binop(&mut self, op: &str, p1: &Token, p2: &Token, target: Option<Reg>) -> Reg {
        let out = self.compile_tok(p1, target);
        self.preserve.insert(out);

        let reg2 = self.compile_tok(p2, None);

        self.l(format!("{op} {out:?}, {reg2:?}"));
        self.preserve.remove(&out);
        out
    }

    fn binop_in_reg(&mut self, op: &str, reg1: Reg, target: Reg, p1: &Token, p2: &Token) -> Reg {
        let save_reg = self.preserve.contains(&reg1);
        if save_reg {
            self.l(format!("push {reg1:?}"));
        }

        self.compile_tok(p1, Some(reg1));
        self.preserve.insert(reg1);
        self.preserve.insert(target); // not strictly necessary but it simplifies things

        let reg2 = self.compile_tok(p2, None);

        self.l(format!("{op} {reg2:?}"));
        self.preserve.remove(&reg1);

        if save_reg {
            if reg1 == target {
                self.l(format!("mov {reg2:?}, {reg1:?}"));
                self.l(format!("pop {reg1:?}"));
                return reg2;
            }
            self.l(format!("pop {reg1:?}"));
        }
        target
    }

    fn next_reg(&self) -> Reg {
        *self.all_regs.difference(&self.preserve).next().unwrap()
    }
}
