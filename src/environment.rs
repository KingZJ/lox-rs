use std::collections::HashMap;

use crate::{
    error::LoxError,
    token::{Object, Token},
};

#[derive(Debug, Default)]
pub struct Environment {
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<Object, LoxError> {
        if let Some(obj) = self.values.get(&name.lexeme) {
            Ok(obj.clone())
        } else {
            Err(LoxError::error(
                name.line,
                format!("undefined variable `{}`", name.lexeme),
            ))
        }
    }

    pub fn assign(&mut self, name: &Token, value: Object) -> Result<(), LoxError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.as_string(), value);
            Ok(())
        } else {
            Err(LoxError::error(
                name.line,
                format!("undefined variable `{}`", name.lexeme),
            ))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::token::{Object, Token};
    use crate::token_type::TokenType;

    use super::Environment;

    #[test]
    fn test_assign() {
        let mut e = Environment::new();

        let lexeme = "test";
        let token = Token::new(TokenType::Identifier, lexeme.to_string(), None, 0);
        e.define(lexeme.to_string(), Object::Number(10.0));
        assert!(e.assign(&token, Object::Number(20.0)).is_ok());
        assert_eq!(e.get(&token).unwrap(), Object::Number(20.0));
    }
}
