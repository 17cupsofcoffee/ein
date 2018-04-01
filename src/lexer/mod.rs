pub mod tokens;

use std::str::CharIndices;
use self::tokens::Token;

#[inline]
fn is_id_start(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphabetic()
}

#[inline]
fn is_id_continue(ch: char) -> bool {
    ch == '_' || ch.is_ascii_digit()
}

pub type SpanResult<'input> = Result<(usize, Token<'input>, usize), String>;

pub struct Lexer<'input> {
    source: &'input str,
    chars: CharIndices<'input>,
    lookahead: Option<(usize, char)>,
    lookahead2: Option<(usize, char)>,
}

impl<'input> Lexer<'input> {
    pub fn new(source: &'input str) -> Lexer<'input> {
        let mut chars = source.char_indices();
        let lookahead = chars.next();
        let lookahead2 = chars.next();

        Lexer {
            source,
            chars,
            lookahead,
            lookahead2,
        }
    }

    fn bump(&mut self) -> Option<(usize, char)> {
        let next = self.lookahead;
        self.lookahead = self.lookahead2;
        self.lookahead2 = self.chars.next();
        next
    }

    fn take_until<F>(&mut self, mut terminate: F) -> Option<usize>
    where
        F: FnMut(char) -> bool,
    {
        while let Some((i, ch)) = self.lookahead {
            if terminate(ch) {
                return Some(i);
            } else {
                self.bump();
            }
        }

        None
    }

    fn take_while<F>(&mut self, mut condition: F) -> Option<usize>
    where
        F: FnMut(char) -> bool,
    {
        self.take_until(|ch| !condition(ch))
    }

    fn skip_to_line_end(&mut self) {
        self.take_while(|ch| ch != '\n');
    }

    fn skip_whitespace(&mut self) {
        self.take_while(|ch| ch.is_whitespace());
    }

    fn read_string(&mut self, pos: usize) -> SpanResult<'input> {
        match self.take_until(|ch| ch == '"') {
            Some(i) => {
                self.bump();
                Ok((pos, Token::String(&self.source[pos + 1..i]), i + 1))
            }
            None => Err(format!("Unterminated string")),
        }
    }

    fn read_number(&mut self, pos: usize) -> SpanResult<'input> {
        let mut end = self.take_while(|ch| ch.is_ascii_digit());

        if let Some((_, '.')) = self.lookahead {
            // Check if it's a decimal or a field access
            if let Some((_, next_ch)) = self.lookahead2 {
                if next_ch.is_ascii_digit() {
                    self.bump();
                    end = self.take_while(|ch| ch.is_ascii_digit());
                }
            }
        }

        let end = end.unwrap_or(self.source.len());

        Ok((
            pos,
            Token::Number(self.source[pos..end].parse().expect("unparsable number")),
            end,
        ))
    }

    fn read_identifier(&mut self, pos: usize) -> SpanResult<'input> {
        let end = self.take_while(|ch| is_id_start(ch) || is_id_continue(ch))
            .unwrap_or(self.source.len());

        match &self.source[pos..end] {
            "and" => Ok((pos, Token::And, end)),
            "else" => Ok((pos, Token::Else, end)),
            "false" => Ok((pos, Token::False, end)),
            "fn" => Ok((pos, Token::Fn, end)),
            "for" => Ok((pos, Token::For, end)),
            "if" => Ok((pos, Token::If, end)),
            "nil" => Ok((pos, Token::Nil, end)),
            "or" => Ok((pos, Token::Or, end)),
            "print" => Ok((pos, Token::Print, end)),
            "return" => Ok((pos, Token::Return, end)),
            "this" => Ok((pos, Token::This, end)),
            "true" => Ok((pos, Token::True, end)),
            "let" => Ok((pos, Token::Let, end)),
            "while" => Ok((pos, Token::While, end)),
            id => Ok((pos, Token::Identifier(id), end)),
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = SpanResult<'input>;

    fn next(&mut self) -> Option<SpanResult<'input>> {
        self.skip_whitespace();

        if let Some((i, ch)) = self.bump() {
            match ch {
                '{' => Some(Ok((i, Token::OpenBrace, i + 1))),
                '}' => Some(Ok((i, Token::CloseBrace, i + 1))),
                '(' => Some(Ok((i, Token::OpenParen, i + 1))),
                ')' => Some(Ok((i, Token::CloseParen, i + 1))),
                '[' => Some(Ok((i, Token::OpenBracket, i + 1))),
                ']' => Some(Ok((i, Token::CloseBracket, i + 1))),
                ';' => Some(Ok((i, Token::Semicolon, i + 1))),
                ',' => Some(Ok((i, Token::Comma, i + 1))),
                '.' => Some(Ok((i, Token::Dot, i + 1))),
                '+' => Some(Ok((i, Token::Plus, i + 1))),
                '-' => Some(Ok((i, Token::Minus, i + 1))),
                '*' => Some(Ok((i, Token::Star, i + 1))),

                '/' => {
                    if let Some((_, '/')) = self.lookahead {
                        self.skip_to_line_end();
                        self.next()
                    } else {
                        Some(Ok((i, Token::Slash, i + 1)))
                    }
                }

                '!' => {
                    if let Some((_, '=')) = self.lookahead {
                        self.bump();
                        Some(Ok((i, Token::NotEqual, i + 2)))
                    } else {
                        Some(Ok((i, Token::Not, i + 1)))
                    }
                }

                '=' => {
                    if let Some((_, '=')) = self.lookahead {
                        self.bump();
                        Some(Ok((i, Token::EqualEqual, i + 2)))
                    } else {
                        Some(Ok((i, Token::Equal, i + 1)))
                    }
                }

                '>' => {
                    if let Some((_, '=')) = self.lookahead {
                        self.bump();
                        Some(Ok((i, Token::GreaterEqual, i + 2)))
                    } else {
                        Some(Ok((i, Token::Greater, i + 1)))
                    }
                }

                '<' => {
                    if let Some((_, '=')) = self.lookahead {
                        self.bump();
                        Some(Ok((i, Token::LessEqual, i + 2)))
                    } else {
                        Some(Ok((i, Token::Less, i + 1)))
                    }
                }

                '"' => Some(self.read_string(i)),
                ch if is_id_start(ch) => Some(self.read_identifier(i)),
                ch if ch.is_ascii_digit() => Some(self.read_number(i)),

                ch => Some(Err(format!("Unexpected token: {}", ch))),
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn lex(source: &str, expected: Vec<(usize, Token, usize)>) {
        let mut lexer = Lexer::new(source);

        let mut actual_len = 0;
        let expected_len = expected.len();

        for (expected, actual) in expected.into_iter().zip(lexer.by_ref()) {
            actual_len += 1;
            let actual = actual.unwrap();
            assert_eq!(expected, actual);
        }

        assert_eq!(expected_len, actual_len);
        assert_eq!(None, lexer.next());
    }

    #[test]
    fn delimiters() {
        lex(
            "{} [] ()",
            vec![
                (0, Token::OpenBrace, 1),
                (1, Token::CloseBrace, 2),
                (3, Token::OpenBracket, 4),
                (4, Token::CloseBracket, 5),
                (6, Token::OpenParen, 7),
                (7, Token::CloseParen, 8),
            ],
        );
    }

    #[test]
    fn operators() {
        lex(
            ", . + - * / = == ! != > >= < <=",
            vec![
                (0, Token::Comma, 1),
                (2, Token::Dot, 3),
                (4, Token::Plus, 5),
                (6, Token::Minus, 7),
                (8, Token::Star, 9),
                (10, Token::Slash, 11),
                (12, Token::Equal, 13),
                (14, Token::EqualEqual, 16),
                (17, Token::Not, 18),
                (19, Token::NotEqual, 21),
                (22, Token::Greater, 23),
                (24, Token::GreaterEqual, 26),
                (27, Token::Less, 28),
                (29, Token::LessEqual, 31),
            ],
        );
    }

    #[test]
    fn line_comment() {
        lex(
            "123; // comment\n 123",
            vec![
                (0, Token::Number(123.0), 3),
                (3, Token::Semicolon, 4),
                (17, Token::Number(123.0), 20),
            ],
        );
    }

    #[test]
    fn string() {
        lex(
            "\"hello, world\"",
            vec![(0, Token::String("hello, world"), 14)],
        );
    }

    #[test]
    fn integer() {
        lex("123", vec![(0, Token::Number(123.0), 3)]);
    }

    #[test]
    fn decimal() {
        lex("123.45", vec![(0, Token::Number(123.45), 6)]);
    }

    #[test]
    fn number_field_access() {
        lex(
            "123.prop",
            vec![
                (0, Token::Number(123.0), 3),
                (3, Token::Dot, 4),
                (4, Token::Identifier("prop"), 8),
            ],
        );
    }

    #[test]
    fn identifiers() {
        lex("id", vec![(0, Token::Identifier("id"), 2)]);
        lex("_id", vec![(0, Token::Identifier("_id"), 3)]);
        lex("id123", vec![(0, Token::Identifier("id123"), 5)]);
    }

    #[test]
    fn keywords() {
        lex("and", vec![(0, Token::And, 3)]);
        lex("else", vec![(0, Token::Else, 4)]);
        lex("false", vec![(0, Token::False, 5)]);
        lex("fn", vec![(0, Token::Fn, 2)]);
        lex("for", vec![(0, Token::For, 3)]);
        lex("if", vec![(0, Token::If, 2)]);
        lex("nil", vec![(0, Token::Nil, 3)]);
        lex("or", vec![(0, Token::Or, 2)]);
        lex("print", vec![(0, Token::Print, 5)]);
        lex("return", vec![(0, Token::Return, 6)]);
        lex("this", vec![(0, Token::This, 4)]);
        lex("true", vec![(0, Token::True, 4)]);
        lex("let", vec![(0, Token::Let, 3)]);
        lex("while", vec![(0, Token::While, 5)]);
    }
}
