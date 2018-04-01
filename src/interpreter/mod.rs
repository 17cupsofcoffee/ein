mod env;

use std::fmt;
use ast::{Expr, Operator, Stmt};
use self::env::Env;

#[derive(Debug, PartialEq, Clone)]
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

    pub fn is_falsey(&self) -> bool {
        !self.is_truthy()
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Nil => write!(f, "nil"),
            Value::Boolean(val) => write!(f, "{}", val),
            Value::Number(val) => write!(f, "{}", val),
            Value::String(ref val) => write!(f, "{}", val),
        }
    }
}

pub struct Context {
    current_env: Env,
}

impl Context {
    pub fn new() -> Context {
        Context {
            current_env: Env::root(),
        }
    }
}

pub trait Evaluate {
    fn eval(&self, ctx: &mut Context) -> Result<Value, String>;
}

impl Evaluate for Vec<Stmt> {
    fn eval(&self, ctx: &mut Context) -> Result<Value, String> {
        let mut ret = Value::Nil;
        for stmt in self {
            ret = stmt.eval(ctx)?;
        }
        Ok(ret)
    }
}

impl Evaluate for Stmt {
    fn eval(&self, ctx: &mut Context) -> Result<Value, String> {
        match *self {
            Stmt::ExprStmt(ref expr) => expr.eval(ctx),

            Stmt::Declaration(ref name, ref expr) => {
                let value = expr.eval(ctx)?;
                ctx.current_env.declare(name.clone(), value);
                Ok(Value::Nil)
            }

            Stmt::If(ref condition, ref when_true, ref when_false) => {
                let cond_val = condition.eval(ctx)?;

                if cond_val.is_truthy() {
                    when_true.eval(ctx)?;
                } else {
                    when_false.eval(ctx)?;
                }

                Ok(Value::Nil)
            }

            Stmt::Print(ref expr) => {
                let value = expr.eval(ctx)?;
                println!("{}", value);
                Ok(Value::Nil)
            }
        }
    }
}

impl Evaluate for Expr {
    fn eval(&self, ctx: &mut Context) -> Result<Value, String> {
        match *self {
            Expr::Nil => Ok(Value::Nil),
            Expr::BooleanLiteral(val) => Ok(Value::Boolean(val)),
            Expr::NumberLiteral(val) => Ok(Value::Number(val)),
            Expr::StringLiteral(ref val) => Ok(Value::String(val.clone())),

            Expr::Identifier(ref name) => match ctx.current_env.get(name) {
                Some(value) => Ok(value),
                None => Ok(Value::Nil),
            },

            Expr::Assign(ref name, ref expr) => {
                let expr_val = expr.eval(ctx)?;
                ctx.current_env.assign(name.clone(), expr_val.clone())?;
                Ok(expr_val)
            }

            Expr::UnaryOp(ref op, ref expr) => {
                let expr_val = expr.eval(ctx)?;

                match *op {
                    Operator::Not => Ok(Value::Boolean(!expr_val.is_truthy())),

                    Operator::UnaryMinus => match expr_val {
                        Value::Number(val) => Ok(Value::Number(-val)),
                        other => Err(format!("{} cannot be negated", other)),
                    },

                    ref other => Err(format!("{:?} is not a valid unary operator", other)),
                }
            }

            Expr::BinaryOp(ref op, ref left, ref right) => {
                match *op {
                    // Arithmatic
                    Operator::Add => {
                        let left_val = left.eval(ctx)?;
                        let right_val = right.eval(ctx)?;

                        match (left_val, right_val) {
                            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                            (Value::String(l), Value::String(r)) => Ok(Value::String(l + &r)),
                            (l, r) => Err(format!("{} and {} cannot be added", l, r)),
                        }
                    }

                    Operator::Subtract => {
                        let left_val = left.eval(ctx)?;
                        let right_val = right.eval(ctx)?;

                        match (left_val, right_val) {
                            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
                            (l, r) => Err(format!("{} and {} cannot be subtracted", l, r)),
                        }
                    }

                    Operator::Multiply => {
                        let left_val = left.eval(ctx)?;
                        let right_val = right.eval(ctx)?;

                        match (left_val, right_val) {
                            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
                            (l, r) => Err(format!("{} and {} cannot be multiplied", l, r)),
                        }
                    }

                    Operator::Divide => {
                        let left_val = left.eval(ctx)?;
                        let right_val = right.eval(ctx)?;

                        match (left_val, right_val) {
                            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l / r)),
                            (l, r) => Err(format!("{} and {} cannot be divided", l, r)),
                        }
                    }

                    // Comparison
                    Operator::Equals => {
                        let left_val = left.eval(ctx)?;
                        let right_val = right.eval(ctx)?;

                        Ok(Value::Boolean(left_val == right_val))
                    }

                    Operator::NotEquals => {
                        let left_val = left.eval(ctx)?;
                        let right_val = right.eval(ctx)?;

                        Ok(Value::Boolean(left_val != right_val))
                    }

                    Operator::GreaterThan => {
                        let left_val = left.eval(ctx)?;
                        let right_val = right.eval(ctx)?;

                        match (left_val, right_val) {
                            (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l > r)),
                            (l, r) => Err(format!("{} and {} cannot be compared", l, r)),
                        }
                    }

                    Operator::GreaterEquals => {
                        let left_val = left.eval(ctx)?;
                        let right_val = right.eval(ctx)?;

                        match (left_val, right_val) {
                            (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l >= r)),
                            (l, r) => Err(format!("{} and {} cannot be compared", l, r)),
                        }
                    }

                    Operator::LessThan => {
                        let left_val = left.eval(ctx)?;
                        let right_val = right.eval(ctx)?;

                        match (left_val, right_val) {
                            (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l < r)),
                            (l, r) => Err(format!("{} and {} cannot be compared", l, r)),
                        }
                    }

                    Operator::LessEquals => {
                        let left_val = left.eval(ctx)?;
                        let right_val = right.eval(ctx)?;

                        match (left_val, right_val) {
                            (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l <= r)),
                            (l, r) => Err(format!("{} and {} cannot be compared", l, r)),
                        }
                    }

                    // Logic
                    Operator::And => {
                        let left_val = left.eval(ctx)?;

                        if left_val.is_falsey() {
                            Ok(left_val)
                        } else {
                            right.eval(ctx)
                        }
                    }

                    Operator::Or => {
                        let left_val = left.eval(ctx)?;

                        if left_val.is_truthy() {
                            Ok(left_val)
                        } else {
                            right.eval(ctx)
                        }
                    }

                    ref other => Err(format!("{:?} is not a valid binary operator", other)),
                }
            }
        }
    }
}
