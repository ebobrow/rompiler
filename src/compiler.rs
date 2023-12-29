use std::collections::{HashMap, HashSet};

use crate::parser::{Expr, LetExpr, Node};

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

fn assert_params(e: &Expr, n: usize) {
    assert_eq!(e.params.len(), n);
}

pub struct Compiler {
    lines: Vec<String>,
    preserve: HashSet<Reg>,
    bindings: HashMap<String, Reg>,

    /// Number of pushes to stack. If even, pointer will not be aligned after making a call and a
    /// push must be made
    rsp_parity: usize,

    /// Convenience variable, should be a constant
    usable_regs: HashSet<Reg>,
}

impl Compiler {
    fn new() -> Self {
        Self {
            lines: Vec::new(),
            preserve: HashSet::new(),
            bindings: HashMap::new(),
            rsp_parity: 0,
            usable_regs: HashSet::from([
                // Reg::RAX,
                Reg::RBX,
                Reg::RCX,
                // Reg::RDX,
                // Reg::RBP,
                // Reg::RSI,
                // Reg::RDI,
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

    pub fn compile(t: Node) -> Vec<String> {
        let mut compiler = Compiler::new();
        compiler.compile_tok(&t, Some(Reg::RAX));
        assert_eq!(compiler.preserve.len(), 0);
        assert_eq!(compiler.bindings.len(), 0);
        compiler.lines
    }

    fn l(&mut self, line: impl ToString) {
        self.lines.push(line.to_string());
    }

    fn compile_tok(&mut self, t: &Node, target: Option<Reg>) -> Reg {
        let out = match t {
            Node::Expr(e) => match &e.op[..] {
                // Arithmetic operations
                "+" => {
                    // TODO: can add an arbitrary amount of numbers
                    assert_params(e, 2);
                    self.binop("add", &e.params[0], &e.params[1], target)
                }
                "-" => {
                    assert_params(e, 2);
                    self.binop("sub", &e.params[0], &e.params[1], target)
                }
                "*" => {
                    assert_params(e, 2);
                    self.binop_in_reg("imul", Reg::RAX, Reg::RAX, &e.params[0], &e.params[1])
                }
                "/" => {
                    assert_params(e, 2);
                    // self.preserve.insert(Reg::RDX); // remainder is stored in rdx
                    // self.binop_in_reg("idiv", Reg::RAX, Reg::RAX, &e.params[0], &e.params[1])
                    self.div(Reg::RAX, &e.params[0], &e.params[1])
                }
                "mod" => {
                    assert_params(e, 2);
                    // self.binop_in_reg("idiv", Reg::RAX, Reg::RDX, &e.params[0], &e.params[1])
                    self.div(Reg::RDX, &e.params[0], &e.params[1])
                }

                // List operations
                "empty" => {
                    assert_params(e, 0);
                    self.call_function("empty")
                }
                f @ ("first" | "rest") => {
                    assert_params(e, 1);
                    self.call_one_param(f, &e.params[0])
                }
                "empty?" => {
                    assert_params(e, 1);
                    self.call_one_param("isempty", &e.params[0])
                }
                f @ ("cons" | "append") => {
                    assert_params(e, 2);
                    self.call_two_param(f, &e.params[0], &e.params[1])
                }
                "list" => self.call_on_stack("list", &e.params[..]),

                _ => unimplemented!(),
            },
            Node::LetExpr(LetExpr { bindings, body }) => self.compile_let_expr(bindings, body),
            Node::String(_) | Node::Float(_) | Node::Integer(_) => self.compile_constant(t, target),
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

    fn compile_constant(&mut self, c: &Node, target: Option<Reg>) -> Reg {
        match c {
            Node::String(s) => {
                let val = match &s[..] {
                    "#t" => 1,
                    "#f" => 0,
                    _ => {
                        let reg = self.bindings.get(s).unwrap().clone();
                        if let Some(target) = target {
                            self.l(format!("mov {target:?}, {reg:?}"));
                        }
                        return reg;
                    }
                };
                let out = target.unwrap_or_else(|| self.next_reg());
                self.l(format!("mov {out:?}, {val}"));
                out
            }
            Node::Float(f) => {
                todo!()
            }
            Node::Integer(i) => {
                let out = target.unwrap_or_else(|| self.next_reg());
                self.l(format!("mov {out:?}, {i}"));
                out
            }
            _ => unreachable!(),
        }
    }

    fn compile_let_expr(&mut self, bindings: &[(String, Node)], body: &Node) -> Reg {
        for (name, val) in bindings {
            let reg = self.next_reg();
            self.compile_tok(val, Some(reg));
            self.bindings.insert(name.to_string(), reg);
            self.preserve.insert(reg);
        }
        let out = self.compile_tok(body, None);
        for (name, _) in bindings {
            let reg = self.bindings.remove(name).unwrap();
            self.preserve.remove(&reg);
        }
        out
    }

    fn binop(&mut self, op: &str, p1: &Node, p2: &Node, target: Option<Reg>) -> Reg {
        let out = self.compile_tok(p1, target);
        self.preserve.insert(out);

        let reg2 = self.compile_tok(p2, None);

        self.l(format!("{op} {out:?}, {reg2:?}"));
        self.preserve.remove(&out);
        out
    }

    fn binop_in_reg(&mut self, op: &str, reg1: Reg, target: Reg, p1: &Node, p2: &Node) -> Reg {
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

    fn div(&mut self, out: Reg, p1: &Node, p2: &Node) -> Reg {
        // RAX must contain the dividend
        let save_rax = self.preserve.contains(&Reg::RAX);
        if save_rax {
            self.l("push rax");
        }
        self.preserve.insert(Reg::RAX);
        // RDX must be empty
        let save_rdx = self.preserve.contains(&Reg::RDX);
        if save_rdx {
            self.l("push rdx");
        }
        self.l("xor rdx, rdx");
        self.preserve.insert(Reg::RDX);

        self.compile_tok(p1, Some(Reg::RAX));
        let reg2 = self.compile_tok(p2, None);
        self.l(format!("idiv {reg2:?}"));

        let mut actual_out = out;
        if save_rdx {
            if out == Reg::RDX {
                actual_out = self.next_reg();
                self.l(format!("move {actual_out:?}, rdx"));
            }
            self.l("pop rdx");
        }
        if save_rax {
            if out == Reg::RAX {
                actual_out = self.next_reg();
                self.l(format!("move {actual_out:?}, rax"));
            }
            self.l("pop rax");
        }
        self.preserve.remove(&Reg::RAX);
        self.preserve.remove(&Reg::RDX);
        actual_out
    }

    fn call_function(&mut self, name: &str) -> Reg {
        if self.rsp_parity % 2 == 0 {
            self.l("sub rsp, 8");
        }

        self.l(format!("call {name}"));

        if self.rsp_parity % 2 == 0 {
            self.l("add rsp, 8");
        }
        Reg::RAX
    }

    fn call_one_param(&mut self, name: &str, p1: &Node) -> Reg {
        self.compile_tok(p1, Some(Reg::RDI));

        self.call_function(name)
    }

    fn call_two_param(&mut self, name: &str, p1: &Node, p2: &Node) -> Reg {
        self.compile_tok(p1, Some(Reg::RDI));
        self.preserve.insert(Reg::RDI);
        self.compile_tok(p2, Some(Reg::RSI));
        self.preserve.remove(&Reg::RDI);

        self.call_function(name)
    }

    fn call_on_stack(&mut self, name: &str, params: &[Node]) -> Reg {
        let stack_misaligned = (self.rsp_parity + params.len()) % 2 == 0;
        if stack_misaligned {
            self.l("sub rsp, 8");
        }

        for param in params.iter().rev() {
            let reg = self.compile_tok(param, None);
            self.l(format!("push {reg:?}"));
            self.rsp_parity += 1;
        }

        self.l(format!("mov rdi, {}", params.len()));
        self.l(format!("call {name}"));

        for _ in params {
            let reg = self.next_reg();
            self.l(format!("pop {reg:?}"));
            self.rsp_parity -= 1;
        }

        if stack_misaligned {
            self.l("add rsp, 8");
        }
        Reg::RAX
    }

    fn next_reg(&self) -> Reg {
        *self.usable_regs.difference(&self.preserve).next().unwrap()
    }
}
