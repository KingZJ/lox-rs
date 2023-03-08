use crate::core::Object;

use super::token_type::TokenType;
use std::fmt;

#[derive(Debug)]
pub struct Token {
    pub tk_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Object>,
    pub line: usize,
}

impl Token {
    pub fn new(tk_type: TokenType, lexeme: String, literal: Option<Object>, line: usize) -> Self {
        Self {
            tk_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn eof(line: usize) -> Self {
        Self {
            tk_type: TokenType::Eof,
            lexeme: "".to_owned(),
            literal: None,
            line,
        }
    }

    pub fn is(&self, expect: TokenType) -> bool {
        self.tk_type == expect
    }

    pub fn as_string(&self) -> String {
        self.lexeme.clone()
    }
}

impl Clone for Token {
    fn clone(&self) -> Self {
        Self {
            tk_type: self.tk_type,
            lexeme: String::from(&self.lexeme),
            literal: self.literal.clone(),
            line: self.line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let literal = match self.literal {
            Some(ref x) => format!("{}", x),
            None => String::from("none"),
        };
        write!(f, "{:?} {} {}", self.tk_type, self.lexeme, literal)
    }
}

// pub enum Token {
//     Literal{},
//     Keyword {},
// }
