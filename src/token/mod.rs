pub mod token_type;

pub use token_type::TokenType;

// Pos
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pos {
    pub index: usize,
    pub line: usize,
    pub column: usize,
}

impl Default for Pos {
    fn default() -> Self {
        Self { index: 0, line: 1, column: 0 }
    }
}

impl Pos {
    pub fn new(index: usize, line: usize, column: usize) -> Self {
        Self {
            index,
            line,
            column
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: Pos,
    pub length: usize,
}

impl Span {
    pub fn new(start: Pos, length: usize) -> Self {
        Self {
            start,
            length,
        }
    }
}

// Token
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub span: Option<Span>
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
