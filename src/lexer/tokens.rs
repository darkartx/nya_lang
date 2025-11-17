use std::str;

use crate::token::*;

use super::error::*;

#[derive(Debug)]
pub struct Tokens<'a> {
    input: str::Chars<'a>,
    index: usize,
    lexeme: String,
    next_char: Option<char>,
    line: usize,
    column: usize,
    token_position: Pos,
}

impl<'a> Tokens<'a> {
    pub(super) fn new(input: &'a String) -> Self {
        let mut input = input.chars();
        let index = 0;
        let lexeme = Default::default();
        let next_char = input.next();
        
        Self {
            input,
            index,
            lexeme,
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

        self.lexeme.clear();
        self.token_position = self.current_position();

        self.read_keyword_or_identifier()
    }

    fn current_position(&self) -> Pos {
        Pos::new(self.index, self.line, self.column)
    }

    // fn token_position(&self) -> Position {
    //     self.token_position
    // } 

    fn make_token(&mut self, token_type: TokenType) -> Token {
        let lexeme = self.lexeme.clone();
        self.lexeme = Default::default();
        Token::new_with_span(token_type, lexeme, Span::new(self.token_position, self.index - self.token_position.index))
    }

    // fn make_token_current_position(&self, token_type: TokenType, lexeme: String, literal_value: LiteralValue) -> Token {
    //     Token::new_with_position(token_type, lexeme, literal_value, self.current_position())
    // }

    fn make_error(&self, error_kind: ErrorKind) -> Error {
        Error::new(error_kind, self.token_position)
    }

    fn advance(&mut self) {
        match self.next_char {
            Some(ch) if is_new_line_char(ch) => {
                self.line += 1;
                self.column = 1;
            }
            Some(_) => {
                self.column += 1;
            }
            _ => return
        }

        self.index += 1;
        self.lexeme.push(self.next_char.unwrap());
        self.next_char = self.input.next();
    }

    fn read_keyword_or_identifier(&mut self) -> Result<Token, Error> {
        match self.next_char {
            Some(ch) if is_idetifier_start_char(ch) => self.advance(),
            _ => { return self.read_number() },
        }

        loop {
            match self.next_char {
                Some(ch) if is_idetifier_char(ch) => self.advance(),
                _ => break,
            }
        }

        let token_type = match self.lexeme.as_str() {
            "let" => TokenType::Let,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "null" => TokenType::Null,
            _ => TokenType::Identifier,
        };

        Ok(self.make_token(token_type))
    }

    fn read_number(&mut self) -> Result<Token, Error> {
        match self.next_char {
            Some('0') => {
                self.advance();
                match self.next_char {
                    _ => self.read_decimal_number()
                }
            },
            Some(ch) if is_number_start_char(ch) => {
                self.advance();
                self.read_decimal_number()
            },
            _ => self.read_string(),
        }
    }

    fn read_decimal_number(&mut self) -> Result<Token, Error> {
        loop {
            match self.next_char {
                Some(ch) if is_number_char(ch) => self.advance(),
                Some('.' | 'e') => return self.read_float_number(),
                _ => break,
            }
        }

        Ok(self.make_token(TokenType::IntNumber))
    }

    fn read_float_number(&mut self) -> Result<Token, Error> {
        loop {
            match self.next_char {
                Some(ch) if is_number_char(ch) => self.advance(),
                Some('.') => {
                    self.advance();
                    break;
                }
                _ => break,
            }
        }

        loop {
            match self.next_char {
                Some(ch) if is_number_char(ch) => self.advance(),
                Some('e') => {
                    self.advance();
                    break;
                }
                _ => break,
            }
        }

        loop {
            match self.next_char {
                Some(ch) if is_number_char(ch) => self.advance(),
                _ => break,
            }
        }

        Ok(self.make_token(TokenType::FloatNumber))
    }

    fn read_string(&mut self) -> Result<Token, Error> {
        match self.next_char {
            Some('"') => self.advance(),
            _ => return self.read_operator(),
        }

        loop {
            match self.next_char {
                Some('\\') => {
                    self.advance();
                    self.advance();
                }
                Some('"') => {
                    self.advance();
                    break;
                },
                Some(_) => self.advance(),
                None => return Err(self.make_error(ErrorKind::UnexpectedEof(UnexpectedEofError)))
            }
        }

        Ok(self.make_token(TokenType::String))
    }

    fn read_operator(&mut self) -> Result<Token, Error> {
        if self.lexeme.len() == 0 {
            self.advance();
        }

        let token_type = match self.lexeme.as_str() {
            "=" => TokenType::Assign,
            ";" => TokenType::Semicolon,
            "+" => TokenType::Plus,
            "*" => TokenType::Mult,
            "(" => TokenType::Lparen,
            ")" => TokenType::Rparen,
            "," => TokenType::Comma,
            "" => TokenType::Eof,
            _ => {
                return Err(self.make_error(ErrorKind::UnexpectedChar(
                    UnexpectedCharError(self.lexeme.chars().next().unwrap())
                )))
            }
        };

        Ok(self.make_token(token_type))
    }

    fn skip_whitespaces(&mut self) {
        loop {
            match self.next_char {
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
    ch.is_ascii_digit() || ch == '_'
}

#[inline]
fn is_whitespace_char(ch: char) -> bool {
    ch.is_whitespace()
}

#[inline]
fn is_new_line_char(ch: char) -> bool {
    ch == '\n'
}
