use super::token_type::TokenType;
use std::fmt;
pub enum Object {
    Number(f64),
    Str(String),
    Nil,
    True,
    False,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(x) => write!(f, "{}", x),
            Self::Str(x) => write!(f, "{:?}", x),
            Self::Nil => write!(f, "nil"),
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
        }
    }
}

pub struct Token {
    tk_type: TokenType,
    lexeme: String,
    literal: Option<Object>,
    line: usize,
}

impl Token {
    pub fn new(tk_type: TokenType, lexeme: String, literal: Option<Object>, line: usize) -> Self {
        Self { tk_type, lexeme, literal, line }
    }

    pub fn eof(line: usize) -> Self {
        Self { tk_type: TokenType::Eof, lexeme: "".to_owned(), literal: None, line }
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