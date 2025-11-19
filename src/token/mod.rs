pub mod token_type;

use std::fmt;
use crate::span::*;

pub use token_type::TokenType;

// Token
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub span: Option<Span>
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.token_type, self.lexeme)
    }
}

impl Default for Token {
    fn default() -> Self {
        Self { token_type: TokenType::Eof, lexeme: Default::default(), span: None }
    }
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            span: None
        }
    }

    pub fn new_with_span(
        token_type: TokenType,
        lexeme: String,
        span: Span
    ) -> Self {
        Self {
            token_type,
            lexeme,
            span: Some(span)
        }
    }
}
