#[derive(Debug, PartialEq)]
pub enum Operator {
    // Binary
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

    // Unary
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

    UnaryOp(Operator, Box<Expr>),
    BinaryOp(Operator, Box<Expr>, Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    ExprStmt(Expr),
    Declaration(String, Expr),
    If(Expr, Vec<Stmt>, Vec<Stmt>),
    // TODO: Replace this with a function
    Print(Expr),
}
