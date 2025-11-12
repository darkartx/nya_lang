pub mod tokens;
pub mod error;

#[cfg(test)]
mod tests;

pub use tokens::Tokens;

#[derive(Debug)]
pub struct Lexer {
    input: String
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    pub fn tokens(&self) -> Tokens<'_> {
        Tokens::new(&self.input)
    }

    pub fn input_ref(&self) -> &str {
        &self.input
    }
}

