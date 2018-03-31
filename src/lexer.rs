use std::str::CharIndices;
use tokens::Token;
use itertools;
use itertools::MultiPeek;

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
    chars: MultiPeek<CharIndices<'input>>,
}

impl<'input> Lexer<'input> {
    pub fn new(source: &'input str) -> Lexer<'input> {
        Lexer {
            source,
            chars: itertools::multipeek(source.char_indices()),
        }
    }

    fn skip_to_line_end(&mut self) {
        while let Some(&(_, ch)) = self.chars.peek() {
            if ch != '\n' {
                self.chars.next();
            } else {
                break;
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&(_, ch)) = self.chars.peek() {
            if ch.is_whitespace() {
                self.chars.next();
            } else {
                break;
            }
        }
    }

    fn read_string(&mut self, pos: usize) -> SpanResult<'input> {
        let mut end = pos;

        while let Some((i, ch)) = self.chars.next() {
            if ch == '"' {
                return Ok((pos, Token::String(&self.source[pos + 1..end + 1]), end + 1));
            } else {
                end = i;
            }
        }

        Err(format!("Unterminated string"))
    }

    fn read_number(&mut self, pos: usize) -> SpanResult<'input> {
        let mut end = pos;
        let mut consumed_dot = false;

        while let Some(&(i, ch)) = self.chars.peek() {
            match ch {
                // If we encounter a dot, we need to do an extra character of
                // lookahead to check whether it's a decimal or a field
                // access
                // TODO: This code could almost certainly be cleaner
                '.' if !consumed_dot => match self.chars.peek() {
                    Some(&(_, next_ch)) if next_ch.is_ascii_digit() => {
                        end = i;
                        consumed_dot = true;
                        self.chars.next();
                    }
                    _ => {
                        break;
                    }
                },
                ch if ch.is_ascii_digit() => {
                    end = i;
                    self.chars.next();
                }
                _ => {
                    break;
                }
            }
        }

        Ok((
            pos,
            Token::Number(
                self.source[pos..end + 1]
                    .parse()
                    .expect("unparsable number"),
            ),
            end,
        ))
    }

    fn read_identifier(&mut self, pos: usize) -> SpanResult<'input> {
        let mut end = pos;

        while let Some(&(i, ch)) = self.chars.peek() {
            if is_id_start(ch) || is_id_continue(ch) {
                end = i;
                self.chars.next();
            } else {
                break;
            }
        }

        match &self.source[pos..end + 1] {
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
        while let Some((i, ch)) = self.chars.next() {
            return match ch {
                '{' => Some(Ok((i, Token::OpenBrace, i + 1))),
                '}' => Some(Ok((i, Token::CloseBrace, i + 1))),
                '(' => Some(Ok((i, Token::OpenParen, i + 1))),
                ')' => Some(Ok((i, Token::CloseParen, i + 1))),
                '[' => Some(Ok((i, Token::OpenBracket, i + 1))),
                ']' => Some(Ok((i, Token::CloseBracket, i + 1))),
                ',' => Some(Ok((i, Token::Comma, i + 1))),
                '.' => Some(Ok((i, Token::Dot, i + 1))),
                '+' => Some(Ok((i, Token::Plus, i + 1))),
                '-' => Some(Ok((i, Token::Minus, i + 1))),
                '*' => Some(Ok((i, Token::Star, i + 1))),

                '/' => {
                    if let Some(&(_, '/')) = self.chars.peek() {
                        self.skip_to_line_end();
                        continue;
                    } else {
                        Some(Ok((i, Token::Slash, i + 1)))
                    }
                }

                '!' => {
                    if let Some(&(_, '=')) = self.chars.peek() {
                        self.chars.next();
                        Some(Ok((i, Token::NotEqual, i + 2)))
                    } else {
                        Some(Ok((i, Token::Not, i + 1)))
                    }
                }

                '=' => {
                    if let Some(&(_, '=')) = self.chars.peek() {
                        self.chars.next();
                        Some(Ok((i, Token::EqualEqual, i + 2)))
                    } else {
                        Some(Ok((i, Token::Equal, i + 1)))
                    }
                }

                '>' => {
                    if let Some(&(_, '=')) = self.chars.peek() {
                        self.chars.next();
                        Some(Ok((i, Token::GreaterEqual, i + 2)))
                    } else {
                        Some(Ok((i, Token::Greater, i + 1)))
                    }
                }

                '<' => {
                    if let Some(&(_, '=')) = self.chars.peek() {
                        self.chars.next();
                        Some(Ok((i, Token::LessEqual, i + 2)))
                    } else {
                        Some(Ok((i, Token::Less, i + 1)))
                    }
                }

                '"' => Some(self.read_string(i)),

                '\n' => {
                    self.skip_whitespace();
                    Some(Ok((i, Token::NewLine, i + 1)))
                }

                ch if is_id_start(ch) => Some(self.read_identifier(i)),
                ch if ch.is_ascii_digit() => Some(self.read_number(i)),
                ch if ch.is_whitespace() => continue,

                ch => Some(Err(format!("Unexpected token: {}", ch))),
            };
        }

        None
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
            "123 // comment\n 123",
            vec![
                (0, Token::Number(123.0), 2),
                (14, Token::NewLine, 15),
                (16, Token::Number(123.0), 18),
            ],
        );
    }

    #[test]
    fn string() {
        lex(
            "\"hello, world\"",
            vec![(0, Token::String("hello, world"), 13)],
        );
    }

    #[test]
    fn integer() {
        lex("123", vec![(0, Token::Number(123.0), 2)]);
    }

    #[test]
    fn decimal() {
        lex("123.45", vec![(0, Token::Number(123.45), 5)]);
    }

    #[test]
    fn number_field_access() {
        lex(
            "123.prop",
            vec![
                (0, Token::Number(123.0), 2),
                (3, Token::Dot, 4),
                (4, Token::Identifier("prop"), 7),
            ],
        );
    }

    #[test]
    fn identifiers() {
        lex("id", vec![(0, Token::Identifier("id"), 1)]);
        lex("_id", vec![(0, Token::Identifier("_id"), 2)]);
        lex("id123", vec![(0, Token::Identifier("id123"), 4)]);
    }

    #[test]
    fn keywords() {
        lex("and", vec![(0, Token::And, 2)]);
        lex("else", vec![(0, Token::Else, 3)]);
        lex("false", vec![(0, Token::False, 4)]);
        lex("fn", vec![(0, Token::Fn, 1)]);
        lex("for", vec![(0, Token::For, 2)]);
        lex("if", vec![(0, Token::If, 1)]);
        lex("nil", vec![(0, Token::Nil, 2)]);
        lex("or", vec![(0, Token::Or, 1)]);
        lex("print", vec![(0, Token::Print, 4)]);
        lex("return", vec![(0, Token::Return, 5)]);
        lex("this", vec![(0, Token::This, 3)]);
        lex("true", vec![(0, Token::True, 3)]);
        lex("let", vec![(0, Token::Let, 2)]);
        lex("while", vec![(0, Token::While, 4)]);
    }
}
