use crate::stmt::*;
use crate::token_type::TokenType;
use crate::{error::LoxError, token::Token};

mod parser_expr;
mod parser_stmt;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: &Vec<Token>) -> Self {
        let mut own_tokens: Vec<Token> = vec![];
        for token in tokens {
            own_tokens.push(token.clone())
        }
        Self {
            tokens: own_tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, LoxError> {
        self.program()
    }

    fn peek(&self) -> Option<Token> {
        // match self.tokens.get(self.current) {
        //     None => None,
        //     Some(token) => Some(token.clone()),
        // }

        // self.tokens.get(self.current).map(|token| token.clone())
        self.tokens.get(self.current).cloned()
    }

    fn previous(&self) -> Option<Token> {
        self.tokens.get(self.current - 1).cloned()
    }

    fn is_match(&mut self, types: &Vec<TokenType>) -> bool {
        for tk_type in types {
            if self.is_expect(*tk_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn is_expect(&self, token_type: TokenType) -> bool {
        !self.is_at_end() && self.peek().unwrap().tk_type == token_type
    }

    fn is_at_end(&self) -> bool {
        match self.peek() {
            None => true,
            Some(token) => token.tk_type == TokenType::Eof,
        }
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn consume(&mut self, tk_type: TokenType, message: &str) -> Result<Token, LoxError> {
        if self.is_expect(tk_type) {
            self.advance();
            return Ok(self.previous().unwrap());
        }
        let token = self.peek().unwrap();
        let line = token.line;
        let mut message = message.to_string();
        if token.tk_type == TokenType::Eof {
            message = format!("parser error at end {}", message);
        } else {
            message = format!("parser error at `{}` {}", token.lexeme, message);
        }

        Err(LoxError::error(line, message))
    }

    pub fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().unwrap().tk_type == TokenType::SemiColon {
                return;
            }

            match self.peek().unwrap().tk_type {
                TokenType::Class
                | TokenType::Func
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => (),
            }

            self.advance();
        }
    }
}
