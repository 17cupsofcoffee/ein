mod env;
mod native;
mod value;

use ein_syntax::ast::{BinaryOp, Expr, Stmt, UnaryOp};

use crate::env::{Env, EnvRef};
use crate::native::NativeFn;
pub use crate::value::Value;

pub struct Context {
    stack: Vec<EnvRef>,
}

impl Context {
    pub fn new() -> Context {
        let env = Env::root();

        env.borrow_mut().declare(
            "print".to_string(),
            Value::NativeFunction(NativeFn(native::print)),
        );

        Context { stack: vec![env] }
    }

    pub fn current_env(&self) -> EnvRef {
        self.stack
            .last()
            .expect("The stack should never be unrooted")
            .clone()
    }

    pub fn push_env(&mut self, env: EnvRef) {
        self.stack.push(env);
    }

    pub fn pop_env(&mut self) -> EnvRef {
        if self.stack.len() == 1 {
            panic!("The stack should never be unrooted");
        }

        self.stack
            .pop()
            .expect("The stack should never be unrooted")
    }
}

pub trait Evaluate<T> {
    fn eval(&self, ctx: &mut Context) -> Result<T, String>;

    fn eval_in_env(&self, ctx: &mut Context, env: EnvRef) -> Result<T, String> {
        ctx.push_env(env);
        let val = self.eval(ctx);
        ctx.pop_env();
        val
    }
}

impl Evaluate<Option<Value>> for Vec<Stmt> {
    fn eval(&self, ctx: &mut Context) -> Result<Option<Value>, String> {
        for stmt in self {
            if let Some(ret) = stmt.eval(ctx)? {
                return Ok(Some(ret));
            }
        }

        Ok(None)
    }
}

impl Evaluate<Option<Value>> for Stmt {
    fn eval(&self, ctx: &mut Context) -> Result<Option<Value>, String> {
        match self {
            Stmt::Return(expr) => Ok(Some(expr.eval(ctx)?)),

            Stmt::ExprStmt(expr) => {
                expr.eval(ctx)?;
                Ok(None)
            }

            Stmt::Declaration(name, expr) => {
                // TODO: Re-inline this once NLL is on stable
                let value = expr.eval(ctx)?;
                let env = ctx.current_env();
                env.borrow_mut().declare(name.clone(), value);
                Ok(None)
            }

            Stmt::If(condition, when_true, when_false) => {
                let cond_val = condition.eval(ctx)?;

                if cond_val.is_truthy() {
                    when_true.eval(ctx)
                } else {
                    when_false.eval(ctx)
                }
            }

            Stmt::While(condition, body) => {
                while condition.eval(ctx)?.is_truthy() {
                    if let Some(ret) = body.eval(ctx)? {
                        return Ok(Some(ret));
                    }
                }
                Ok(None)
            }
        }
    }
}

impl Evaluate<Value> for Expr {
    fn eval(&self, ctx: &mut Context) -> Result<Value, String> {
        match self {
            Expr::Nil => Ok(Value::Nil),
            Expr::BooleanLiteral(val) => Ok(Value::Boolean(*val)),
            Expr::NumberLiteral(val) => Ok(Value::Number(*val)),
            Expr::StringLiteral(val) => Ok(Value::String(val.clone())),

            Expr::Identifier(name) => {
                // TODO: Re-inline this once NLL is on stable
                let env = ctx.current_env();
                let resolved = env.borrow().get(name);

                match resolved {
                    Some(value) => Ok(value),
                    None => Ok(Value::Nil),
                }
            }

            Expr::Assign(name, expr) => {
                // TODO: Re-inline this once NLL is on stable
                let expr_val = expr.eval(ctx)?;
                let env = ctx.current_env();
                env.borrow_mut().assign(name.clone(), expr_val.clone())?;
                Ok(expr_val)
            }

            Expr::Function(params, body) => Ok(Value::Function(
                params.clone(),
                body.clone(),
                ctx.current_env(),
            )),

            Expr::Call(target, args) => {
                let target_val = target.eval(ctx)?;

                match target_val {
                    Value::Function(params, body, env) => {
                        if args.len() != params.len() {
                            return Err(format!(
                                "Expected {} arguments, found {}",
                                params.len(),
                                args.len()
                            ));
                        }

                        // TODO: Proper lexical scoping
                        let fn_env = Env::child(&env);

                        for (arg, name) in args.iter().zip(params) {
                            fn_env.borrow_mut().declare(name.clone(), arg.eval(ctx)?);
                        }

                        if let Some(ret) = body.eval_in_env(ctx, fn_env)? {
                            return Ok(ret);
                        }

                        Ok(Value::Nil)
                    }

                    Value::NativeFunction(pointer) => pointer.0(ctx, args),

                    other => Err(format!("{} is not a callable object", other)),
                }
            }

            Expr::UnaryOp(op, expr) => {
                let expr_val = expr.eval(ctx)?;

                match op {
                    UnaryOp::Not => Ok(Value::Boolean(!expr_val.is_truthy())),

                    UnaryOp::UnaryMinus => match expr_val {
                        Value::Number(val) => Ok(Value::Number(-val)),
                        other => Err(format!("{} cannot be negated", other)),
                    },
                }
            }

            Expr::BinaryOp(op, left, right) => {
                match op {
                    // Arithmatic
                    BinaryOp::Add => {
                        let left_val = left.eval(ctx)?;
                        let right_val = right.eval(ctx)?;

                        match (left_val, right_val) {
                            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                            (Value::String(l), Value::String(r)) => Ok(Value::String(l + &r)),
                            (l, r) => Err(format!("{} and {} cannot be added", l, r)),
                        }
                    }

                    BinaryOp::Subtract => {
                        let left_val = left.eval(ctx)?;
                        let right_val = right.eval(ctx)?;

                        match (left_val, right_val) {
                            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
                            (l, r) => Err(format!("{} and {} cannot be subtracted", l, r)),
                        }
                    }

                    BinaryOp::Multiply => {
                        let left_val = left.eval(ctx)?;
                        let right_val = right.eval(ctx)?;

                        match (left_val, right_val) {
                            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
                            (l, r) => Err(format!("{} and {} cannot be multiplied", l, r)),
                        }
                    }

                    BinaryOp::Divide => {
                        let left_val = left.eval(ctx)?;
                        let right_val = right.eval(ctx)?;

                        match (left_val, right_val) {
                            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l / r)),
                            (l, r) => Err(format!("{} and {} cannot be divided", l, r)),
                        }
                    }

                    // Comparison
                    BinaryOp::Equals => {
                        let left_val = left.eval(ctx)?;
                        let right_val = right.eval(ctx)?;

                        Ok(Value::Boolean(left_val == right_val))
                    }

                    BinaryOp::NotEquals => {
                        let left_val = left.eval(ctx)?;
                        let right_val = right.eval(ctx)?;

                        Ok(Value::Boolean(left_val != right_val))
                    }

                    BinaryOp::GreaterThan => {
                        let left_val = left.eval(ctx)?;
                        let right_val = right.eval(ctx)?;

                        match (left_val, right_val) {
                            (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l > r)),
                            (l, r) => Err(format!("{} and {} cannot be compared", l, r)),
                        }
                    }

                    BinaryOp::GreaterEquals => {
                        let left_val = left.eval(ctx)?;
                        let right_val = right.eval(ctx)?;

                        match (left_val, right_val) {
                            (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l >= r)),
                            (l, r) => Err(format!("{} and {} cannot be compared", l, r)),
                        }
                    }

                    BinaryOp::LessThan => {
                        let left_val = left.eval(ctx)?;
                        let right_val = right.eval(ctx)?;

                        match (left_val, right_val) {
                            (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l < r)),
                            (l, r) => Err(format!("{} and {} cannot be compared", l, r)),
                        }
                    }

                    BinaryOp::LessEquals => {
                        let left_val = left.eval(ctx)?;
                        let right_val = right.eval(ctx)?;

                        match (left_val, right_val) {
                            (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l <= r)),
                            (l, r) => Err(format!("{} and {} cannot be compared", l, r)),
                        }
                    }

                    // Logic
                    BinaryOp::And => {
                        let left_val = left.eval(ctx)?;

                        if left_val.is_falsey() {
                            Ok(left_val)
                        } else {
                            right.eval(ctx)
                        }
                    }

                    BinaryOp::Or => {
                        let left_val = left.eval(ctx)?;

                        if left_val.is_truthy() {
                            Ok(left_val)
                        } else {
                            right.eval(ctx)
                        }
                    }
                }
            }
        }
    }
}
