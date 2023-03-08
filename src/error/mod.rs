use crate::token::Token;

#[derive(Debug)]
pub enum LoxResult {
    // #[default]
    ParseError { token: Token, message: String },
    RuntimeError { token: Token, message: String },
    LoxError { line: usize, message: String },
}

// #[derive(Debug, Default)]
// pub struct LoxError {
//     line: usize,
//     message: String,
// }

impl LoxResult {
    pub fn error(line: usize, message: String) -> Self {
        let err = Self::LoxError { line, message };
        err.report("Error");
        err
    }

    pub fn parse_error(token: Token, message: String) -> Self {
        let err = Self::ParseError { token, message };
        err.report("Parse Error");
        err
    }

    pub fn runtime_error(token: &Token, message: String) -> Self {
        let err = Self::RuntimeError {
            token: token.clone(),
            message,
        };
        err.report("Runtime Error");
        err
    }

    pub fn report(&self, loc: &str) {
        match self {
            Self::LoxError { line, message } => {
                eprintln!("[line: {}], {}: {}", line, loc, message);
            }
            Self::ParseError { token, message } | Self::RuntimeError { token, message } => {
                if token.is(crate::token_type::TokenType::Eof) {
                    eprintln!("[line: {} at end], {}: {}", token.line, loc, message);
                } else {
                    eprintln!(
                        "[line: {} at `{}`], {}: {}",
                        token.line,
                        loc,
                        message,
                        token.as_string()
                    );
                }
            }
        }
    }
}
