use std::{error, fmt};

use crate::token::{Pos};

#[derive(Debug)]
pub enum ErrorKind {
    UnexpectedEof(UnexpectedEofError),
    UnexpectedChar(UnexpectedCharError),
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub position: Pos,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Lexer error at {}:{}: ", self.position.line, self.position.column)
            .and_then(|_| {
                match &self.kind {
                    ErrorKind::UnexpectedEof(err) => write!(f, "{err}"),
                    ErrorKind::UnexpectedChar(err) => write!(f, "{err}"),
                }
            })
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.kind {
            ErrorKind::UnexpectedChar(err) => Some(err),
            ErrorKind::UnexpectedEof(err) => Some(err),
        }
    }
}

impl Error {
    pub(super) fn new(kind: ErrorKind, position: Pos) -> Self {
        Self {
            kind,
            position
        }
    }
}

#[derive(Debug)]
pub struct UnexpectedCharError(pub char);

impl fmt::Display for UnexpectedCharError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unexpected char \'{}\'", self.0)
    }
}

impl error::Error for UnexpectedCharError {}

#[derive(Debug)]
pub struct UnexpectedEofError;

impl error::Error for UnexpectedEofError {}

impl fmt::Display for UnexpectedEofError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unexpected EOF")
    }
}
