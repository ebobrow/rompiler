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

fn assert_params(e: &Expr, n: usize) {
    assert_eq!(e.params.len(), n);
}

pub struct Compiler {
    lines: Vec<String>,
    preserve: HashSet<Reg>,
    bindings: HashMap<String, Reg>,
    consts: Vec<(String, f64)>,

    /// Number of pushes to stack. If even, pointer will not be aligned after making a call and a
    /// push must be made
    rsp_parity: usize,

    /// Convenience variable, should be a constant
    usable_regs: HashSet<Reg>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            preserve: HashSet::new(),
            bindings: HashMap::new(),
            consts: Vec::new(),
            rsp_parity: 0,
            usable_regs: HashSet::from([
                // Reg::RAX,
                Reg::RBX,
                Reg::RCX,
                Reg::RDX,
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

    pub fn with_consts(consts: Vec<(String, f64)>) -> Self {
        Self {
            lines: Vec::new(),
            preserve: HashSet::new(),
            bindings: HashMap::new(),
            consts,
            rsp_parity: 0,
            usable_regs: HashSet::from([
                // Reg::RAX,
                Reg::RBX,
                Reg::RCX,
                Reg::RDX,
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

    pub fn compile(mut self, t: Node) -> (Vec<(String, f64)>, Vec<String>) {
        self.compile_tok(&t, Some(Reg::RAX));
        assert_eq!(self.preserve.len(), 0);
        assert_eq!(self.bindings.len(), 0);
        (self.consts, self.lines)
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
                    self.arith("madd", &e.params[0], &e.params[1])
                }
                "-" => {
                    assert_params(e, 2);
                    self.arith("msub", &e.params[0], &e.params[1])
                }
                "*" => {
                    assert_params(e, 2);
                    self.arith("mmul", &e.params[0], &e.params[1])
                }
                "/" => {
                    assert_params(e, 2);
                    self.arith("mdiv", &e.params[0], &e.params[1])
                }
                "mod" => {
                    assert_params(e, 2);
                    self.arith("mmod", &e.params[0], &e.params[1])
                }
                "=" => {
                    assert_params(e, 2);
                    self.arith("eq", &e.params[0], &e.params[1])
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
                        let reg = *self.bindings.get(s).unwrap();
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
                let name = if let Some((name, _)) = self.consts.iter().find(|(_, val)| val == f) {
                    name.clone()
                } else {
                    let name = self.next_var_name();
                    self.consts.push((name.clone(), *f));
                    name
                };
                self.l(format!("movss XMM0, [{name}]"));
                self.call_function("newfloat")
            }
            Node::Integer(i) => {
                self.l(format!("mov RDI, {i}"));
                self.call_function("newint")
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

    fn arith(&mut self, op: &str, p1: &Node, p2: &Node) -> Reg {
        // RAX must contain the dividend
        let save_rax = self.preserve.contains(&Reg::RAX);
        if save_rax {
            self.l("push rax");
            self.rsp_parity += 1;
        }
        self.preserve.insert(Reg::RAX);

        let r1 = self.compile_tok(p1, None);
        self.preserve.insert(r1);
        let r2 = self.compile_tok(p2, None);
        self.l(format!("mov rdi, {r1:?}"));
        self.l(format!("mov rsi, {r2:?}"));
        let out = self.call_function(op);

        if save_rax {
            self.l("pop rax");
            self.rsp_parity -= 1;
        } else {
            self.preserve.remove(&Reg::RAX);
        }
        self.preserve.remove(&r1);
        out
    }

    fn call_function(&mut self, name: &str) -> Reg {
        if self.preserve.contains(&Reg::RAX) {
            self.l("push rax");
            self.rsp_parity += 1;
        }
        if self.rsp_parity % 2 == 0 {
            self.l("sub rsp, 8");
        }

        self.l(format!("call {name}"));

        if self.rsp_parity % 2 == 0 {
            self.l("add rsp, 8");
        }
        if self.preserve.contains(&Reg::RAX) {
            let out = self.next_reg();
            self.l(format!("mov {out:?}, rax"));
            self.l("pop rax");
            self.rsp_parity -= 1;
            return out;
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

    fn next_var_name(&self) -> String {
        if let Some((name, _)) = self.consts.last() {
            if name.chars().last().unwrap() == 'z' {
                format!("{name}a")
            } else {
                name.chars()
                    .enumerate()
                    .map(|(i, c)| {
                        if i == name.len() - 1 {
                            ((c as u8) + 1).into()
                        } else {
                            c
                        }
                    })
                    .collect()
            }
        } else {
            String::from("a")
        }
    }
}
