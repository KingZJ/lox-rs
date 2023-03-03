#[derive(Debug)]
pub struct LoxError {
    line: u32,
    message: String,
}

impl LoxError {
    pub fn error(line: u32, message: String) -> Self {
        Self { line, message }
    }

    pub fn report(&self, loc: &str) {
        println!("[line: {}], Error {}: {}", self.line, loc, self.message);
    }
}