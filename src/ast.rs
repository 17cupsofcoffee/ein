#[derive(Debug, PartialEq)]
pub enum Operator {
    Equals,
    NotEquals,
    GreaterThan,
    GreaterEquals,
    LessThan,
    LessEquals,
    Add,
    Subtract,
    Multiply,
    Divide,
    Not,
    UnaryPlus,
    UnaryMinus,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Nil,

    Identifier(String),
    NumberLiteral(f64),
    StringLiteral(String),
    BooleanLiteral(bool),

    UnaryOp(Operator, Box<Expr>),
    BinaryOp(Operator, Box<Expr>, Box<Expr>),
}
