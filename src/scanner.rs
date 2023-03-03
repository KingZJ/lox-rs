use crate::error::LoxError;
use crate::token::{Object, Token};
use crate::token_type::TokenType;

// type Result<Vec<Token>> = std::result::Result<Vec<Token>, LoxError>;
pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    current: usize,
    start: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            current: 0,
            start: 0,
            line: 1,
        }
    }
    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, LoxError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token::eof(self.line));
        Ok(&self.tokens)
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) -> Result<(), LoxError> {
        let c = self.advance();
        match c {
            Some(item) => match item {
                ' ' | '\r' | '\t' => (),
                '\n' => self.line += 1,
                '"' => self.string()?,
                // '0'..='9' => self.number(),
                '(' => self.add_token(TokenType::LeftParen),
                ')' => self.add_token(TokenType::RightParen),
                '{' => self.add_token(TokenType::LeftBrace),
                '}' => self.add_token(TokenType::RightBrace),
                ',' => self.add_token(TokenType::Comma),
                '.' => self.add_token(TokenType::Dot),
                '-' => self.add_token(TokenType::Minus),
                '+' => self.add_token(TokenType::Plus),
                ';' => self.add_token(TokenType::SemiColon),
                '*' => self.add_token(TokenType::Star),
                '!' => {
                    let tk_type = if self.is_match('=') {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    };
                    self.add_token(tk_type);
                }
                '/' => {
                    if self.is_match('/') {
                        // comment

                        // loop {
                        //     println!("{:?}", self.peek());
                        //     match self.peek() {
                        //         Some(c) if c != '\n' => {
                        //             self.advance();
                        //             // continue;
                        //         },
                        //         _ => break,
                        //     }
                        // }
                        while let Some(c) = self.peek() {
                            if c != '\n' {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    } else {
                        self.add_token(TokenType::Slash);
                    }
                }
                c @ _ => {
                    if c.is_ascii_digit() {
                        self.number();
                    } else if c.is_ascii_alphabetic() || c == '_' {
                        self.identifier();
                    } else {
                        // unreachable!("unmatched token type");
                        return Err(LoxError::error(
                            self.line,
                            "unmatched token type".to_string(),
                        ));
                    }
                }
            },
            None => (),
        }

        Ok(())
    }

    fn peek(&self) -> Option<char> {
        // Option<&char> -> Option<char>
        self.source.get(self.current).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.source.get(self.current + 1).copied()
    }

    fn string(&mut self) -> Result<(), LoxError> {
        while let Some(c) = self.peek() {
            if c == '"' {
                break;
            } else if c == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            return Err(LoxError::error(self.line, "Unterminated string".to_owned()));
        }

        self.advance(); // close "

        // Token { tk_type: String, lexeme: "\"hello\"", literal: Some(Str("\"hello\"")), line: 1 }
        // TODO 转义
        let literal: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect(); // 把引号去掉
        self.add_token_object(TokenType::String, Some(Object::Str(literal)));

        Ok(())
    }

    fn number(&mut self) {
        self.check_number();

        if Some('.') == self.peek() && Self::is_digit(self.peek_next()) {
            self.advance();
            self.check_number();
        }

        let literal: String = self.source[self.start..self.current].iter().collect();
        let literal = literal.parse::<f64>().unwrap();
        self.add_token_object(TokenType::Number, Some(Object::Number(literal)));
    }

    fn identifier(&mut self) {
        while Self::is_alpha_numeric(self.peek()) {
            self.advance();
        }
        let text: String = self.source[self.start..self.current].iter().collect();

        if let Some(tk_type) = Self::keyword(&text) {
            self.add_token(tk_type);
        } else {
            self.add_token(TokenType::Identifier);
        }
    }

    fn check_number(&mut self) {
        while Self::is_digit(self.peek()) {
            self.advance();
        }
    }

    fn is_digit(ch: Option<char>) -> bool {
        if let Some(c) = ch {
            c.is_ascii_digit()
        } else {
            false
        }
    }

    fn is_alpha_numeric(ch: Option<char>) -> bool {
        if let Some(c) = ch {
            c.is_ascii_alphanumeric() || c == '_'
        } else {
            false
        }
    }

    fn keyword(text: &str) -> Option<TokenType> {
        match text {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "for" => Some(TokenType::For),
            "func" => Some(TokenType::Func),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "while" => Some(TokenType::While),
            _ => None,
        }
    }

    fn is_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            false
        } else if self.peek() != Some(expected) {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn add_token(&mut self, tk_type: TokenType) {
        // Some(1);
        self.add_token_object(tk_type, None);
    }

    fn add_token_object(&mut self, tk_type: TokenType, literal: Option<Object>) {
        // let s = &self.source[self.start..self.current];  // String::from_iter(s)
        let lexeme: String = self.source[self.start..self.current].iter().collect();

        self.tokens
            .push(Token::new(tk_type, lexeme, literal, self.line));
    }

    fn advance(&mut self) -> Option<char> {
        let result = self.peek();
        //上面的表用 self.peek() 返回 Option<&char> 后会导致下一行报错， TODO
        self.current += 1;

        result
    }
}
