mod bytecode;
mod macros;
mod value;

pub use bytecode::{Chunk, Emit, Instruction};
pub use value::Value;

pub struct VirtualMachine {
    pc: usize,
    stack: Vec<Value>,
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            pc: 0,
            stack: vec![],
        }
    }

    pub fn run(&mut self, chunk: &Chunk) -> Option<Value> {
        self.pc = 0;
        self.stack = vec![];

        loop {
            let instruction = &chunk.instructions()[self.pc];

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
                    let constant = &chunk.constants()[*i];
                    self.stack.push(constant.clone());
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
