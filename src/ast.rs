#[derive(Debug, PartialEq)]
pub enum BinaryOp {
    And,
    Or,
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
}

#[derive(Debug, PartialEq)]
pub enum UnaryOp {
    Not,
    UnaryMinus,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Nil,

    Identifier(String),
    NumberLiteral(f64),
    StringLiteral(String),
    BooleanLiteral(bool),

    Assign(String, Box<Expr>),
    Function(Vec<String>, Vec<Stmt>),

    UnaryOp(UnaryOp, Box<Expr>),
    BinaryOp(BinaryOp, Box<Expr>, Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Declaration(String, Expr),
    If(Expr, Vec<Stmt>, Vec<Stmt>),
    // TODO: Replace this with a function
    Print(Expr),
}
