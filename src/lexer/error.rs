use std::{error, fmt, num};

use crate::token::{Position};

#[derive(Debug)]
pub enum ErrorKind {
    UnexpectedEof(UnexpectedEofError),
    UnexpectedChar(UnexpectedCharError),
    InvalidEscape(InvalidEscapeError),
    FloatParse(num::ParseFloatError),
    IntParse(num::ParseIntError),
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub position: Position,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Lexer error at {}:{}: ", self.position.line, self.position.column)
            .and_then(|_| {
                match &self.kind {
                    ErrorKind::UnexpectedEof(err) => write!(f, "{err}"),
                    ErrorKind::UnexpectedChar(err) => write!(f, "{err}"),
                    ErrorKind::InvalidEscape(err) => write!(f, "{err}"),
                    ErrorKind::FloatParse(err) => write!(f, "{err}"),
                    ErrorKind::IntParse(err) => write!(f, "{err}"),
                }
            })
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.kind {
            ErrorKind::UnexpectedChar(err) => Some(err),
            ErrorKind::UnexpectedEof(err) => Some(err),
            ErrorKind::InvalidEscape(err) => Some(err),
            ErrorKind::FloatParse(err) => Some(err),
            ErrorKind::IntParse(err) => Some(err),
            // _ => None,
        }
    }
}

impl Error {
    pub(super) fn new(kind: ErrorKind, position: Position) -> Self {
        Self {
            kind,
            position
        }
    }
}

#[derive(Debug)]
pub struct UnexpectedCharError {
    pub current: char,
    pub expected: Option<char>,
}

impl fmt::Display for UnexpectedCharError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unexpected char \'{}\'", self.current)
            .and_then(|r| {
                match self.expected {
                    Some(ch) => write!(f, ", expected \'{ch}\'"),
                    _ => Ok(r)
                }
            })
    }
}

impl error::Error for UnexpectedCharError {}

#[derive(Debug)]
pub struct UnexpectedEofError {
    pub expected: Option<char>,
}

impl error::Error for UnexpectedEofError {}

impl fmt::Display for UnexpectedEofError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unexpected EOF")
            .and_then(|r| {
                match self.expected {
                    Some(ch) => write!(f, ", expected \'{ch}\'"),
                    _ => Ok(r)
                }
            })
    }
}

#[derive(Debug)]
pub enum InvalidEscapeError {
    UknownCharacterEscape,
}

impl fmt::Display for InvalidEscapeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InvalidEscapeError::UknownCharacterEscape => write!(f, "invalid character escape"),
        }
    }
}

impl error::Error for InvalidEscapeError {}
