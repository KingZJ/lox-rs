use std::time::SystemTime;

use crate::core::*;
use crate::error::*;
use crate::interpreter::Interpreter;

pub struct NativeClock {}

impl NativeClock {
    pub fn new() -> Self {
        Self {}
    }
}

impl Callable for NativeClock {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, LoxResult> {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => Ok(Object::Number(n.as_millis() as f64)),
            Err(e) => Err(LoxResult::system_error(format!(
                "clock return invalid duration {:?}",
                e.duration()
            ))),
        }
    }
}
