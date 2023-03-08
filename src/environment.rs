use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::{core::Object, error::LoxResult, token::Token};

#[derive(Debug, Default)]
pub struct Environment {
    values: HashMap<String, Object>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn new_enclosing(enclosing: Rc<RefCell<Environment>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<Object, LoxResult> {
        if let Some(obj) = self.values.get(&name.lexeme) {
            Ok(obj.clone())
        } else if let Some(ref enclosing) = self.enclosing {
            enclosing.borrow().get(name)
        } else {
            Err(LoxResult::error(
                name.line,
                format!("undefined variable `{}`", name.lexeme),
            ))
        }
    }

    pub fn assign(&mut self, name: &Token, value: Object) -> Result<(), LoxResult> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.as_string(), value);
            Ok(())
        } else if let Some(ref enclosing) = self.enclosing {
            enclosing.borrow_mut().assign(name, value)
        } else {
            Err(LoxResult::error(
                name.line,
                format!("undefined variable `{}`", name.lexeme),
            ))
        }
    }
}

#[cfg(test)]
mod test {
    // use std::borrow::BorrowMut;
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::core::Object;
    use crate::token::Token;
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

    #[test]
    fn can_enclose_an_environment() {
        let env = Rc::new(RefCell::new(Environment::new()));
        let enclose_env = Environment::new_enclosing(Rc::clone(&env));
        assert_eq!(
            enclose_env.enclosing.unwrap().borrow().values,
            env.borrow().values
        );
    }

    #[test]
    fn can_read_form_enclose_environment() {
        let env = Rc::new(RefCell::new(Environment::new()));
        let lexeme = "test";
        env.borrow_mut()
            .define(lexeme.to_string(), Object::Number(10.0));

        let enclose_env = Environment::new_enclosing(Rc::clone(&env));

        let name = Token::new(TokenType::Identifier, lexeme.to_string(), None, 0);
        assert_eq!(enclose_env.get(&name).unwrap(), Object::Number(10.0));
    }

    #[test]
    fn can_assign_enclose_environment() {
        let env = Rc::new(RefCell::new(Environment::new()));
        let lexeme = "test";
        env.borrow_mut()
            .define(lexeme.to_string(), Object::Number(10.0));

        let mut enclose_env = Environment::new_enclosing(Rc::clone(&env));

        let name = Token::new(TokenType::Identifier, lexeme.to_string(), None, 0);
        assert!(enclose_env.assign(&name, Object::Number(20.0)).is_ok());
        assert_eq!(enclose_env.get(&name).unwrap(), Object::Number(20.0));
        assert_eq!(env.borrow().get(&name).unwrap(), Object::Number(10.0));
    }
}
