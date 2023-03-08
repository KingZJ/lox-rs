use std::fmt;

use super::Callable;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Number(f64),
    Str(String),
    Nil,
    True,
    False,
    Func(Callable),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(x) => write!(f, "{}", x),
            Self::Str(x) => write!(f, "{}", x),
            Self::Nil => write!(f, "nil"),
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::Func(func) => write!(f, "{:?}", func),
        }
    }
}
