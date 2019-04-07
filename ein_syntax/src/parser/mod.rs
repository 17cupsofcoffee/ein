mod grammar;

use lalrpop_util::ParseError as LParseError;

use crate::ast::{Expr, Stmt};
use crate::lexer::tokens::Token;
use crate::lexer::{Lexer, LexicalError, Location};
use grammar::{ExprParser, ProgramParser};

pub type ParseError<'input> = LParseError<Location, Token<'input>, LexicalError>;

pub fn parse_program(input: &str) -> Result<Vec<Stmt>, ParseError<'_>> {
    let lexer = Lexer::new(input);
    let parser = ProgramParser::new();
    parser.parse(lexer)
}

pub fn parse_expr(input: &str) -> Result<Expr, ParseError<'_>> {
    let lexer = Lexer::new(input);
    let parser = ExprParser::new();
    parser.parse(lexer)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ast::{BinaryOp, Expr, Stmt, UnaryOp};
    use crate::lexer::Lexer;

    fn stmt(input: &str, expected: Vec<Stmt>) {
        let lexer = Lexer::new(input);
        let parser = ProgramParser::new();
        assert_eq!(expected, parser.parse(lexer).unwrap());
    }

    fn expr(input: &str, expected: Expr) {
        let lexer = Lexer::new(input);
        let parser = ExprParser::new();
        assert_eq!(expected, parser.parse(lexer).unwrap());
    }

    #[test]
    fn literals() {
        expr("nil", Expr::Nil);
        expr("true", Expr::BooleanLiteral(true));
        expr("false", Expr::BooleanLiteral(false));
        expr("123", Expr::NumberLiteral(123.0));
        expr("123.45", Expr::NumberLiteral(123.45));
        expr("\"string\"", Expr::StringLiteral("string".to_string()));
    }

    #[test]
    fn identifiers() {
        expr("id", Expr::Identifier("id".to_string()));
        expr("_id", Expr::Identifier("_id".to_string()));
        expr("id123", Expr::Identifier("id123".to_string()));
    }

    #[test]
    fn assignment() {
        expr(
            "x = 10",
            Expr::Assign("x".to_string(), Box::new(Expr::NumberLiteral(10.0))),
        );

        expr(
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
    fn logic() {
        expr(
            "nil && nil",
            Expr::BinaryOp(BinaryOp::And, Box::new(Expr::Nil), Box::new(Expr::Nil)),
        );

        expr(
            "nil || nil",
            Expr::BinaryOp(BinaryOp::Or, Box::new(Expr::Nil), Box::new(Expr::Nil)),
        );
    }

    #[test]
    fn equality() {
        expr(
            "nil == nil",
            Expr::BinaryOp(BinaryOp::Equals, Box::new(Expr::Nil), Box::new(Expr::Nil)),
        );

        expr(
            "nil != nil",
            Expr::BinaryOp(
                BinaryOp::NotEquals,
                Box::new(Expr::Nil),
                Box::new(Expr::Nil),
            ),
        );
    }

    #[test]
    fn comparison() {
        expr(
            "10 > 5",
            Expr::BinaryOp(
                BinaryOp::GreaterThan,
                Box::new(Expr::NumberLiteral(10.0)),
                Box::new(Expr::NumberLiteral(5.0)),
            ),
        );

        expr(
            "10 >= 5",
            Expr::BinaryOp(
                BinaryOp::GreaterEquals,
                Box::new(Expr::NumberLiteral(10.0)),
                Box::new(Expr::NumberLiteral(5.0)),
            ),
        );

        expr(
            "10 < 5",
            Expr::BinaryOp(
                BinaryOp::LessThan,
                Box::new(Expr::NumberLiteral(10.0)),
                Box::new(Expr::NumberLiteral(5.0)),
            ),
        );

        expr(
            "10 <= 5",
            Expr::BinaryOp(
                BinaryOp::LessEquals,
                Box::new(Expr::NumberLiteral(10.0)),
                Box::new(Expr::NumberLiteral(5.0)),
            ),
        );
    }

    #[test]
    fn addition() {
        expr(
            "1 + 1",
            Expr::BinaryOp(
                BinaryOp::Add,
                Box::new(Expr::NumberLiteral(1.0)),
                Box::new(Expr::NumberLiteral(1.0)),
            ),
        );
    }

    #[test]
    fn subtraction() {
        expr(
            "1 - 1",
            Expr::BinaryOp(
                BinaryOp::Subtract,
                Box::new(Expr::NumberLiteral(1.0)),
                Box::new(Expr::NumberLiteral(1.0)),
            ),
        );
    }

    #[test]
    fn multiplication() {
        expr(
            "1 * 1",
            Expr::BinaryOp(
                BinaryOp::Multiply,
                Box::new(Expr::NumberLiteral(1.0)),
                Box::new(Expr::NumberLiteral(1.0)),
            ),
        );
    }

    #[test]
    fn division() {
        expr(
            "1 / 1",
            Expr::BinaryOp(
                BinaryOp::Divide,
                Box::new(Expr::NumberLiteral(1.0)),
                Box::new(Expr::NumberLiteral(1.0)),
            ),
        );
    }

    #[test]
    fn unary() {
        expr(
            "!true",
            Expr::UnaryOp(UnaryOp::Not, Box::new(Expr::BooleanLiteral(true))),
        );

        expr(
            "-true",
            Expr::UnaryOp(UnaryOp::UnaryMinus, Box::new(Expr::BooleanLiteral(true))),
        );
    }

    #[test]
    fn precedence() {
        // TODO: Might be worth adding test cases for the other ops
        expr(
            "x = -1 * 2 + 3 > 4 != 5",
            Expr::Assign(
                "x".to_string(),
                Box::new(Expr::BinaryOp(
                    BinaryOp::NotEquals,
                    Box::new(Expr::BinaryOp(
                        BinaryOp::GreaterThan,
                        Box::new(Expr::BinaryOp(
                            BinaryOp::Add,
                            Box::new(Expr::BinaryOp(
                                BinaryOp::Multiply,
                                Box::new(Expr::UnaryOp(
                                    UnaryOp::UnaryMinus,
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

        expr(
            "nil || nil && nil",
            Expr::BinaryOp(
                BinaryOp::Or,
                Box::new(Expr::Nil),
                Box::new(Expr::BinaryOp(
                    BinaryOp::And,
                    Box::new(Expr::Nil),
                    Box::new(Expr::Nil),
                )),
            ),
        );
    }

    #[test]
    fn grouping() {
        expr(
            "2 * (1 + 2) * 3",
            Expr::BinaryOp(
                BinaryOp::Multiply,
                Box::new(Expr::BinaryOp(
                    BinaryOp::Multiply,
                    Box::new(Expr::NumberLiteral(2.0)),
                    Box::new(Expr::BinaryOp(
                        BinaryOp::Add,
                        Box::new(Expr::NumberLiteral(1.0)),
                        Box::new(Expr::NumberLiteral(2.0)),
                    )),
                )),
                Box::new(Expr::NumberLiteral(3.0)),
            ),
        );
    }

    #[test]
    fn call() {
        expr(
            "id(1, x)",
            Expr::Call(
                Box::new(Expr::Identifier("id".to_string())),
                vec![Expr::NumberLiteral(1.0), Expr::Identifier("x".to_string())],
            ),
        );
    }

    #[test]
    fn declaration() {
        stmt(
            "let x = 10;",
            vec![Stmt::Declaration(
                "x".to_string(),
                Expr::NumberLiteral(10.0),
            )],
        );
    }

    #[test]
    fn function_declaration() {
        stmt(
            "fn test(a, b) {\nreturn a + b;\n}",
            vec![Stmt::Declaration(
                "test".to_string(),
                Expr::Function(
                    vec!["a".to_string(), "b".to_string()],
                    vec![Stmt::Return(Expr::BinaryOp(
                        BinaryOp::Add,
                        Box::new(Expr::Identifier("a".to_string())),
                        Box::new(Expr::Identifier("b".to_string())),
                    ))],
                ),
            )],
        );
    }

    #[test]
    fn function_return() {
        stmt(
            "fn test(a, b) {\nreturn a + b;\n}",
            vec![Stmt::Declaration(
                "test".to_string(),
                Expr::Function(
                    vec!["a".to_string(), "b".to_string()],
                    vec![Stmt::Return(Expr::BinaryOp(
                        BinaryOp::Add,
                        Box::new(Expr::Identifier("a".to_string())),
                        Box::new(Expr::Identifier("b".to_string())),
                    ))],
                ),
            )],
        );
    }

    #[test]
    fn if_stmt() {
        stmt(
            "if x > 10 { return true; }",
            vec![Stmt::If(
                Expr::BinaryOp(
                    BinaryOp::GreaterThan,
                    Box::new(Expr::Identifier("x".to_string())),
                    Box::new(Expr::NumberLiteral(10.0)),
                ),
                vec![Stmt::Return(Expr::BooleanLiteral(true))],
                vec![],
            )],
        );

        stmt(
            "if x > 10 { return true; } else { return false; }",
            vec![Stmt::If(
                Expr::BinaryOp(
                    BinaryOp::GreaterThan,
                    Box::new(Expr::Identifier("x".to_string())),
                    Box::new(Expr::NumberLiteral(10.0)),
                ),
                vec![Stmt::Return(Expr::BooleanLiteral(true))],
                vec![Stmt::Return(Expr::BooleanLiteral(false))],
            )],
        );
    }

    #[test]
    fn while_stmt() {
        stmt(
            "while true { return 123; }",
            vec![Stmt::While(
                Expr::BooleanLiteral(true),
                vec![Stmt::Return(Expr::NumberLiteral(123.0))],
            )],
        )
    }

    #[test]
    fn block_stmt() {
        stmt(
            "{ 1 + 1; return 123; }",
            vec![Stmt::Block(vec![
                Stmt::ExprStmt(Expr::BinaryOp(
                    BinaryOp::Add,
                    Box::new(Expr::NumberLiteral(1.0)),
                    Box::new(Expr::NumberLiteral(1.0)),
                )),
                Stmt::Return(Expr::NumberLiteral(123.0)),
            ])],
        )
    }

    #[test]
    fn program() {
        stmt(
            "\nlet x = 1;\nlet y = 2;\n\nlet z = 3;\n",
            vec![
                Stmt::Declaration("x".to_string(), Expr::NumberLiteral(1.0)),
                Stmt::Declaration("y".to_string(), Expr::NumberLiteral(2.0)),
                Stmt::Declaration("z".to_string(), Expr::NumberLiteral(3.0)),
            ],
        );
    }
}
