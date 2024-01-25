use crate::lexer::Token;

#[derive(Debug, PartialEq)]
pub enum Node {
    Expr(Expr),
    String(String),
    Float(f64),
    Integer(i64),
    LetExpr(Box<LetExpr>),
    LambdaExpr(Box<LambdaExpr>),
}

#[derive(Debug, PartialEq)]
pub struct Expr {
    pub op: String,
    pub params: Vec<Node>,
}

impl Expr {
    pub fn new(op: String, params: Vec<Node>) -> Self {
        Self { op, params }
    }
}

#[derive(Debug, PartialEq)]
pub struct LetExpr {
    pub bindings: Vec<(String, Node)>,
    pub body: Node,
}

#[derive(Debug, PartialEq)]
pub struct LambdaExpr {
    pub params: Vec<String>,
    pub body: Node,
}

pub struct Parser {
    ptr: usize,
    data: Vec<Token>,
}

impl Parser {
    pub fn parse(data: Vec<Token>) -> Node {
        let mut parser = Parser { ptr: 0, data };
        parser.parse_expr()
    }

    fn parse_expr(&mut self) -> Node {
        self.consume_open();
        let op = self.consume_ident().clone().inner_ident();
        if op == "let*" {
            Node::LetExpr(Box::new(self.parse_let_expr()))
        } else if op == "lambda" {
            Node::LambdaExpr(Box::new(self.parse_lambda_expr()))
        } else {
            let mut params = Vec::new();
            while self.peek_is(|c| c != &Token::RightParen) {
                params.push(self.parse_param());
            }
            let e = Expr::new(op, params);
            self.consume_close();
            Node::Expr(e)
        }
    }

    fn parse_let_expr(&mut self) -> LetExpr {
        self.consume_open();
        let mut bindings = Vec::new();
        while self.peek_is(|c| c != &Token::RightParen && c != &Token::RightBracket) {
            self.consume_open();
            let name = self.consume_ident().clone().inner_ident();
            let e = self.parse_param();
            bindings.push((name, e));
            self.consume_close();
        }
        self.consume_close();
        let body = self.parse_param();
        self.consume_close();
        LetExpr { bindings, body }
    }

    fn parse_lambda_expr(&mut self) -> LambdaExpr {
        self.consume_open();
        let mut params = Vec::new();
        while self.peek_is(|c| c != &Token::RightParen && c != &Token::RightBracket) {
            let name = self.consume_ident().clone().inner_ident();
            params.push(name);
        }
        self.consume_close();
        let body = self.parse_param();
        self.consume_close();
        LambdaExpr { params, body }
    }

    fn parse_param(&mut self) -> Node {
        if self.peek_is(|c| c == &Token::LeftParen) {
            self.parse_expr()
        } else {
            match self.advance() {
                Token::Integer(i) => Node::Integer(*i),
                Token::Float(i) => Node::Float(*i),
                Token::Identifier(i) => Node::String(i.to_string()),
                _ => panic!(),
            }
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.data.get(self.ptr)
    }

    fn peek_is(&self, f: impl Fn(&Token) -> bool) -> bool {
        matches!(self.peek(), Some(c) if f(c))
    }

    fn advance(&mut self) -> &Token {
        self.ptr += 1;
        &self.data[self.ptr - 1]
    }

    fn consume_open(&mut self) {
        assert!(matches!(
            self.advance(),
            Token::LeftParen | Token::LeftBracket
        ));
    }

    fn consume_close(&mut self) {
        assert!(matches!(
            self.advance(),
            Token::RightParen | Token::RightBracket
        ));
    }

    fn consume_ident(&mut self) -> &Token {
        let next = self.advance();
        assert!(matches!(next, Token::Identifier(_)));
        next
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;

    use super::*;

    #[test]
    fn let_expr() {
        let rkt = String::from("(let* ((x 5) (y 4.0)) (+ x y))");
        assert_eq!(
            Parser::parse(Lexer::lex(rkt)),
            Node::LetExpr(Box::new(LetExpr {
                bindings: vec![
                    ("x".into(), Node::Integer(5)),
                    ("y".into(), Node::Float(4.0))
                ],
                body: Node::Expr(Expr {
                    op: "+".into(),
                    params: vec![Node::String("x".into()), Node::String("y".into())]
                })
            }))
        );
    }

    #[test]
    fn lambda() {
        let rkt = String::from("(lambda (x y) (+ x y))");
        assert_eq!(
            Parser::parse(Lexer::lex(rkt)),
            Node::LambdaExpr(Box::new(LambdaExpr {
                params: vec!["x".into(), "y".into()],
                body: Node::Expr(Expr {
                    op: "+".into(),
                    params: vec![Node::String("x".into()), Node::String("y".into())]
                })
            }))
        );
    }
}
