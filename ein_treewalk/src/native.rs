use ein_syntax::ast::Expr;
use std::fmt;
use std::io::{self, Write};
use crate::value::Value;
use crate::Context;
use crate::Evaluate;

#[derive(Clone)]
pub struct NativeFn(pub fn(&mut Context, &[Expr]) -> Result<Value, String>);

impl fmt::Debug for NativeFn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<native fn>")
    }
}

pub fn print(ctx: &mut Context, args: &[Expr]) -> Result<Value, String> {
    let mut args_iter = args.iter();

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    if let Some(arg) = args_iter.next() {
        let value = arg.eval(ctx)?;
        write!(handle, "{}", value).map_err(|e| e.to_string())?;

        for arg in args_iter {
            let value = arg.eval(ctx)?;
            write!(handle, " {}", value).map_err(|e| e.to_string())?;
        }
    }

    writeln!(handle).map_err(|e| e.to_string())?;

    Ok(Value::Nil)
}
