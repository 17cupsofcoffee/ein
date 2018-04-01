#[allow(unknown_lints)]
#[allow(clippy)]
mod grammar;

pub use self::grammar::{ExprParser, ProgramParser};

#[cfg(test)]
mod test {
    use super::*;
    use ast::{Expr, Operator, Stmt};
    use lexer::Lexer;

    fn parse_stmt(input: &str, expected: Vec<Stmt>) {
        let lexer = Lexer::new(input);
        let parser = ProgramParser::new();
        assert_eq!(expected, parser.parse(lexer).unwrap());
    }

    fn parse_expr(input: &str, expected: Expr) {
        let lexer = Lexer::new(input);
        let parser = ExprParser::new();
        assert_eq!(expected, parser.parse(lexer).unwrap());
    }

    #[test]
    fn literals() {
        parse_expr("nil", Expr::Nil);
        parse_expr("true", Expr::BooleanLiteral(true));
        parse_expr("false", Expr::BooleanLiteral(false));
        parse_expr("123", Expr::NumberLiteral(123.0));
        parse_expr("123.45", Expr::NumberLiteral(123.45));
        parse_expr("\"string\"", Expr::StringLiteral("string".to_string()));
    }

    #[test]
    fn identifiers() {
        parse_expr("id", Expr::Identifier("id".to_string()));
        parse_expr("_id", Expr::Identifier("_id".to_string()));
        parse_expr("id123", Expr::Identifier("id123".to_string()));
    }

    #[test]
    fn assignment() {
        parse_expr(
            "x = 10",
            Expr::Assign("x".to_string(), Box::new(Expr::NumberLiteral(10.0))),
        );

        parse_expr(
            "x = y = 10",
            Expr::Assign(
                "x".to_string(),
                Box::new(Expr::Assign(
                    "y".to_string(),
                    Box::new(Expr::NumberLiteral(10.0)),
                )),
            ),
        );
    }

    #[test]
    fn equality() {
        parse_expr(
            "nil == nil",
            Expr::BinaryOp(Operator::Equals, Box::new(Expr::Nil), Box::new(Expr::Nil)),
        );

        parse_expr(
            "nil != nil",
            Expr::BinaryOp(
                Operator::NotEquals,
                Box::new(Expr::Nil),
                Box::new(Expr::Nil),
            ),
        );
    }

    #[test]
    fn comparison() {
        parse_expr(
            "10 > 5",
            Expr::BinaryOp(
                Operator::GreaterThan,
                Box::new(Expr::NumberLiteral(10.0)),
                Box::new(Expr::NumberLiteral(5.0)),
            ),
        );

        parse_expr(
            "10 >= 5",
            Expr::BinaryOp(
                Operator::GreaterEquals,
                Box::new(Expr::NumberLiteral(10.0)),
                Box::new(Expr::NumberLiteral(5.0)),
            ),
        );

        parse_expr(
            "10 < 5",
            Expr::BinaryOp(
                Operator::LessThan,
                Box::new(Expr::NumberLiteral(10.0)),
                Box::new(Expr::NumberLiteral(5.0)),
            ),
        );

        parse_expr(
            "10 <= 5",
            Expr::BinaryOp(
                Operator::LessEquals,
                Box::new(Expr::NumberLiteral(10.0)),
                Box::new(Expr::NumberLiteral(5.0)),
            ),
        );
    }

    #[test]
    fn addition() {
        parse_expr(
            "1 + 1",
            Expr::BinaryOp(
                Operator::Add,
                Box::new(Expr::NumberLiteral(1.0)),
                Box::new(Expr::NumberLiteral(1.0)),
            ),
        );
    }

    #[test]
    fn subtraction() {
        parse_expr(
            "1 - 1",
            Expr::BinaryOp(
                Operator::Subtract,
                Box::new(Expr::NumberLiteral(1.0)),
                Box::new(Expr::NumberLiteral(1.0)),
            ),
        );
    }

    #[test]
    fn multiplication() {
        parse_expr(
            "1 * 1",
            Expr::BinaryOp(
                Operator::Multiply,
                Box::new(Expr::NumberLiteral(1.0)),
                Box::new(Expr::NumberLiteral(1.0)),
            ),
        );
    }

    #[test]
    fn division() {
        parse_expr(
            "1 / 1",
            Expr::BinaryOp(
                Operator::Divide,
                Box::new(Expr::NumberLiteral(1.0)),
                Box::new(Expr::NumberLiteral(1.0)),
            ),
        );
    }

    #[test]
    fn unary() {
        parse_expr(
            "!true",
            Expr::UnaryOp(Operator::Not, Box::new(Expr::BooleanLiteral(true))),
        );

        parse_expr(
            "-true",
            Expr::UnaryOp(Operator::UnaryMinus, Box::new(Expr::BooleanLiteral(true))),
        );
    }

    #[test]
    fn precedence() {
        // TODO: Might be worth adding test cases for the other ops
        parse_expr(
            "x = -1 * 2 + 3 > 4 != 5",
            Expr::Assign(
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
            ),
        );
    }

    #[test]
    fn grouping() {
        parse_expr(
            "2 * (1 + 2) * 3",
            Expr::BinaryOp(
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
            ),
        );
    }

    #[test]
    fn if_expr() {
        parse_expr(
            "if x > 10 { print true; }",
            Expr::If(
                Box::new(Expr::BinaryOp(
                    Operator::GreaterThan,
                    Box::new(Expr::Identifier("x".to_string())),
                    Box::new(Expr::NumberLiteral(10.0)),
                )),
                vec![Stmt::Print(Expr::BooleanLiteral(true))],
                vec![],
            ),
        );

        parse_expr(
            "if x > 10 { print true; } else { print false; }",
            Expr::If(
                Box::new(Expr::BinaryOp(
                    Operator::GreaterThan,
                    Box::new(Expr::Identifier("x".to_string())),
                    Box::new(Expr::NumberLiteral(10.0)),
                )),
                vec![Stmt::Print(Expr::BooleanLiteral(true))],
                vec![Stmt::Print(Expr::BooleanLiteral(false))],
            ),
        );
    }

    #[test]
    fn declaration() {
        parse_stmt(
            "let x = 10;",
            vec![
                Stmt::Declaration("x".to_string(), Expr::NumberLiteral(10.0)),
            ],
        );
    }

    #[test]
    fn program() {
        parse_stmt(
            "\nx;\ny;\n\nz;\n",
            vec![
                Stmt::ExprStmt(Expr::Identifier("x".to_string())),
                Stmt::ExprStmt(Expr::Identifier("y".to_string())),
                Stmt::ExprStmt(Expr::Identifier("z".to_string())),
            ],
        );
    }
}
