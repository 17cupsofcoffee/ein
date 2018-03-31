#[allow(unknown_lints)]
#[allow(clippy)]
mod grammar;

pub use self::grammar::ProgramParser;

#[cfg(test)]
mod test {
    use super::*;
    use ast::{Expr, Operator, Stmt};
    use lexer::Lexer;

    fn parse(input: &str, expected: Vec<Stmt>) {
        let lexer = Lexer::new(input);
        let parser = ProgramParser::new();
        assert_eq!(expected, parser.parse(lexer).unwrap());
    }

    #[test]
    fn literals() {
        parse("nil", vec![Stmt::ExprStmt(Expr::Nil)]);
        parse("true", vec![Stmt::ExprStmt(Expr::BooleanLiteral(true))]);
        parse("false", vec![Stmt::ExprStmt(Expr::BooleanLiteral(false))]);
        parse("123", vec![Stmt::ExprStmt(Expr::NumberLiteral(123.0))]);
        parse("123.45", vec![Stmt::ExprStmt(Expr::NumberLiteral(123.45))]);
        parse(
            "\"string\"",
            vec![Stmt::ExprStmt(Expr::StringLiteral("string".to_string()))],
        );
    }

    #[test]
    fn identifiers() {
        parse(
            "id",
            vec![Stmt::ExprStmt(Expr::Identifier("id".to_string()))],
        );

        parse(
            "_id",
            vec![Stmt::ExprStmt(Expr::Identifier("_id".to_string()))],
        );

        parse(
            "id123",
            vec![Stmt::ExprStmt(Expr::Identifier("id123".to_string()))],
        );
    }

    #[test]
    fn assignment() {
        parse(
            "x = 10",
            vec![
                Stmt::ExprStmt(Expr::Assign(
                    "x".to_string(),
                    Box::new(Expr::NumberLiteral(10.0)),
                )),
            ],
        );

        parse(
            "x = y = 10",
            vec![
                Stmt::ExprStmt(Expr::Assign(
                    "x".to_string(),
                    Box::new(Expr::Assign(
                        "y".to_string(),
                        Box::new(Expr::NumberLiteral(10.0)),
                    )),
                )),
            ],
        );
    }

    #[test]
    fn equality() {
        parse(
            "nil == nil",
            vec![
                Stmt::ExprStmt(Expr::BinaryOp(
                    Operator::Equals,
                    Box::new(Expr::Nil),
                    Box::new(Expr::Nil),
                )),
            ],
        );

        parse(
            "nil != nil",
            vec![
                Stmt::ExprStmt(Expr::BinaryOp(
                    Operator::NotEquals,
                    Box::new(Expr::Nil),
                    Box::new(Expr::Nil),
                )),
            ],
        );
    }

    #[test]
    fn comparison() {
        parse(
            "10 > 5",
            vec![
                Stmt::ExprStmt(Expr::BinaryOp(
                    Operator::GreaterThan,
                    Box::new(Expr::NumberLiteral(10.0)),
                    Box::new(Expr::NumberLiteral(5.0)),
                )),
            ],
        );

        parse(
            "10 >= 5",
            vec![
                Stmt::ExprStmt(Expr::BinaryOp(
                    Operator::GreaterEquals,
                    Box::new(Expr::NumberLiteral(10.0)),
                    Box::new(Expr::NumberLiteral(5.0)),
                )),
            ],
        );

        parse(
            "10 < 5",
            vec![
                Stmt::ExprStmt(Expr::BinaryOp(
                    Operator::LessThan,
                    Box::new(Expr::NumberLiteral(10.0)),
                    Box::new(Expr::NumberLiteral(5.0)),
                )),
            ],
        );

        parse(
            "10 <= 5",
            vec![
                Stmt::ExprStmt(Expr::BinaryOp(
                    Operator::LessEquals,
                    Box::new(Expr::NumberLiteral(10.0)),
                    Box::new(Expr::NumberLiteral(5.0)),
                )),
            ],
        );
    }

    #[test]
    fn addition() {
        parse(
            "1 + 1",
            vec![
                Stmt::ExprStmt(Expr::BinaryOp(
                    Operator::Add,
                    Box::new(Expr::NumberLiteral(1.0)),
                    Box::new(Expr::NumberLiteral(1.0)),
                )),
            ],
        );
    }

    #[test]
    fn subtraction() {
        parse(
            "1 - 1",
            vec![
                Stmt::ExprStmt(Expr::BinaryOp(
                    Operator::Subtract,
                    Box::new(Expr::NumberLiteral(1.0)),
                    Box::new(Expr::NumberLiteral(1.0)),
                )),
            ],
        );
    }

    #[test]
    fn multiplication() {
        parse(
            "1 * 1",
            vec![
                Stmt::ExprStmt(Expr::BinaryOp(
                    Operator::Multiply,
                    Box::new(Expr::NumberLiteral(1.0)),
                    Box::new(Expr::NumberLiteral(1.0)),
                )),
            ],
        );
    }

    #[test]
    fn division() {
        parse(
            "1 / 1",
            vec![
                Stmt::ExprStmt(Expr::BinaryOp(
                    Operator::Divide,
                    Box::new(Expr::NumberLiteral(1.0)),
                    Box::new(Expr::NumberLiteral(1.0)),
                )),
            ],
        );
    }

    #[test]
    fn unary() {
        parse(
            "!true",
            vec![
                Stmt::ExprStmt(Expr::UnaryOp(
                    Operator::Not,
                    Box::new(Expr::BooleanLiteral(true)),
                )),
            ],
        );

        parse(
            "-true",
            vec![
                Stmt::ExprStmt(Expr::UnaryOp(
                    Operator::UnaryMinus,
                    Box::new(Expr::BooleanLiteral(true)),
                )),
            ],
        );
    }

    #[test]
    fn precedence() {
        // TODO: Might be worth adding test cases for the other ops
        parse(
            "x = -1 * 2 + 3 > 4 != 5",
            vec![
                Stmt::ExprStmt(Expr::Assign(
                    "x".to_string(),
                    Box::new(Expr::BinaryOp(
                        Operator::NotEquals,
                        Box::new(Expr::BinaryOp(
                            Operator::GreaterThan,
                            Box::new(Expr::BinaryOp(
                                Operator::Add,
                                Box::new(Expr::BinaryOp(
                                    Operator::Multiply,
                                    Box::new(Expr::UnaryOp(
                                        Operator::UnaryMinus,
                                        Box::new(Expr::NumberLiteral(1.0)),
                                    )),
                                    Box::new(Expr::NumberLiteral(2.0)),
                                )),
                                Box::new(Expr::NumberLiteral(3.0)),
                            )),
                            Box::new(Expr::NumberLiteral(4.0)),
                        )),
                        Box::new(Expr::NumberLiteral(5.0)),
                    )),
                )),
            ],
        );
    }

    #[test]
    fn grouping() {
        parse(
            "2 * (1 + 2) * 3",
            vec![
                Stmt::ExprStmt(Expr::BinaryOp(
                    Operator::Multiply,
                    Box::new(Expr::BinaryOp(
                        Operator::Multiply,
                        Box::new(Expr::NumberLiteral(2.0)),
                        Box::new(Expr::BinaryOp(
                            Operator::Add,
                            Box::new(Expr::NumberLiteral(1.0)),
                            Box::new(Expr::NumberLiteral(2.0)),
                        )),
                    )),
                    Box::new(Expr::NumberLiteral(3.0)),
                )),
            ],
        );
    }

    #[test]
    fn declaration() {
        parse(
            "let x = 10",
            vec![
                Stmt::Declaration("x".to_string(), Expr::NumberLiteral(10.0)),
            ],
        );
    }

    #[test]
    fn program() {
        parse(
            "\nx\ny\n\nz\n",
            vec![
                Stmt::ExprStmt(Expr::Identifier("x".to_string())),
                Stmt::ExprStmt(Expr::Identifier("y".to_string())),
                Stmt::ExprStmt(Expr::Identifier("z".to_string())),
            ],
        );
    }
}
