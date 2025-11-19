use std::mem;

use crate::{
    ast, lexer::Tokens, token::*, span::*
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
            TokenType::IntNumber => self.parse_int_literal(),
            TokenType::String => self.parse_string_literal(),
            TokenType::True | TokenType::False => self.parse_bool_literal(),
            TokenType::FloatNumber => self.parse_float_literal(),
            _ => Err(self.make_error(ErrorKind::UnexpectedToken(self.current_token.clone())))
        }
    }

    fn parse_int_literal(&mut self) -> Result<BoxExpression, Error> {
        let token = self.advance()?;
        let value: i64 = self.handle_result(token.lexeme.parse(), token.span)?;

        Ok(Box::new(ast::Literal::Int(value)))
    }

    fn parse_string_literal(&mut self) -> Result<BoxExpression, Error> {
        let token = self.advance()?;
        let value = &token.lexeme[1..token.lexeme.len() - 1];
        let value = self.handle_result(unescape_string(value), token.span)?;

        Ok(Box::new(ast::Literal::Str(value)))
    }

    fn parse_bool_literal(&mut self) -> Result<BoxExpression, Error> {
        let token = self.advance()?;
        let value = match token.token_type {
            TokenType::True => true,
            TokenType::False => false,
            _ => unreachable!()
        };

        Ok(Box::new(ast::Literal::Bool(value)))
    }

    fn parse_float_literal(&mut self) -> Result<BoxExpression, Error> {
        let token = self.advance()?;
        let value: f64 = self.handle_result(token.lexeme.parse(), token.span)?;

        Ok(Box::new(ast::Literal::Float(value)))
    }

    fn handle_result<T, E: Into<Error>>(&self, result: Result<T, E>, span: Option<Span>) -> Result<T, Error> {
        result.map_err(|err| err.into().with_span(span))
    }

    fn skip_terminal(&mut self) -> Result<(), Error> {
        self.skip_tokens(&[
            TokenType::Semicolon,
            TokenType::Eof,
            TokenType::NewLine,
        ])
    }

    fn skip_tokens(&mut self, token_types: &[TokenType]) -> Result<(), Error> {
        if self.current_token_type_is(token_types) {
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

fn unescape_string(value: &str) -> Result<String, ParseStringError> {
    let mut result = String::new();
    let mut chars = value.chars();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('n') => result.push('\n'),
                Some('t') => result.push('\t'),
                Some('r') => result.push('\r'),
                Some('\\') => result.push('\\'),
                Some('"') => result.push('"'),
                Some('x') => {
                    let h1 = chars.next().ok_or(ParseStringError::UnexpectedEnding)?;
                    let h2 = chars.next().ok_or(ParseStringError::UnexpectedEnding)?;
                    let byte = u8::from_str_radix(&format!("{h1}{h2}"), 16)?;
                    result.push(byte as char);
                }
                Some('u') => {
                    match chars.next() {
                        Some('{') => {},
                        Some(ch) => return Err(ParseStringError::UnexpectedChar(ch)),
                        None => return Err(ParseStringError::UnexpectedEnding),
                    }

                    let mut hex = String::new();
                    loop {
                        match chars.next() {
                            Some('}') => break,
                            Some(ch) => hex.push(ch),
                            None => return Err(ParseStringError::UnexpectedEnding)
                        }
                    }

                    let code = u32::from_str_radix(&hex, 16)?;
                    let ch = char::from_u32(code).ok_or(ParseStringError::UndefinedUnicode(code))?;

                    result.push(ch);
                }
                other => {
                    result.push('\\');
                    if let Some(o) = other {
                        result.push(o);
                    }
                }
            }
        } else {
            result.push(ch);
        }
    }

    Ok(result)
}