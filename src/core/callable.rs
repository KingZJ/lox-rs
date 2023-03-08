use std::{
    fmt::{self, Debug, Display},
    rc::Rc,
};

use crate::{
    error::LoxResult,
    interpreter::{self, Interpreter},
};

use super::Object;

#[derive(Clone)]
pub struct LoxCallable {
    pub func: Rc<dyn Callable>,
    pub arity: usize,
}

impl Display for LoxCallable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<callable>")
    }
}

impl Debug for LoxCallable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<callable>")
    }
}

impl PartialEq for LoxCallable {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.func, &other.func)
    }
}

impl Callable for LoxCallable {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Object>) -> Result<Object, LoxResult> {
        self.func.call(interpreter, arguments)
    }

    fn arity(&self) -> usize {
        self.arity
    }
}

pub trait Callable {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Object>) -> Result<Object, LoxResult>;

    fn arity(&self) -> usize;
}
