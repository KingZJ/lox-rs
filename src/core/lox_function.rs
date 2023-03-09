use std::rc::Rc;

use crate::environment::Environment;
use crate::error::LoxResult;
use crate::interpreter::Interpreter;
use crate::stmt::*;
use crate::token::Token;

use super::{Callable, Object};

pub struct LoxFunction {
    params: Rc<Vec<Token>>,
    name: Token,
    body: Rc<Vec<Stmt>>,
}

impl LoxFunction {
    pub fn new(declaration: &FunctionStmt) -> Self {
        Self {
            params: declaration.params.clone(),
            name: declaration.name.clone(),
            body: declaration.body.clone(),
        }
    }
}

impl ToString for LoxFunction {
    fn to_string(&self) -> String {
        format!("<Func {}>", self.name.lexeme)
    }
}

impl Callable for LoxFunction {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(&self, interpreter: &Interpreter, arguments: Vec<Object>) -> Result<Object, LoxResult> {
        let mut environment = Environment::new_enclosing(interpreter.globals.clone());

        for (param, value) in self.params.iter().zip(arguments.into_iter()) {
            environment.define(param.as_string(), value)
        }

        match interpreter.execute_block(&self.body, environment) {
            Err(LoxResult::Return { value }) => Ok(value),
            Ok(_) => Ok(Object::Nil),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    #[test]
    fn test1() {
        let five = Rc::new(5);
        let five1 = Rc::clone(&five);
        let five2 = five.clone();
        println!("{}, {}, {}", five, five1, five2);
        println!(
            "{} {} {}",
            Rc::weak_count(&five),
            Rc::strong_count(&five),
            Rc::strong_count(&five2)
        );
        assert_eq!(five, five1);
        assert_eq!(five, five2);
        assert_eq!(five1, five2);
    }
}
