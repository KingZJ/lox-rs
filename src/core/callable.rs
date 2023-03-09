use std::{
    fmt::{self, Debug, Display},
    rc::Rc,
};

use crate::{error::LoxResult, interpreter::Interpreter};

use super::Object;

#[derive(Clone)]
pub struct LoxCallable {
    pub func: Rc<dyn Callable>,
}

impl Display for LoxCallable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.func.to_string())
    }
}

impl Debug for LoxCallable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.func.to_string())
    }
}

impl PartialEq for LoxCallable {
    fn eq(&self, other: &Self) -> bool {
        // Rc::ptr_eq(&self.func, &other.func)
        std::ptr::eq(&self.func, &other.func)
    }
}

impl Callable for LoxCallable {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Object>) -> Result<Object, LoxResult> {
        self.func.call(interpreter, arguments)
    }

    fn arity(&self) -> usize {
        self.func.arity()
    }
}

pub trait Callable: ToString {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Object>) -> Result<Object, LoxResult>;

    // 函数中的参数个数
    fn arity(&self) -> usize;
}
