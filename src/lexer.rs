use std::str::CharIndices;
use tokens::{DelimToken, Token};
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

    fn read_string(&mut self, position: usize) -> Result<Token<'input>, String> {
        let mut end = position;

        while let Some((i, ch)) = self.chars.next() {
            if ch == '"' {
                return Ok(Token::String(&self.source[position + 1..end + 1]));
            } else {
                end = i;
            }
        }

        Err(format!("Unterminated string"))
    }

    fn read_number(&mut self, position: usize) -> Result<Token<'input>, String> {
        let mut end = position;
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

        Ok(Token::Number(
            self.source[position..end + 1]
                .parse()
                .expect("unparsable number"),
        ))
    }

    fn read_identifier(&mut self, position: usize) -> Result<Token<'input>, String> {
        let mut end = position;

        while let Some(&(i, ch)) = self.chars.peek() {
            if is_id_start(ch) || is_id_continue(ch) {
                end = i;
                self.chars.next();
            } else {
                break;
            }
        }

        match &self.source[position..end + 1] {
            "and" => Ok(Token::And),
            "else" => Ok(Token::Else),
            "false" => Ok(Token::False),
            "fn" => Ok(Token::Fn),
            "for" => Ok(Token::For),
            "if" => Ok(Token::If),
            "nil" => Ok(Token::Nil),
            "or" => Ok(Token::Or),
            "print" => Ok(Token::Print),
            "return" => Ok(Token::Return),
            "this" => Ok(Token::This),
            "true" => Ok(Token::True),
            "let" => Ok(Token::Let),
            "while" => Ok(Token::While),
            id => Ok(Token::Identifier(id)),
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Result<Token<'input>, String>;

    fn next(&mut self) -> Option<Result<Token<'input>, String>> {
        while let Some((i, ch)) = self.chars.next() {
            return match ch {
                '{' => Some(Ok(Token::OpenDelim(DelimToken::Brace))),
                '}' => Some(Ok(Token::CloseDelim(DelimToken::Brace))),
                '(' => Some(Ok(Token::OpenDelim(DelimToken::Paren))),
                ')' => Some(Ok(Token::CloseDelim(DelimToken::Paren))),
                '[' => Some(Ok(Token::OpenDelim(DelimToken::Bracket))),
                ']' => Some(Ok(Token::CloseDelim(DelimToken::Bracket))),
                ',' => Some(Ok(Token::Comma)),
                '.' => Some(Ok(Token::Dot)),
                '+' => Some(Ok(Token::Plus)),
                '-' => Some(Ok(Token::Minus)),
                '*' => Some(Ok(Token::Star)),

                '/' => {
                    if let Some(&(_, '/')) = self.chars.peek() {
                        self.skip_to_line_end();
                        continue;
                    } else {
                        Some(Ok(Token::Slash))
                    }
                }

                '!' => {
                    if let Some(&(_, '=')) = self.chars.peek() {
                        self.chars.next();
                        Some(Ok(Token::NotEqual))
                    } else {
                        Some(Ok(Token::Not))
                    }
                }

                '=' => {
                    if let Some(&(_, '=')) = self.chars.peek() {
                        self.chars.next();
                        Some(Ok(Token::EqualEqual))
                    } else {
                        Some(Ok(Token::Equal))
                    }
                }

                '>' => {
                    if let Some(&(_, '=')) = self.chars.peek() {
                        self.chars.next();
                        Some(Ok(Token::GreaterEqual))
                    } else {
                        Some(Ok(Token::Greater))
                    }
                }

                '<' => {
                    if let Some(&(_, '=')) = self.chars.peek() {
                        self.chars.next();
                        Some(Ok(Token::LessEqual))
                    } else {
                        Some(Ok(Token::Less))
                    }
                }

                '"' => Some(self.read_string(i)),
                '\n' => Some(Ok(Token::NewLine)),

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

    fn lex(source: &str, expected: Vec<Token>) {
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
                Token::OpenDelim(DelimToken::Brace),
                Token::CloseDelim(DelimToken::Brace),
                Token::OpenDelim(DelimToken::Bracket),
                Token::CloseDelim(DelimToken::Bracket),
                Token::OpenDelim(DelimToken::Paren),
                Token::CloseDelim(DelimToken::Paren),
            ],
        );
    }

    #[test]
    fn operators() {
        lex(
            ", . + - * / = == ! != > >= < <=",
            vec![
                Token::Comma,
                Token::Dot,
                Token::Plus,
                Token::Minus,
                Token::Star,
                Token::Slash,
                Token::Equal,
                Token::EqualEqual,
                Token::Not,
                Token::NotEqual,
                Token::Greater,
                Token::GreaterEqual,
                Token::Less,
                Token::LessEqual,
            ],
        );
    }

    #[test]
    fn line_comment() {
        lex(
            "123 // comment\n 123",
            vec![Token::Number(123.0), Token::NewLine, Token::Number(123.0)],
        );
    }

    #[test]
    fn string() {
        lex("\"hello, world\"", vec![Token::String("hello, world")]);
    }

    #[test]
    fn integer() {
        lex("123", vec![Token::Number(123.0)]);
    }

    #[test]
    fn decimal() {
        lex("123.45", vec![Token::Number(123.45)]);
    }

    #[test]
    fn number_field_access() {
        lex(
            "123.prop",
            vec![Token::Number(123.0), Token::Dot, Token::Identifier("prop")],
        );
    }

    #[test]
    fn identifiers() {
        lex("id", vec![Token::Identifier("id")]);
        lex("_id", vec![Token::Identifier("_id")]);
        lex("id123", vec![Token::Identifier("id123")]);
    }

    #[test]
    fn keywords() {
        lex("and", vec![Token::And]);
        lex("else", vec![Token::Else]);
        lex("false", vec![Token::False]);
        lex("fn", vec![Token::Fn]);
        lex("for", vec![Token::For]);
        lex("if", vec![Token::If]);
        lex("nil", vec![Token::Nil]);
        lex("or", vec![Token::Or]);
        lex("print", vec![Token::Print]);
        lex("return", vec![Token::Return]);
        lex("this", vec![Token::This]);
        lex("true", vec![Token::True]);
        lex("let", vec![Token::Let]);
        lex("while", vec![Token::While]);
    }
}
