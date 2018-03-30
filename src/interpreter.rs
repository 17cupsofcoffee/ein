use ast::{Expr, Operator};

#[derive(Debug, PartialEq)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    // TODO: Don't copy strings, intern them
    String(String),
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match *self {
            Value::Nil | Value::Boolean(false) => false,
            _ => true,
        }
    }
}

pub trait Evaluate {
    fn eval(&self) -> Result<Value, String>;
}

impl Evaluate for Expr {
    fn eval(&self) -> Result<Value, String> {
        match *self {
            Expr::Nil => Ok(Value::Nil),
            Expr::BooleanLiteral(val) => Ok(Value::Boolean(val)),
            Expr::NumberLiteral(val) => Ok(Value::Number(val)),
            Expr::StringLiteral(ref val) => Ok(Value::String(val.clone())),

            Expr::Identifier(_) => unimplemented!(),

            Expr::UnaryOp(ref op, ref expr) => {
                let expr_val = expr.eval()?;

                match *op {
                    Operator::Not => Ok(Value::Boolean(!expr_val.is_truthy())),

                    Operator::UnaryMinus => match expr_val {
                        Value::Number(val) => Ok(Value::Number(-val)),
                        other => Err(format!("{:?} cannot be negated", other)),
                    },

                    ref other => Err(format!("{:?} is not a valid unary operator", other)),
                }
            }

            Expr::BinaryOp(ref op, ref left, ref right) => {
                let left_val = left.eval()?;
                let right_val = right.eval()?;

                match *op {
                    // Arithmatic
                    Operator::Add => match (left_val, right_val) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                        (Value::String(l), Value::String(r)) => Ok(Value::String(l + &r)),
                        (l, r) => Err(format!("{:?} and {:?} cannot be added", l, r)),
                    },

                    Operator::Subtract => match (left_val, right_val) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
                        (l, r) => Err(format!("{:?} and {:?} cannot be subtracted", l, r)),
                    },

                    Operator::Multiply => match (left_val, right_val) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
                        (l, r) => Err(format!("{:?} and {:?} cannot be multiplied", l, r)),
                    },

                    Operator::Divide => match (left_val, right_val) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l / r)),
                        (l, r) => Err(format!("{:?} and {:?} cannot be added", l, r)),
                    },

                    // Comparison
                    Operator::Equals => Ok(Value::Boolean(left_val == right_val)),
                    Operator::NotEquals => Ok(Value::Boolean(left_val != right_val)),

                    Operator::GreaterThan => match (left_val, right_val) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l > r)),
                        (l, r) => Err(format!("{:?} and {:?} cannot be compared", l, r)),
                    },

                    Operator::GreaterEquals => match (left_val, right_val) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l >= r)),
                        (l, r) => Err(format!("{:?} and {:?} cannot be compared", l, r)),
                    },

                    Operator::LessThan => match (left_val, right_val) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l < r)),
                        (l, r) => Err(format!("{:?} and {:?} cannot be compared", l, r)),
                    },

                    Operator::LessEquals => match (left_val, right_val) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l >= r)),
                        (l, r) => Err(format!("{:?} and {:?} cannot be compared", l, r)),
                    },

                    ref other => Err(format!("{:?} is not a valid binary operator", other)),
                }
            }
        }
    }
}
