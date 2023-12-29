#[derive(Debug, PartialEq)]
pub enum Token {
    Expr(Expr),
    Const(String),
    LetExpr(LetExpr),
}

#[derive(Debug, PartialEq)]
pub struct Expr {
    pub op: String,
    pub params: Vec<Token>,
}

impl Expr {
    pub fn new(op: String, params: Vec<Token>) -> Self {
        Self { op, params }
    }
}

#[derive(Debug, PartialEq)]
pub struct LetExpr {
    pub bindings: Vec<(String, Token)>,
    pub body: Box<Token>,
}

pub struct Parser {
    ptr: usize,
    data: String,
}

impl Parser {
    pub fn parse(data: String) -> Token {
        let mut parser = Parser { ptr: 0, data };
        parser.parse_expr()
    }

    fn parse_expr(&mut self) -> Token {
        assert_eq!(self.advance(), Some('('));
        self.skip_whitespace();
        let mut op = String::new();
        while self.peek_is(|c| !c.is_whitespace() && c != ')') {
            op.push(self.advance().unwrap());
        }
        self.skip_whitespace();
        if op == "let*" {
            Token::LetExpr(self.parse_let_expr())
        } else {
            let mut params = Vec::new();
            while self.peek_is(|c| c != ')') {
                params.push(self.parse_param());
                self.skip_whitespace();
            }
            let e = Expr::new(op, params);
            assert_eq!(self.advance(), Some(')'));
            Token::Expr(e)
        }
    }

    fn parse_let_expr(&mut self) -> LetExpr {
        assert_eq!(self.advance(), Some('('));
        self.skip_whitespace();
        let mut bindings = Vec::new();
        while self.peek_is(|c| c != ')') {
            assert_eq!(self.advance(), Some('('));
            let mut name = String::new();
            while self.peek_is(|c| !c.is_whitespace()) {
                name.push(self.advance().unwrap());
            }
            self.skip_whitespace();
            let e = self.parse_param();
            bindings.push((name, e));
            assert_eq!(self.advance(), Some(')'));
            self.skip_whitespace();
        }
        assert_eq!(self.advance(), Some(')'));
        self.skip_whitespace();
        let body = self.parse_param();
        assert_eq!(self.advance(), Some(')'));
        LetExpr {
            bindings,
            body: Box::new(body),
        }
    }

    fn parse_param(&mut self) -> Token {
        if self.peek_is(|c| c == '(') {
            self.parse_expr()
        } else {
            let mut param = String::new();
            while self.peek_is(|c| !c.is_whitespace() && c != ')') {
                param.push(self.advance().unwrap());
            }
            Token::Const(param)
        }
    }

    fn peek(&self) -> Option<char> {
        self.data.chars().nth(self.ptr)
    }

    fn peek_is(&self, f: impl Fn(char) -> bool) -> bool {
        matches!(self.peek(), Some(c) if f(c))
    }

    fn advance(&mut self) -> Option<char> {
        self.ptr += 1;
        self.data.chars().nth(self.ptr - 1)
    }

    fn skip_whitespace(&mut self) {
        while self.peek_is(char::is_whitespace) {
            self.advance();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn let_expr() {
        let rkt = String::from("(let* ((x 5) (y 4)) (+ x y))");
        assert_eq!(
            Parser::parse(rkt),
            Token::LetExpr(LetExpr {
                bindings: vec![
                    ("x".into(), Token::Const("5".into())),
                    ("y".into(), Token::Const("4".into()))
                ],
                body: Box::new(Token::Expr(Expr {
                    op: "+".into(),
                    params: vec![Token::Const("x".into()), Token::Const("y".into())]
                }))
            })
        );
    }
}
