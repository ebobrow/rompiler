#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Identifier(String),
    Integer(i64),
    Float(f64),
    EOF,
}

impl Token {
    pub fn inner_ident(self) -> String {
        if let Token::Identifier(s) = self {
            s
        } else {
            panic!("expected identifier, got {self:?}");
        }
    }
}

pub struct Lexer {
    src: String,
    ptr: usize,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn lex(src: String) -> Vec<Token> {
        let mut lexer = Lexer {
            src,
            ptr: 0,
            tokens: Vec::new(),
        };

        while lexer.ptr < lexer.src.len() {
            let token = lexer.scan_token();
            lexer.tokens.push(token);
        }

        lexer.tokens
    }

    fn scan_token(&mut self) -> Token {
        if self.ptr == self.src.len() {
            return Token::EOF;
        }
        match self.advance() {
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '[' => Token::LeftBracket,
            ']' => Token::RightBracket,
            ' ' | '\n' => self.scan_token(),
            c => {
                if c.is_numeric() {
                    self.number()
                } else {
                    self.identifier()
                }
            }
        }
    }

    fn peek(&self) -> char {
        self.src.chars().nth(self.ptr).unwrap()
    }

    fn advance(&mut self) -> char {
        self.ptr += 1;
        self.src.chars().nth(self.ptr - 1).unwrap()
    }

    fn number(&mut self) -> Token {
        let start = self.ptr - 1;
        while self.peek().is_numeric() || self.peek() == '.' {
            self.advance();
        }
        // self.advance();
        let substr = &self.src[start..self.ptr];
        if substr.contains('.') {
            Token::Float(substr.parse().unwrap())
        } else {
            Token::Integer(substr.parse().unwrap())
        }
    }

    fn identifier(&mut self) -> Token {
        let start = self.ptr - 1;
        while !self.peek().is_whitespace() && self.peek() != ')' {
            self.advance();
        }
        Token::Identifier(self.src[start..self.ptr].to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer() {
        let toks = Lexer::lex(String::from("(let* [(x 1) (y 2.0)] (+ x y))"));
        assert_eq!(
            toks,
            vec![
                Token::LeftParen,
                Token::Identifier("let*".into()),
                Token::LeftBracket,
                Token::LeftParen,
                Token::Identifier("x".into()),
                Token::Integer(1),
                Token::RightParen,
                Token::LeftParen,
                Token::Identifier("y".into()),
                Token::Float(2.0),
                Token::RightParen,
                Token::RightBracket,
                Token::LeftParen,
                Token::Identifier("+".into()),
                Token::Identifier("x".into()),
                Token::Identifier("y".into()),
                Token::RightParen,
                Token::RightParen,
            ]
        );
    }
}
