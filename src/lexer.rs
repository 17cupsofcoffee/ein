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

    fn skip_whitespace(&mut self) {
        while let Some(&(_, ch)) = self.chars.peek() {
            if ch.is_whitespace() {
                self.chars.next();
            } else {
                break;
            }
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
                ch if is_id_start(ch) => Some(self.read_identifier(i)),
                ch if ch.is_ascii_digit() => Some(self.read_number(i)),

                '\n' => {
                    self.skip_whitespace();
                    Some(Ok(Token::NewLine))
                }

                ch if ch.is_whitespace() => continue,

                ch => Some(Err(format!("Unexpected token: {}", ch))),
            };
        }

        None
    }
}
