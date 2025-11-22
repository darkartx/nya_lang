use std::{str, mem};

use crate::{
    token::*,
    span::*,
};

use super::error::*;

#[derive(Debug)]
pub struct Tokens<'a> {
    input: str::Chars<'a>,
    index: usize,
    lexeme: String,
    current_char: Option<char>,
    next_char: Option<char>,
    line: usize,
    column: usize,
    token_position: Pos,
}

impl<'a> Tokens<'a> {
    pub(super) fn new(input: &'a String) -> Self {
        let mut input = input.chars();
        let index = 0;
        let lexeme: String = Default::default();
        let current_char = input.next();
        let next_char = input.next();
        
        Self {
            input,
            index,
            lexeme,
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

        self.lexeme.clear();
        self.token_position = self.current_position();

        self.read_keyword_or_identifier()
    }

    fn current_position(&self) -> Pos {
        Pos::new(self.index, self.line, self.column)
    }

    fn token_span(&self) -> Span {
        Span::new(self.token_position, self.index - self.token_position.index)
    }

    fn make_token(&mut self, token_type: TokenType) -> Token {
        let lexeme = mem::take(&mut self.lexeme);
        Token::new_with_span(token_type, lexeme, self.token_span())
    }

    fn make_error(&self, error_kind: ErrorKind) -> Error {
        Error::new(error_kind, self.token_span())
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
            _ => return
        }

        self.index += 1;
        self.lexeme.push(self.current_char.unwrap());
        self.current_char = self.next_char;
        self.next_char = self.input.next();
    }

    fn read_keyword_or_identifier(&mut self) -> Result<Token, Error> {
        match self.current_char {
            Some(ch) if is_idetifier_start_char(ch) => self.advance(),
            _ => { return self.read_number() },
        }

        loop {
            match self.current_char {
                Some(ch) if is_idetifier_char(ch) => self.advance(),
                _ => break,
            }
        }

        let token_type = match self.lexeme.as_str() {
            "let" => TokenType::Let,
            "const" => TokenType::Const,
            "fn" => TokenType::Fn,
            "async" => TokenType::Async,
            "await" => TokenType::Await,
            "return" => TokenType::Return,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "for" => TokenType::For,
            "while" => TokenType::While,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            "class" => TokenType::Class,
            "static" => TokenType::Static,
            "import" => TokenType::Import,
            "from" => TokenType::From,
            "export" => TokenType::Export,
            "try" => TokenType::Try,
            "catch" => TokenType::Catch,
            "finally" => TokenType::Finally,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "null" => TokenType::Null,
            _ => TokenType::Identifier,
        };

        Ok(self.make_token(token_type))
    }

    fn read_number(&mut self) -> Result<Token, Error> {
        match self.current_char {
            Some('0') => {
                self.advance();
                match self.current_char {
                    Some('x') => {
                        self.advance();
                        self.read_hexdecimal_number()
                    }
                    Some('b') => {
                        self.advance();
                        self.read_bindecimal_number()
                    }
                    Some('o') => {
                        self.advance();
                        self.read_octdecimal_number()
                    }
                    _ => self.read_decimal_float_number(),
                }
            },
            Some(ch) if is_number_start_char(ch) => {
                self.advance();
                self.read_decimal_float_number()
            },
            _ => self.read_string(),
        }
    }

    fn read_decimal_float_number(&mut self) -> Result<Token, Error> {
        loop {
            match self.current_char {
                Some(ch) if is_number_char(ch) => self.advance(),
                Some('.') => {
                    match self.next_char {
                        Some(ch) if is_idetifier_start_char(ch) => break,
                        Some('.') => break,
                        _ => return self.read_float_number(),
                    }
                }
                Some('e' | 'E') => return self.read_float_number(),
                _ => break,
            }
        }

        self.read_decimal_number()
    }

    fn read_decimal_number(&mut self) -> Result<Token, Error> {
        loop {
            match self.current_char {
                Some(ch) if is_number_char(ch) => self.advance(),
                _ => break,
            }
        }

        Ok(self.make_token(TokenType::IntNumber))
    }

    fn read_hexdecimal_number(&mut self) -> Result<Token, Error> {
        loop {
            match self.current_char {
                Some(ch) if is_hexdecimal_char(ch) => self.advance(),
                _ => break,
            }
        }

        Ok(self.make_token(TokenType::IntNumber))
    }

    fn read_octdecimal_number(&mut self) -> Result<Token, Error> {
        loop {
            match self.current_char {
                Some(ch) if is_octdecimal_char(ch) => self.advance(),
                _ => break,
            }
        }

        Ok(self.make_token(TokenType::IntNumber))
    }

    fn read_bindecimal_number(&mut self) -> Result<Token, Error> {
        loop {
            match self.current_char {
                Some(ch) if is_bindecimal_char(ch) => self.advance(),
                _ => break,
            }
        }

        Ok(self.make_token(TokenType::IntNumber))
    }

    fn read_float_number(&mut self) -> Result<Token, Error> {
        loop {
            match self.current_char {
                Some(ch) if is_number_char(ch) => self.advance(),
                Some('.') => {
                    self.advance();
                    break;
                }
                _ => break,
            }
        }

        loop {
            match self.current_char {
                Some(ch) if is_number_char(ch) => self.advance(),
                Some('e' | 'E') => {
                    self.advance();

                    match self.current_char {
                        Some('+' | '-') => self.advance(),
                        _ => {}
                    }

                    break;
                }
                _ => break,
            }
        }

        loop {
            match self.current_char {
                Some(ch) if is_number_char(ch) => self.advance(),
                _ => break,
            }
        }

        Ok(self.make_token(TokenType::FloatNumber))
    }

    fn read_string(&mut self) -> Result<Token, Error> {
        match self.current_char {
            Some('"') => self.advance(),
            _ => return self.read_sign(),
        }

        loop {
            match self.current_char {
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

    fn read_sign(&mut self) -> Result<Token, Error> {
        let token_type = match (self.current_char, self.next_char) {
            (Some('.'), Some('.')) => self.advance_twice_and_return_tt(TokenType::Range),
            (Some('.'), _) => self.advance_and_return_tt(TokenType::Dot),
            (Some('='), Some('=')) => self.advance_twice_and_return_tt(TokenType::Eq),
            (Some('='), _) => self.advance_and_return_tt(TokenType::Assign),
            (Some('!'), Some('=')) => self.advance_twice_and_return_tt(TokenType::Neq),
            (Some('!'), _) => self.advance_and_return_tt(TokenType::Not),
            (Some('&'), Some('&')) => self.advance_twice_and_return_tt(TokenType::And),
            (Some('&'), Some('=')) => self.advance_twice_and_return_tt(TokenType::AssignBitAnd),
            (Some('&'), _) => self.advance_and_return_tt(TokenType::BitAnd),
            (Some('|'), Some('|')) => self.advance_twice_and_return_tt(TokenType::Or),
            (Some('|'), Some('=')) => self.advance_twice_and_return_tt(TokenType::AssignBitOr),
            (Some('|'), _) => self.advance_and_return_tt(TokenType::BitOr),
            (Some('<'), _) => {
                self.advance();
                match (self.current_char, self.next_char) {
                    (Some('='), _) => self.advance_and_return_tt(TokenType::Lte),
                    (Some('<'), Some('=')) => self.advance_twice_and_return_tt(TokenType::AssignShiftLeft),
                    (Some('<'), _) => self.advance_and_return_tt(TokenType::ShiftLeft),
                    _ => TokenType::Lt
                }
            }
            (Some('>'), _) => {
                self.advance();
                match (self.current_char, self.next_char) {
                    (Some('='), _) => self.advance_and_return_tt(TokenType::Gte),
                    (Some('>'), Some('=')) => self.advance_twice_and_return_tt(TokenType::AssignShiftRight),
                    (Some('>'), _) => self.advance_and_return_tt(TokenType::ShiftRight),
                    _ => TokenType::Gt
                }
            }
            (Some('+'), Some('=')) => self.advance_twice_and_return_tt(TokenType::AssignPlus),
            (Some('+'), _) => self.advance_and_return_tt(TokenType::Plus),
            (Some('-'), Some('=')) => self.advance_twice_and_return_tt(TokenType::AssignMinus),
            (Some('-'), _) => self.advance_and_return_tt(TokenType::Minus),
            (Some('*'), Some('=')) => self.advance_twice_and_return_tt(TokenType::AssignMult),
            (Some('*'), _) => self.advance_and_return_tt(TokenType::Mult),
            (Some('/'), Some('=')) => self.advance_twice_and_return_tt(TokenType::AssignDiv),
            (Some('/'), Some('/')) => return self.read_singleline_comment(),
            (Some('/'), _) => self.advance_and_return_tt(TokenType::Div),
            (Some('%'), Some('=')) => self.advance_twice_and_return_tt(TokenType::AssignMod),
            (Some('%'), _) => self.advance_and_return_tt(TokenType::Mod),
            (Some('^'), Some('=')) => self.advance_twice_and_return_tt(TokenType::AssignBitXor),
            (Some('^'), _) => self.advance_and_return_tt(TokenType::BitXor),
            (Some('~'), _) => self.advance_and_return_tt(TokenType::BitNot),
            (Some(';'), _) => self.advance_and_return_tt(TokenType::Semicolon),
            (Some(','), _) => self.advance_and_return_tt(TokenType::Comma),
            (Some('('), _) => self.advance_and_return_tt(TokenType::Lparen),
            (Some(')'), _) => self.advance_and_return_tt(TokenType::Rparen),
            (Some('{'), _) => self.advance_and_return_tt(TokenType::Lbrace),
            (Some('}'), _) => self.advance_and_return_tt(TokenType::Rbrace),
            (None, _) => self.advance_and_return_tt(TokenType::Eof),
            (Some(ch), _) => {
                return Err(self.make_error(ErrorKind::UnexpectedChar(
                    UnexpectedCharError(ch)
                )))
            }
        };

        Ok(self.make_token(token_type))
    }

    fn read_singleline_comment(&mut self) -> Result<Token, Error> {
        loop {
            match self.current_char {
                Some('\n') => break,
                None => break,
                _ => self.advance(),
            }
        }

        Ok(self.make_token(TokenType::SingleLineComment))
    }

    #[inline]
    fn advance_twice_and_return_tt(&mut self, token_type: TokenType) -> TokenType {
        self.advance();
        self.advance();
        token_type
    }

    #[inline]
    fn advance_and_return_tt(&mut self, token_type: TokenType) -> TokenType {
        self.advance();
        token_type
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

#[inline]
fn is_hexdecimal_char(ch: char) -> bool {
    ch.is_ascii_digit() || matches!(ch, 'a'..='f' | 'A'..='F' | '_')
}

#[inline]
fn is_bindecimal_char(ch: char) -> bool {
    matches!(ch, '0' | '1' | '_')
}

#[inline]
fn is_octdecimal_char(ch: char) -> bool {
    matches!(ch, '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '_')
}
