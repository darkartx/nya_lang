use std::{fmt, error, num};

use crate::{
    lexer::Error as LexerError,
    span::*,
    token::Token,
};

#[derive(Debug, Clone)]
pub(super) enum ErrorKind {
    Lexer(LexerError),
    UnexpectedToken(Token),
    ParseInt(num::ParseIntError),
}

#[derive(Debug, Clone)]
pub struct Error {
    kind: ErrorKind,
    span: Option<Span>
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse error ")?;

        if let Some(span) = self.span {
            write!(f, "at {}:{}", span.position.line, span.position.column)?;
        }

        write!(f, ":")?;

        match &self.kind {
            ErrorKind::Lexer(err) => write!(f, "{err}"),
            ErrorKind::UnexpectedToken(token) => write!(f, "unexpected token {}", token),
            ErrorKind::ParseInt(err) => write!(f, "{err}"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.kind {
            ErrorKind::Lexer(err) => Some(err),
            _ => None,
        }
    }
}

impl From<LexerError> for Error {
    fn from(value: LexerError) -> Self {
        let span = value.span;
        Self::new(ErrorKind::Lexer(value), Some(span))
    }
}

impl From<num::ParseIntError> for Error {
    fn from(value: num::ParseIntError) -> Self {
        Self::new(ErrorKind::ParseInt(value), None)
    }
}

impl Error {
    pub(super) fn new(kind: ErrorKind, span: Option<Span>) -> Self {
        Self { kind, span }
    }

    pub(super) fn with_span(mut self, span: Option<Span>) -> Self {
        self.span = span;
        self
    }
}
