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
    LoadConstant(u8),
    LoadGlobal(u8),

    // Stores
    DefineGlobal(u8),
    StoreGlobal(u8),

    // Jumps
    Jump(u8),
    JumpIfTrue(u8),
    JumpIfFalse(u8),
    Loop(u8),

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

                    let jump = chunk.add_instruction(Instruction::JumpIfFalse(0));

                    chunk.add_instruction(Instruction::Pop);
                    rhs.emit(chunk);

                    chunk.patch_jump(jump);
                }

                BinaryOp::Or => {
                    lhs.emit(chunk);

                    let jump = chunk.add_instruction(Instruction::JumpIfTrue(0));

                    chunk.add_instruction(Instruction::Pop);
                    rhs.emit(chunk);

                    chunk.patch_jump(jump)
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

            Stmt::If(condition, when_true, when_false) => {
                condition.emit(chunk);

                let else_jump = chunk.add_instruction(Instruction::JumpIfFalse(0));

                chunk.add_instruction(Instruction::Pop);

                when_true.emit(chunk);

                let then_jump = chunk.add_instruction(Instruction::Jump(0));

                chunk.patch_jump(else_jump);
                chunk.add_instruction(Instruction::Pop);

                when_false.emit(chunk);

                chunk.patch_jump(then_jump);
            }

            Stmt::While(condition, body) => {
                let loop_start = chunk.next_instruction();

                condition.emit(chunk);

                let exit_jump = chunk.add_instruction(Instruction::JumpIfFalse(0));

                chunk.add_instruction(Instruction::Pop);

                body.emit(chunk);

                chunk.emit_loop(loop_start);
                chunk.patch_jump(exit_jump);
                chunk.add_instruction(Instruction::Pop);
            }

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

    pub fn get_instruction_mut(&mut self, addr: usize) -> &mut Instruction {
        &mut self.instructions[addr]
    }

    pub fn set_instruction(&mut self, addr: usize, instruction: Instruction) {
        self.instructions[addr] = instruction;
    }

    pub fn prev_instruction(&self) -> usize {
        self.instructions.len() - 1
    }

    pub fn next_instruction(&self) -> usize {
        self.instructions.len()
    }

    pub fn patch_jump(&mut self, addr: usize) {
        let next = self.next_instruction();

        match self.get_instruction_mut(addr) {
            Instruction::Jump(old)
            | Instruction::JumpIfTrue(old)
            | Instruction::JumpIfFalse(old) => {
                *old = (next - addr) as u8 - 1;
            }
            other => panic!("{:?} is not a jump instruction", other),
        }
    }

    pub fn emit_loop(&mut self, target: usize) {
        let next = self.next_instruction();

        self.add_instruction(Instruction::Loop((next - target) as u8 + 1));
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
