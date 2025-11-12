use std::{iter, num, str};

use crate::token::*;

use super::error::*;

#[derive(Debug)]
pub struct Tokens<'a> {
    input: str::Chars<'a>,
    index: usize,
    current_char: Option<char>,
    next_char: Option<char>,
    line: usize,
    column: usize,
    token_position: Position,
}

impl iter::Iterator for Tokens<'_> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_token())
    }
}

impl<'a> Tokens<'a> {
    pub(super) fn new(input: &'a String) -> Self {
        let mut input = input.chars();
        let index = 0;
        let current_char = input.next();
        let next_char = input.next();
        
        Self {
            input,
            index,
            current_char,
            next_char,
            line: 1,
            column: 1,
            token_position: Default::default(),
        }
    }
}

impl Tokens<'_> {
    pub fn next_token(&mut self) -> Result<Token, Error> {
        self.skip_whitespaces();

        self.token_position = self.current_position();

        match self.current_char {
            Some(ch) if is_idetifier_start_char(ch) => self.read_identifier(),
            Some(ch) if is_number_start_char(ch) => self.read_number(),
            Some(ch) if is_string_start_end_char(ch) => self.read_string(),
            None => Ok(self.make_token(TokenType::Eof, "".to_string(), LiteralValue::None)),
            _ => self.read_token(),
        }
    }

    fn current_position(&self) -> Position {
        Position::new(self.index, self.line, self.column)
    }

    // fn token_position(&self) -> Position {
    //     self.token_position
    // } 

    fn make_token(&self, token_type: TokenType, lexeme: String, literal_value: LiteralValue) -> Token {
        Token::new_with_position(token_type, lexeme, literal_value, self.token_position)
    }

    // fn make_token_current_position(&self, token_type: TokenType, lexeme: String, literal_value: LiteralValue) -> Token {
    //     Token::new_with_position(token_type, lexeme, literal_value, self.current_position())
    // }

    fn make_error(&self, error_kind: ErrorKind) -> Error {
        Error::new(error_kind, self.token_position)
    }

    fn advance(&mut self) {
        match self.current_char {
            Some(ch) if is_new_line_char(ch) => {
                self.line += 1;
                self.column = 1;
            }
            Some(_) => {
                self.column += 1;
            }
            _ => {}
        }

        self.index += 1;
        self.current_char = self.next_char;
        self.next_char = self.input.next();
    }

    fn read_identifier(&mut self) -> Result<Token, Error> {
        let mut lexeme = self.current_char.unwrap().to_string();

        loop {
            self.advance();

            match self.current_char {
                Some(ch) if is_idetifier_char(ch) => lexeme.push(ch),
                _ => break
            }
        }

        let token_type = match lexeme.as_str() {
            "let" => TokenType::Let,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "null" => TokenType::Null,
            _ => TokenType::Identifier
        };

        let literal_value = match token_type {
            TokenType::True => LiteralValue::Bool(true),
            TokenType::False => LiteralValue::Bool(false),
            TokenType::Null => LiteralValue::Null,
            TokenType::Identifier => LiteralValue::new_str(&lexeme),
            _ => LiteralValue::None
        };

        Ok(self.make_token(token_type, lexeme, literal_value))
    }

    fn read_token(&mut self) -> Result<Token, Error> {
        let lexeme = self.current_char.unwrap().to_string();
        self.advance();

        let token_type = match lexeme.as_str() {
            "=" => TokenType::Assign,
            ";" => TokenType::Semicolon,
            "+" => TokenType::Plus,
            "*" => TokenType::Mult,
            "(" => TokenType::Lparen,
            ")" => TokenType::Rparen,
            "{" => TokenType::LBrace,
            "}" => TokenType::Rbrace,
            "," => TokenType::Comma,
            ch @ _ => {
                return Err(self.make_error(
                    ErrorKind::UnexpectedChar(
                        UnexpectedCharError { 
                            current: ch.chars().next().unwrap(),
                            expected: None
                        }
                    )
                ))
            }
        };

        Ok(self.make_token(token_type, lexeme, LiteralValue::None))
    }

    // TODO: extend number parsing
    fn read_number(&mut self) -> Result<Token, Error> {
        let mut lexeme = self.current_char.unwrap().to_string();
        let mut is_float = false;

        loop {
            self.advance();

            match self.current_char {
                Some(ch) if is_number_char(ch) => lexeme.push(ch),
                Some('.') if !is_float => {
                    lexeme.push('.');
                    is_float = true;
                } 
                _ => break,
            };
        }

        if is_float {
            let value = match parse_float(&lexeme) {
                Ok(value) => value,
                Err(err) => {
                    return Err(self.make_error(
                        ErrorKind::FloatParse(err)
                    ))
                }
            };

            let token_type = TokenType::FloatNumber;
            let literal_value = LiteralValue::FloatNumber(value);
            return Ok(self.make_token(token_type, lexeme, literal_value));
        }

        
        let value = match parse_integer(&lexeme) {
            Ok(value) => value,
            Err(err) => {
                return Err(self.make_error(
                    ErrorKind::IntParse(err)
                ))
            }
        };
        let token_type = TokenType::IntNumber;
        let literal_value = LiteralValue::IntNumber(value);
        Ok(self.make_token(token_type, lexeme, literal_value))
    }

    // TODO: Extend strings
    fn read_string(&mut self) -> Result<Token, Error> {
        let mut lexeme = self.current_char.unwrap().to_string();
        let mut value = String::new();
        let mut is_escaping = false;

        loop {
            self.advance();

            match self.current_char {
                Some(ch) if !is_escaping && is_string_start_end_char(ch) => {
                    lexeme.push(ch);
                    self.advance();
                    break;
                }
                Some(ch) if !is_escaping && is_escape_char(ch) => {
                    lexeme.push(ch);
                    is_escaping = true;
                }
                Some(ch) => {
                    let value_ch = if is_escaping {
                        let escaped_ch = escape_char(ch);

                        if escaped_ch.is_none() {
                            return Err(self.make_error(
                                ErrorKind::InvalidEscape(InvalidEscapeError::UknownCharacterEscape)
                            ))
                        }

                        is_escaping = false;
                        escaped_ch.unwrap()
                    } else {
                        ch
                    };

                    lexeme.push(ch);
                    value.push(value_ch);
                }
                None => {
                    return Err(self.make_error(
                        ErrorKind::UnexpectedEof(
                            UnexpectedEofError { expected: Some('"') }
                        )
                    ))
                }
            }
        }

        let token_type = TokenType::String;
        let literal_value = LiteralValue::Str(value);

        Ok(self.make_token(token_type, lexeme, literal_value))
    }

    fn skip_whitespaces(&mut self) {
        loop {
            match self.current_char {
                Some(ch) if is_whitespace_char(ch) => {},
                _ => break
            }

            self.advance();
        }
    }
}

#[inline]
fn is_idetifier_start_char(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

#[inline]
fn is_idetifier_char(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
}

#[inline]
fn is_number_start_char(ch: char) -> bool {
    ch.is_ascii_digit()
}

#[inline]
fn is_number_char(ch: char) -> bool {
    ch.is_ascii_digit()
}

#[inline]
fn is_whitespace_char(ch: char) -> bool {
    ch.is_whitespace()
}

#[inline]
fn is_new_line_char(ch: char) -> bool {
    ch == '\n'
}

#[inline]
fn is_string_start_end_char(ch: char) -> bool {
    ch == '"'
}

#[inline]
fn is_escape_char(ch: char) -> bool {
    ch == '\\'
}

fn escape_char(ch: char) -> Option<char> {
    match ch {
        ch @ ('"' | '\'' | '\\') => Some(ch),
        'n' => Some('\n'),
        't' => Some('\t'),
        'r' => Some('\r'),
        '0' => Some('\0'),
        _ => None,
    }
}

fn parse_float(lexeme: &str) -> Result<f64, num::ParseFloatError> {
    lexeme.parse()
}

fn parse_integer(lexeme: &str) -> Result<i64, num::ParseIntError> {
    lexeme.parse()
}
