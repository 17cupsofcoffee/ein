use ein_syntax::ast::{BinaryOp, Expr, Stmt, UnaryOp};

use crate::Value;

#[derive(Debug)]
pub enum Instruction {
    // No-op
    NoOp,

    // Stack control
    Return,
    Pop,

    // Loads
    LoadNil,
    LoadTrue,
    LoadFalse,
    LoadConstant(u8),
    LoadGlobal(u8),

    // Stores
    DefineGlobal(u8),
    StoreGlobal(u8),

    // Jumps
    Jump(u8),
    JumpIfTrue(u8),
    JumpIfFalse(u8),

    // Operators
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
}

pub trait Emit {
    fn emit(&self, chunk: &mut Chunk);
}

impl Emit for Expr {
    fn emit(&self, chunk: &mut Chunk) {
        match self {
            Expr::Nil => {
                chunk.add_instruction(Instruction::LoadNil);
            }

            Expr::Identifier(name) => {
                let constant = chunk.add_constant(Value::String(name.clone()));
                chunk.add_instruction(Instruction::LoadGlobal(constant));
            }

            Expr::NumberLiteral(v) => {
                let constant = chunk.add_constant(Value::Number(*v));
                chunk.add_instruction(Instruction::LoadConstant(constant));
            }

            Expr::StringLiteral(v) => {
                let constant = chunk.add_constant(Value::String(v.clone()));
                chunk.add_instruction(Instruction::LoadConstant(constant));
            }

            Expr::BooleanLiteral(v) => {
                chunk.add_instruction(if *v {
                    Instruction::LoadTrue
                } else {
                    Instruction::LoadFalse
                });
            }

            Expr::Assign(name, value) => {
                value.emit(chunk);

                let constant = chunk.add_constant(Value::String(name.clone()));
                chunk.add_instruction(Instruction::StoreGlobal(constant));
            }

            Expr::Function(_, _) => unimplemented!(),

            Expr::Call(_, _) => unimplemented!(),

            Expr::UnaryOp(op, val) => {
                val.emit(chunk);

                chunk.add_instruction(match op {
                    UnaryOp::Not => unimplemented!(),
                    UnaryOp::UnaryMinus => Instruction::Negate,
                });
            }

            Expr::BinaryOp(op, lhs, rhs) => match op {
                BinaryOp::And => {
                    lhs.emit(chunk);
                    let jump = chunk.add_instruction(Instruction::NoOp);
                    chunk.add_instruction(Instruction::Pop);
                    rhs.emit(chunk);
                    let jump_end = chunk.instruction_count() - 1;
                    chunk.set_instruction(jump, Instruction::JumpIfFalse((jump_end - jump) as u8));
                }

                BinaryOp::Or => {
                    lhs.emit(chunk);
                    let jump = chunk.add_instruction(Instruction::NoOp);
                    chunk.add_instruction(Instruction::Pop);
                    rhs.emit(chunk);
                    let jump_end = chunk.instruction_count() - 1;
                    chunk.set_instruction(jump, Instruction::JumpIfTrue((jump_end - jump) as u8));
                }

                BinaryOp::Equals => unimplemented!(),
                BinaryOp::NotEquals => unimplemented!(),
                BinaryOp::GreaterThan => unimplemented!(),
                BinaryOp::GreaterEquals => unimplemented!(),
                BinaryOp::LessThan => unimplemented!(),
                BinaryOp::LessEquals => unimplemented!(),

                BinaryOp::Add => {
                    lhs.emit(chunk);
                    rhs.emit(chunk);
                    chunk.add_instruction(Instruction::Add);
                }

                BinaryOp::Subtract => {
                    lhs.emit(chunk);
                    rhs.emit(chunk);
                    chunk.add_instruction(Instruction::Subtract);
                }

                BinaryOp::Multiply => {
                    lhs.emit(chunk);
                    rhs.emit(chunk);
                    chunk.add_instruction(Instruction::Multiply);
                }

                BinaryOp::Divide => {
                    lhs.emit(chunk);
                    rhs.emit(chunk);
                    chunk.add_instruction(Instruction::Divide);
                }
            },
        }
    }
}

impl Emit for Stmt {
    fn emit(&self, chunk: &mut Chunk) {
        match self {
            Stmt::Return(_) => unimplemented!(),

            Stmt::ExprStmt(e) => {
                e.emit(chunk);
                chunk.add_instruction(Instruction::Pop);
            }

            Stmt::Declaration(name, value) => {
                value.emit(chunk);

                let constant = chunk.add_constant(Value::String(name.clone()));
                chunk.add_instruction(Instruction::DefineGlobal(constant));
            }

            Stmt::If(condition, t, f) => {
                condition.emit(chunk);

                let else_start_jump = chunk.add_instruction(Instruction::NoOp);
                chunk.add_instruction(Instruction::Pop);

                t.emit(chunk);

                let then_end_jump = chunk.add_instruction(Instruction::NoOp);

                let else_start = chunk.instruction_count() - 1;

                chunk.add_instruction(Instruction::Pop);

                f.emit(chunk);

                let else_end = chunk.instruction_count() - 1;

                chunk.set_instruction(
                    else_start_jump,
                    Instruction::JumpIfFalse((else_start - else_start_jump) as u8),
                );

                chunk.set_instruction(
                    then_end_jump,
                    Instruction::Jump((else_end - then_end_jump) as u8),
                );
            }

            Stmt::While(_, _) => unimplemented!(),

            Stmt::Block(_) => unimplemented!(),
        }
    }
}

impl Emit for Vec<Stmt> {
    fn emit(&self, chunk: &mut Chunk) {
        for stmt in self {
            stmt.emit(chunk)
        }
    }
}

#[derive(Debug)]
pub struct Chunk {
    constants: Vec<Value>,
    instructions: Vec<Instruction>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            constants: vec![],
            instructions: vec![],
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) -> usize {
        let i = self.instructions.len();
        self.instructions.push(instruction);
        i
    }

    pub fn get_instruction(&self, addr: usize) -> &Instruction {
        &self.instructions[addr]
    }

    pub fn set_instruction(&mut self, addr: usize, instruction: Instruction) {
        self.instructions[addr] = instruction;
    }

    pub fn instruction_count(&self) -> usize {
        self.instructions.len()
    }

    pub fn add_constant(&mut self, value: Value) -> u8 {
        let i = self.constants.len();

        if i >= u8::max_value() as usize {
            panic!("Chunks cannot contain more than 255 constants.");
        }

        self.constants.push(value);
        i as u8
    }

    pub fn get_constant(&self, idx: u8) -> &Value {
        &self.constants[idx as usize]
    }

    pub fn emit<T>(&mut self, node: &T)
    where
        T: Emit,
    {
        node.emit(self)
    }
}
