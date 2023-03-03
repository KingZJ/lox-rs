use crate::token::Token;
use crate::error::LoxError;


// type Result<Vec<Token>> = std::result::Result<Vec<Token>, LoxError>;
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    current: usize,
    start: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self { source, tokens: Vec::new(), current: 0, start: 0, line: 1 }
    }
    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, LoxError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::eof(self.line));
        Ok(&self.tokens)
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_token(&mut self) {
        // self.source.lines();
        // self.current += 1;
    }
}