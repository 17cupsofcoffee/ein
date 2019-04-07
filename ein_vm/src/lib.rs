use std::fmt::{self, Display, Formatter};

use ein_syntax::ast::{BinaryOp, Expr, Stmt, UnaryOp};

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Boolean(v) => write!(f, "{}", v),
            Value::Number(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    Return,
    Pop,

    LoadNil,
    LoadTrue,
    LoadFalse,
    LoadConstant(usize),

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

macro_rules! arith_impl {
    ($self:ident, $name:literal, $op:tt) => {
        {
            let rhs = $self.stack.pop().unwrap();
            let lhs = $self.stack.pop().unwrap();

            match (lhs, rhs) {
                (Value::Number(a), Value::Number(b)) => $self.stack.push(Value::Number(a $op b)),
                (other_a, other_b) => panic!("Cannot {} {} and {}", $name, other_a, other_b),
            }
        }
    }
}

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
            let instruction = &chunk.instructions[self.pc];

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
                    let constant = &chunk.constants[*i];
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
