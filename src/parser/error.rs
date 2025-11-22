use std::{fmt, error, num};

use crate::{
    lexer::Error as LexerError,
    span::*,
    token::*,
};

#[derive(Debug, Clone)]
pub enum ErrorKind {
    Lexer(LexerError),
    UnexpectedToken(UnexpectedTokenError),
    ExpectExpression(UnexpectedTokenError),
    ExpectTerminal(UnexpectedTokenError),
    ExpectStatement(UnexpectedTokenError),
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
        write!(f, "Parse error")?;

        if let Some(span) = self.span {
            write!(f, " at {}:{}", span.position.line, span.position.column)?;
        }

        write!(f, ": ")?;

        match &self.kind {
            ErrorKind::Lexer(err) => write!(f, "{err}"),
            ErrorKind::UnexpectedToken(err) => write!(f, "{err}"),
            ErrorKind::ParseInt(err) => write!(f, "{err}"),
            ErrorKind::ParseString(err) => write!(f, "{err}"),
            ErrorKind::ParseFloat(err) => write!(f, "{err}"),
            ErrorKind::ExpectExpression(err) => write!(f, "expect expression, got {}", err.token),
            ErrorKind::ExpectTerminal(err) => write!(f, "expect terminal, got {}", err.token),
            ErrorKind::ExpectStatement(err) => write!(f, "expect statement, got {}", err.token),
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

impl From<UnexpectedTokenError> for Error {
    fn from(value: UnexpectedTokenError) -> Self {
        Self::new(ErrorKind::UnexpectedToken(value), None)
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

#[derive(Debug, Clone)]
pub struct UnexpectedTokenError {
    pub token: Token,
    pub expected: Vec<TokenType>,
}

impl fmt::Display for UnexpectedTokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unexpected token {}", self.token)?;

        if self.expected.len() > 0 {
            let token_types_string = self.expected
                .iter()
                .map(|tt| tt.to_string())
                .collect::<Vec<String>>()
                .join(", ");

            write!(f, ", expected one of {}", token_types_string)?;
        }

        Ok(())
    }
}

impl error::Error for UnexpectedTokenError {}
