use ein_syntax::ast::Stmt;
use env::EnvRef;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    // TODO: Don't copy strings, intern them
    String(String),
    Function(Vec<String>, Vec<Stmt>, EnvRef),
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil | Value::Boolean(false) => false,
            _ => true,
        }
    }

    pub fn is_falsey(&self) -> bool {
        !self.is_truthy()
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Boolean(val) => write!(f, "{}", val),
            Value::Number(val) => write!(f, "{}", val),
            Value::String(val) => write!(f, "{}", val),
            Value::Function(_, _, _) => write!(f, "<fn>"),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        use Value::*;

        match (self, other) {
            (Nil, Nil) => true,
            (Boolean(a), Boolean(b)) => a == b,
            (Number(a), Number(b)) => a == b,
            (String(a), String(b)) => a == b,
            _ => false,
        }
    }
}
