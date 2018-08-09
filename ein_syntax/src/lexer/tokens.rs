use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Token<'input> {
    // Sigils
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    OpenParen,
    CloseParen,
    Semicolon,
    Comma,
    Dot,
    Plus,
    Minus,
    Star,
    Slash,
    Not,
    NotEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    AmpAmp,
    PipePipe,

    // Literals
    Identifier(&'input str),
    String(&'input str),
    Number(f64),

    // Keywords
    Else,
    False,
    Fn,
    For,
    If,
    Nil,
    Return,
    This,
    True,
    Let,
    While,
}

impl<'input> fmt::Display for Token<'input> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Token::*;

        match *self {
            OpenBrace => write!(f, "{{"),
            CloseBrace => write!(f, "}}"),
            OpenBracket => write!(f, "["),
            CloseBracket => write!(f, "]"),
            OpenParen => write!(f, "("),
            CloseParen => write!(f, ")"),
            Semicolon => write!(f, ";"),
            Comma => write!(f, ","),
            Dot => write!(f, "."),
            Plus => write!(f, "+"),
            Minus => write!(f, "-"),
            Star => write!(f, "*"),
            Slash => write!(f, "/"),
            Not => write!(f, "!"),
            NotEqual => write!(f, "!="),
            Equal => write!(f, "="),
            EqualEqual => write!(f, "=="),
            Greater => write!(f, ">"),
            GreaterEqual => write!(f, ">="),
            Less => write!(f, "<"),
            LessEqual => write!(f, "<="),
            AmpAmp => write!(f, "&&"),
            PipePipe => write!(f, "||"),

            Identifier(i) => write!(f, "{}", i),
            String(s) => write!(f, "\"{}\"", s),
            Number(n) => write!(f, "{}", n),

            Else => write!(f, "else"),
            False => write!(f, "false"),
            Fn => write!(f, "fn"),
            For => write!(f, "for"),
            If => write!(f, "if"),
            Nil => write!(f, "nil"),
            Return => write!(f, "return"),
            This => write!(f, "this"),
            True => write!(f, "true"),
            Let => write!(f, "let"),
            While => write!(f, "while"),
        }
    }
}
