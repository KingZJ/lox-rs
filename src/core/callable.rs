use crate::{error::LoxResult, interpreter::Interpreter};

use super::Object;

#[derive(Debug, PartialEq, Clone)]
pub struct Callable {}

impl Callable {
    pub fn call(
        &self,
        interpreter: &Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, LoxResult> {
        Ok(Object::Nil)
    }
}
