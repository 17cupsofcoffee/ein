use ein_syntax::ast::{BinaryOp, Expr, Stmt, UnaryOp};

use crate::Value;

#[derive(Debug)]
pub enum Instruction {
    // Stack control
    Return,
    Pop,

    // Loads
    LoadNil,
    LoadTrue,
    LoadFalse,
    LoadConstant(usize),

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

            Expr::Identifier(_) => unimplemented!(),

            Expr::NumberLiteral(v) => {
                let constant = chunk.add_constant(Value::Number(*v));
                chunk.add_instruction(Instruction::LoadConstant(constant));
            }

            Expr::StringLiteral(_) => unimplemented!(),

            Expr::BooleanLiteral(v) => {
                chunk.add_instruction(if *v {
                    Instruction::LoadTrue
                } else {
                    Instruction::LoadFalse
                });
            }

            Expr::Assign(_, _) => unimplemented!(),

            Expr::Function(_, _) => unimplemented!(),

            Expr::Call(_, _) => unimplemented!(),

            Expr::UnaryOp(op, val) => {
                val.emit(chunk);

                chunk.add_instruction(match op {
                    UnaryOp::Not => unimplemented!(),
                    UnaryOp::UnaryMinus => Instruction::Negate,
                });
            }

            Expr::BinaryOp(op, lhs, rhs) => {
                lhs.emit(chunk);
                rhs.emit(chunk);

                chunk.add_instruction(match op {
                    BinaryOp::And => unimplemented!(),
                    BinaryOp::Or => unimplemented!(),
                    BinaryOp::Equals => unimplemented!(),
                    BinaryOp::NotEquals => unimplemented!(),
                    BinaryOp::GreaterThan => unimplemented!(),
                    BinaryOp::GreaterEquals => unimplemented!(),
                    BinaryOp::LessThan => unimplemented!(),
                    BinaryOp::LessEquals => unimplemented!(),
                    BinaryOp::Add => Instruction::Add,
                    BinaryOp::Subtract => Instruction::Subtract,
                    BinaryOp::Multiply => Instruction::Multiply,
                    BinaryOp::Divide => Instruction::Divide,
                });
            }
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

            Stmt::Declaration(_, _) => unimplemented!(),

            Stmt::If(_, _, _) => unimplemented!(),

            Stmt::While(_, _) => unimplemented!(),
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

    pub fn constants(&self) -> &[Value] {
        &self.constants
    }

    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }

    pub fn add_instruction(&mut self, instruction: Instruction) -> usize {
        let i = self.instructions.len();
        self.instructions.push(instruction);
        i
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        let i = self.constants.len();
        self.constants.push(value);
        i
    }

    pub fn emit<T>(&mut self, node: &T)
    where
        T: Emit,
    {
        node.emit(self)
    }
}
