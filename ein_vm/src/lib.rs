mod bytecode;
mod macros;
mod value;

use std::fmt::{self, Display, Formatter};

use hashbrown::HashMap;

pub use bytecode::{Chunk, Emit, Instruction};
pub use value::Value;

pub enum RuntimeError {
    UndefinedName { name: String },
    InvalidOperation { reason: String },
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            RuntimeError::UndefinedName { name } => write!(f, "{} is undefined", name),
            RuntimeError::InvalidOperation { reason } => write!(f, "Invalid operation: {}", reason),
        }
    }
}

fn is_falsey(value: &Value) -> bool {
    match value {
        Value::Nil | Value::Boolean(false) => true,
        _ => false,
    }
}

fn is_truthy(value: &Value) -> bool {
    !is_falsey(value)
}

pub struct VirtualMachine {
    pc: usize,
    stack: Vec<Value>,
    globals: HashMap<String, Value>,
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            pc: 0,
            stack: vec![],
            globals: HashMap::new(),
        }
    }

    pub fn run(&mut self, chunk: &Chunk) -> Result<Option<Value>, RuntimeError> {
        self.pc = 0;
        self.stack = vec![];

        loop {
            let instruction = chunk.get_instruction(self.pc);

            println!("[{:04X}] {:?}", self.pc, instruction);

            self.pc += 1;

            match instruction {
                Instruction::Return => {
                    return Ok(self.stack.last().cloned());
                }

                Instruction::Pop => {
                    self.stack.pop().unwrap();
                }

                Instruction::LoadNil => {
                    self.stack.push(Value::Nil);
                }

                Instruction::LoadTrue => {
                    self.stack.push(Value::Boolean(true));
                }

                Instruction::LoadFalse => {
                    self.stack.push(Value::Boolean(false));
                }

                Instruction::LoadConstant(i) => {
                    let constant = chunk.get_constant(*i);
                    self.stack.push(constant.clone());
                }

                Instruction::LoadGlobal(i) => {
                    let constant = chunk.get_constant(*i);

                    if let Value::String(name) = constant {
                        match self.globals.get(name) {
                            Some(value) => self.stack.push(value.clone()),
                            None => return Err(RuntimeError::UndefinedName { name: name.clone() }),
                        }
                    } else {
                        panic!("{} is not a valid global name", constant);
                    }
                }

                Instruction::DefineGlobal(i) => {
                    let constant = chunk.get_constant(*i);

                    if let Value::String(name) = constant {
                        self.globals.insert(name.clone(), self.stack.pop().unwrap());
                    } else {
                        panic!("{} is not a valid global name", constant);
                    }
                }

                Instruction::StoreGlobal(i) => {
                    let constant = chunk.get_constant(*i);

                    if let Value::String(name) = constant {
                        match self.globals.get_mut(name) {
                            Some(old_value) => *old_value = self.stack.last().unwrap().clone(),
                            None => return Err(RuntimeError::UndefinedName { name: name.clone() }),
                        }
                    } else {
                        panic!("{} is not a valid global name", constant);
                    }
                }

                Instruction::Jump(offset) => {
                    self.pc += *offset as usize;
                }

                Instruction::JumpIfTrue(offset) => {
                    let val = self.stack.last().unwrap();

                    if is_truthy(val) {
                        self.pc += *offset as usize;
                    }
                }

                Instruction::JumpIfFalse(offset) => {
                    let val = self.stack.last().unwrap();

                    if is_falsey(val) {
                        self.pc += *offset as usize;
                    }
                }

                Instruction::Loop(offset) => {
                    self.pc -= *offset as usize;
                }

                Instruction::Add => arith_impl!(self, "add", +),
                Instruction::Subtract => arith_impl!(self, "subtract", -),
                Instruction::Multiply => arith_impl!(self, "multiply", *),
                Instruction::Divide => arith_impl!(self, "divide", /),

                Instruction::Negate => {
                    let val = self.stack.pop().unwrap();

                    match val {
                        Value::Number(i) => self.stack.push(Value::Number(-i)),
                        other => {
                            return Err(RuntimeError::InvalidOperation {
                                reason: format!("{} is not a number", other),
                            })
                        }
                    }
                }
            }
        }
    }
}
