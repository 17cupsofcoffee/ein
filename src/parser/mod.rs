#[allow(unknown_lints)]
#[allow(clippy)]
mod grammar;

use lalrpop_util::ParseError as LParseError;
use ast::{Expr, Stmt};
use lexer::{Lexer, LexicalError, Location};
use lexer::tokens::Token;
use self::grammar::{ExprParser, ProgramParser};

pub type ParseError<'input> = LParseError<Location, Token<'input>, LexicalError>;

pub fn parse_program(input: &str) -> Result<Vec<Stmt>, ParseError> {
    let lexer = Lexer::new(input);
    let parser = ProgramParser::new();
    parser.parse(lexer)
}

pub fn parse_expr(input: &str) -> Result<Expr, ParseError> {
    let lexer = Lexer::new(input);
    let parser = ExprParser::new();
    parser.parse(lexer)
}

#[cfg(test)]
mod test {
    use super::*;
    use ast::{Expr, Operator, Stmt};
    use lexer::Lexer;

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
            Expr::BinaryOp(Operator::And, Box::new(Expr::Nil), Box::new(Expr::Nil)),
        );

        expr(
            "nil || nil",
            Expr::BinaryOp(Operator::Or, Box::new(Expr::Nil), Box::new(Expr::Nil)),
        );
    }

    #[test]
    fn equality() {
        expr(
            "nil == nil",
            Expr::BinaryOp(Operator::Equals, Box::new(Expr::Nil), Box::new(Expr::Nil)),
        );

        expr(
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
        expr(
            "10 > 5",
            Expr::BinaryOp(
                Operator::GreaterThan,
                Box::new(Expr::NumberLiteral(10.0)),
                Box::new(Expr::NumberLiteral(5.0)),
            ),
        );

        expr(
            "10 >= 5",
            Expr::BinaryOp(
                Operator::GreaterEquals,
                Box::new(Expr::NumberLiteral(10.0)),
                Box::new(Expr::NumberLiteral(5.0)),
            ),
        );

        expr(
            "10 < 5",
            Expr::BinaryOp(
                Operator::LessThan,
                Box::new(Expr::NumberLiteral(10.0)),
                Box::new(Expr::NumberLiteral(5.0)),
            ),
        );

        expr(
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
        expr(
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
        expr(
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
        expr(
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
        expr(
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
        expr(
            "!true",
            Expr::UnaryOp(Operator::Not, Box::new(Expr::BooleanLiteral(true))),
        );

        expr(
            "-true",
            Expr::UnaryOp(Operator::UnaryMinus, Box::new(Expr::BooleanLiteral(true))),
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

        expr(
            "nil || nil && nil",
            Expr::BinaryOp(
                Operator::Or,
                Box::new(Expr::Nil),
                Box::new(Expr::BinaryOp(
                    Operator::And,
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
    fn declaration() {
        stmt(
            "let x = 10;",
            vec![
                Stmt::Declaration("x".to_string(), Expr::NumberLiteral(10.0)),
            ],
        );
    }

    #[test]
    fn if_stmt() {
        stmt(
            "if x > 10 { print true; }",
            vec![
                Stmt::If(
                    Expr::BinaryOp(
                        Operator::GreaterThan,
                        Box::new(Expr::Identifier("x".to_string())),
                        Box::new(Expr::NumberLiteral(10.0)),
                    ),
                    vec![Stmt::Print(Expr::BooleanLiteral(true))],
                    vec![],
                ),
            ],
        );

        stmt(
            "if x > 10 { print true; } else { print false; }",
            vec![
                Stmt::If(
                    Expr::BinaryOp(
                        Operator::GreaterThan,
                        Box::new(Expr::Identifier("x".to_string())),
                        Box::new(Expr::NumberLiteral(10.0)),
                    ),
                    vec![Stmt::Print(Expr::BooleanLiteral(true))],
                    vec![Stmt::Print(Expr::BooleanLiteral(false))],
                ),
            ],
        );
    }

    #[test]
    fn program() {
        stmt(
            "\nprint x\nprint y\n\nprint z\n",
            vec![
                Stmt::Print(Expr::Identifier("x".to_string())),
                Stmt::Print(Expr::Identifier("y".to_string())),
                Stmt::Print(Expr::Identifier("z".to_string())),
            ],
        );
    }
}
