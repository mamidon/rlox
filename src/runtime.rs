use std::fmt::{Debug, Formatter, Result};

pub enum LoxValue {
    Number(f64)
}

impl Debug for LoxValue {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            LoxValue::Number(foo) => write!(f, "{}", foo)
        }
    }
}
