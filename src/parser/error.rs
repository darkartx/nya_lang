use std::{fmt, error, num};

use crate::{
    lexer::Error as LexerError,
    span::*,
    token::Token,
};

#[derive(Debug, Clone)]
pub enum ErrorKind {
    Lexer(LexerError),
    UnexpectedToken(Token),
    ParseInt(num::ParseIntError),
    ParseString(ParseStringError),
    ParseFloat(num::ParseFloatError),
}

#[derive(Debug, Clone)]
pub struct Error {
    pub kind: ErrorKind,
    pub span: Option<Span>
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
            ErrorKind::ParseString(err) => write!(f, "{err}"),
            ErrorKind::ParseFloat(err) => write!(f, "{err}"),
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

impl From<ParseStringError> for Error {
    fn from(value: ParseStringError) -> Self {
        Self::new(ErrorKind::ParseString(value), None)
    }
}

impl From<num::ParseFloatError> for Error {
    fn from(value: num::ParseFloatError) -> Self {
        Self::new(ErrorKind::ParseFloat(value), None)
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

#[derive(Debug, Clone)]
pub enum ParseStringError {
    UnexpectedEnding,
    UnexpectedChar(char),
    ParseInt(num::ParseIntError),
    UndefinedUnicode(u32),
}

impl fmt::Display for ParseStringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ParseStringError::*;

        match self {
            UnexpectedEnding => write!(f, "unexpected string ending"),
            UnexpectedChar(ch) => write!(f, "unexpected char \'{ch}\'"),
            ParseInt(err) => write!(f, "{err}"),
            UndefinedUnicode(u) => write!(f, "undefined unicode: {u}"),
        }
    }
}

impl From<num::ParseIntError> for ParseStringError {
    fn from(value: num::ParseIntError) -> Self {
        Self::ParseInt(value)
    }
}

impl error::Error for ParseStringError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            ParseStringError::ParseInt(err) => Some(err),
            _ => None,
        }
    }
}
