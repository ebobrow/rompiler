#[derive(Debug)]
pub enum Token {
    Expr(Expr),
    Const(String),
}

#[derive(Debug)]
pub struct Expr {
    pub op: String,
    pub params: Vec<Token>,
}

impl Expr {
    pub fn new(op: String, params: Vec<Token>) -> Self {
        Self { op, params }
    }
}

pub struct Parser {
    ptr: usize,
    data: String,
}

impl Parser {
    pub fn parse(data: String) -> Expr {
        let mut parser = Parser { ptr: 0, data };
        parser.parse_expr()
    }

    fn parse_expr(&mut self) -> Expr {
        assert_eq!(self.advance(), Some('('));
        self.skip_whitespace();
        let mut op = String::new();
        while self.peek_is(|c| !c.is_whitespace() && c != ')') {
            op.push(self.advance().unwrap());
        }
        self.skip_whitespace();
        let mut params = Vec::new();
        while self.peek_is(|c| c != ')') {
            params.push(self.parse_param());
            self.skip_whitespace();
        }
        let e = Expr::new(op, params);
        assert_eq!(self.advance(), Some(')'));
        e
    }

    fn parse_param(&mut self) -> Token {
        if self.peek_is(|c| c == '(') {
            Token::Expr(self.parse_expr())
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
