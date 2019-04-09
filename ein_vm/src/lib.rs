mod bytecode;
mod macros;
mod value;

use hashbrown::HashMap;

pub use bytecode::{Chunk, Emit, Instruction};
pub use value::Value;

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

    pub fn run(&mut self, chunk: &Chunk) -> Option<Value> {
        self.pc = 0;
        self.stack = vec![];

        loop {
            let instruction = chunk.get_instruction(self.pc);

            println!("{:04X} {:?}", self.pc, instruction);

            self.pc += 1;

            match instruction {
                Instruction::Return => {
                    return self.stack.last().cloned();
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
                            None => panic!("{} is undefined", name),
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
                            Some(old_value) => *old_value = self.stack.pop().unwrap(),
                            None => panic!("{} is undefined", name),
                        }
                    } else {
                        panic!("{} is not a valid global name", constant);
                    }
                }

                Instruction::Add => arith_impl!(self, "add", +),
                Instruction::Subtract => arith_impl!(self, "subtract", -),
                Instruction::Multiply => arith_impl!(self, "multiply", *),
                Instruction::Divide => arith_impl!(self, "divide", /),

                Instruction::Negate => {
                    let val = self.stack.pop().unwrap();

                    match val {
                        Value::Number(i) => self.stack.push(Value::Number(-i)),
                        other => panic!("Cannot negate {}", other),
                    }
                }
            }
        }
    }
}
