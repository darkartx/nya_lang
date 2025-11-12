pub mod token_type;

pub use token_type::TokenType;

// Literal value
#[derive(Debug, Clone, Default, PartialEq)]
pub enum LiteralValue {
    #[default]
    None,
    IntNumber(i64),
    FloatNumber(f64),
    Str(String),
    Bool(bool),
    Null,
}

impl LiteralValue {
    pub fn new_str(value: &str) -> Self {
        Self::Str(value.into())
    }
}

// Position
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub index: usize,
    pub line: usize,
    pub column: usize,
}

impl Default for Position {
    fn default() -> Self {
        Self { index: 0, line: 1, column: 0 }
    }
}

impl Position {
    pub fn new(index: usize, line: usize, column: usize) -> Self {
        Self {
            index,
            line,
            column
        }
    }
}

// Token
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal_value: LiteralValue,
    pub position: Option<Position>
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal_value: LiteralValue,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal_value,
            position: None
        }
    }

    pub fn new_with_position(
        token_type: TokenType,
        lexeme: String,
        literal_value: LiteralValue,
        position: Position
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal_value,
            position: Some(position)
        }
    }
}
