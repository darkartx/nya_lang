pub mod error;
mod tokens_parser;

#[cfg(test)]
mod tests;

use crate::{
    ast::Ast, lexer::Lexer,
};

pub use error::Error;
use tokens_parser::TokensParser;

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self { lexer }
    }

    pub fn parse(&self) -> Result<Ast, Error> {
        TokensParser::new(self.lexer.tokens()).parse()
    }
}
