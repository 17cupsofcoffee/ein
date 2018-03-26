#[derive(Debug)]
pub enum Token<'input> {
    // Sigils
    OpenDelim(DelimToken),
    CloseDelim(DelimToken),
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
    NewLine,

    // Literals
    Identifier(&'input str),
    String(&'input str),
    Number(f64),

    // Keywords
    And,
    Else,
    False,
    Fn,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    This,
    True,
    Let,
    While,
}

#[derive(Debug)]
pub enum DelimToken {
    Paren,
    Bracket,
    Brace,
}
