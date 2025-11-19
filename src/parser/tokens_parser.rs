use std::mem;

use crate::{
    ast, lexer::Tokens, token::*
};

use super::error::*;

type BoxStatement = Box<dyn ast::Statement>;
type BoxExpression = Box<dyn ast::Expression>;

#[derive(Debug)]
pub(super) struct TokensParser<'a> {
    tokens: Tokens<'a>,
    current_token: Token,
    peek_token: Token,
}

impl<'a> TokensParser<'a> {
    pub(super) fn new(tokens: Tokens<'a>) -> Self {
        Self {
            tokens,
            current_token: Default::default(),
            peek_token: Default::default(),
        }
    }
}

impl TokensParser<'_> {
    pub(super) fn parse(mut self) -> Result<ast::Ast, Error> {
        self.advance()?;
        self.advance()?;
        self.parse_program()
    }

    fn parse_program(&mut self) -> Result<ast::Ast, Error> {
        let mut statements = vec![];

        loop {
            if self.current_token_type() == TokenType::Eof {
                break;
            }

            statements.push(
                self.parse_statement()?
            );
        }

        Ok(
            ast::Ast::new(statements)
        )
    }

    fn parse_statement(&mut self) -> Result<BoxStatement, Error> {
        match self.current_token_type() {
            TokenType::Let => self.parse_let_statement(),
            _ => Err(self.make_error(ErrorKind::UnexpectedToken(self.current_token.clone())))
        }
    }

    fn parse_let_statement(&mut self) -> Result<BoxStatement, Error> {
        self.advance()?;

        let identifier = self.parse_idetifier()?;
        
        let expression = if self.current_token_type_is(&[TokenType::Assign]) {
            self.advance()?;
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.skip_terminal()?;

        Ok(Box::new(ast::Let::new(identifier, expression)))
    }

    fn parse_expression(&mut self) -> Result<BoxExpression, Error> {
        match self.current_token_type() {
            TokenType::IntNumber => self.parse_int_number(),
            _ => Err(self.make_error(ErrorKind::UnexpectedToken(self.current_token.clone())))
        }
    }

    fn parse_int_number(&mut self) -> Result<BoxExpression, Error> {
        let token = self.advance()?;
        let value: i64 = self.handle_result(token.lexeme.parse())?;

        Ok(Box::new(ast::Value::Int(value)))
    }

    fn handle_result<T, E: Into<Error>>(&self, result: Result<T, E>) -> Result<T, Error> {
        result.map_err(|err| err.into().with_span(self.current_token.span))
    }

    fn skip_terminal(&mut self) -> Result<(), Error> {
        if self.current_token_type_is(&[
            TokenType::Semicolon,
            TokenType::Eof,
            TokenType::NewLine,
        ]) {
            self.advance()?;
        }

        Ok(())
    }

    fn current_token_type_is(&self, token_types: &[TokenType]) -> bool {
        token_types.iter().any(|tt| *tt == self.current_token.token_type)
    }

    fn parse_idetifier(&mut self) -> Result<ast::Identifier, Error> {
        match self.current_token_type() {
            TokenType::Identifier => {
                let token = self.advance()?;
                Ok(ast::Identifier::new(token.lexeme))
            },
            _ => Err(self.make_error(ErrorKind::UnexpectedToken(self.current_token.clone())))
        }
    }

    fn make_error(&self, kind: ErrorKind) -> Error {
        Error::new(kind, self.current_token.span)
    }

    fn current_token_type(&self) -> TokenType {
        self.current_token.token_type
    }

    fn advance(&mut self) -> Result<Token, Error> {
        let next_token = self.tokens.next_token()?;
        let current_token = mem::replace(&mut self.peek_token, next_token);
        let result = mem::replace(&mut self.current_token, current_token);

        Ok(result)
    }
}