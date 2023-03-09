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
    // declaration: Rc<FunctionStmt>,
}

impl LoxFunction {
    pub fn new(declaration: &FunctionStmt) -> Self {
        Self {
            params: declaration.params.clone(),
            name: declaration.name.clone(),
            body: declaration.body.clone(),
            // declaration: Rc::clone(declaration),
        }
    }
}

impl ToString for LoxFunction {
    fn to_string(&self) -> String {
        // match self.declaration.deref() {
        //     Stmt::Function(stmt) => format!("<Func {}>", stmt.name.lexeme),
        //     _ => panic!("Lox look for the string on a non-function statement object"),
        // }
        format!("<Func {}>", self.name.lexeme)
    }
}

impl Callable for LoxFunction {
    fn arity(&self) -> usize {
        // match self.declaration.deref() {
        //     Stmt::Function(stmt) => stmt.params.len(),
        //     _ => panic!("Lox look for the arity on a non-function statement object"),
        // }

        self.params.len()
    }

    fn call(&self, interpreter: &Interpreter, arguments: Vec<Object>) -> Result<Object, LoxResult> {
        let mut environment = Environment::new_enclosing(interpreter.globals.clone());

        // match self.declaration.deref() {
        //     Stmt::Function(stmt) => {
        //         for (param, value) in stmt.params.iter().zip(arguments.into_iter()) {
        //             environment.define(param.as_string(), value)
        //         }

        //         interpreter.execute_block(&stmt.body, environment)?;
        //         Ok(super::Object::Nil)
        //     }
        //     // _ => Err(LoxResult::system_error("only function allow".to_owned())),
        //     _ => panic!("Lox created a callable on a non-function statement object"),
        // }

        for (param, value) in self.params.iter().zip(arguments.into_iter()) {
            environment.define(param.as_string(), value)
        }

        interpreter.execute_block(&self.body, environment)?;
        Ok(super::Object::Nil)
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
