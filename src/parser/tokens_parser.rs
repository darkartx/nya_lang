use std::mem;

use crate::{
    ast, lexer::Tokens, span::*, token::*
};

use super::error::*;

type TT = TokenType;
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
            self.skip_newlines()?;

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
            TT::Let => self.parse_let_statement(),
            _ => Err(make_error(
                UnexpectedTokenError {
                    current: self.current_token.clone(),
                    expected: vec![TT::Let]
                },
                self.current_token.span
            )),
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
        self.parse_assigment()
    }

    fn parse_assigment(&mut self) -> Result<BoxExpression, Error> {
        self.parse_logic_or()
    }

    fn parse_logic_or(&mut self) -> Result<BoxExpression, Error> {
        self.parse_logic_and()
    }

    fn parse_logic_and(&mut self) -> Result<BoxExpression, Error> {
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> Result<BoxExpression, Error> {
        self.parse_comparsion()
    }

    fn parse_comparsion(&mut self) -> Result<BoxExpression, Error> {
        self.parse_term()
    }

    fn parse_term(&mut self) -> Result<BoxExpression, Error> {
        let mut result = self.parse_factor()?;

        self.skip_newlines()?;

        if self.current_token_type_is(&[TT::Plus, TT::Minus]) {
            let token = self.advance()?;
            let right = self.parse_factor()?;
            let op = match token.token_type {
                TT::Plus => ast::BinaryOp::Plus,
                TT::Minus => ast::BinaryOp::Minus,
                _ => unreachable!(),
            };

            result = Box::new(ast::Binary{ left: result, op, right })
        }

        Ok(result)
    }

    fn parse_factor(&mut self) -> Result<BoxExpression, Error> {
        let mut result = self.parse_unary()?;

        self.skip_newlines()?;

        if self.current_token_type_is(&[TT::Mult, TT::Div, TT::Mod]) {
            let token = self.advance()?;
            let right = self.parse_unary()?;
            let op = match token.token_type {
                TT::Mult => ast::BinaryOp::Mult,
                TT::Div => ast::BinaryOp::Div,
                TT::Mod => ast::BinaryOp::Mod,
                _ => unreachable!(),
            };

            result = Box::new(ast::Binary{ left: result, op, right })
        }

        Ok(result)
    }

    fn parse_unary(&mut self) -> Result<BoxExpression, Error> {
        let unary_token = if self.current_token_type_is(&[TT::Minus, TT::Not]) {
            Some(self.advance()?)
        } else {
            None
        };

        let mut result = self.parse_primary()?;

        if let Some(unary_token) = unary_token {
            let op = match unary_token.token_type {
                TT::Minus => ast::UnaryOp::Minus,
                TT::Not => ast::UnaryOp::Not,
                _ => unreachable!(),
            };
            result = Box::new(ast::Unary{op, right: result})
        }

        Ok(result)
    }

    fn parse_primary(&mut self) -> Result<BoxExpression, Error> {
        match self.current_token_type() {
            TT::IntNumber => self.parse_int_literal(),
            TT::String => self.parse_string_literal(),
            TT::True | TT::False => self.parse_bool_literal(),
            TT::FloatNumber => self.parse_float_literal(),
            TT::Identifier => self.parse_idetifier()
                .map(|i| Box::new(i) as Box<dyn ast::Expression>),
            TT::Lparen => self.parse_group(),
            _ => Err(make_error(
                UnexpectedTokenError {
                    current: self.current_token.clone(),
                    expected: vec![TT::IntNumber, TT::String, TT::True, TT::False, TT::FloatNumber, TT::Identifier, TT::Lparen]
                },
                self.current_token.span
            ))
        }
    }

    fn parse_int_literal(&mut self) -> Result<BoxExpression, Error> {
        let token = self.expect_advance(&[TT::IntNumber])?;
        let value: i64 = handle_result(token.lexeme.parse(), token.span)?;

        Ok(Box::new(ast::Literal::Int(value)))
    }

    fn parse_string_literal(&mut self) -> Result<BoxExpression, Error> {
        let token = self.expect_advance(&[TT::String])?;
        let value = &token.lexeme[1..token.lexeme.len() - 1];
        let value = handle_result(unescape_string(value), token.span)?;

        Ok(Box::new(ast::Literal::Str(value)))
    }

    fn parse_bool_literal(&mut self) -> Result<BoxExpression, Error> {
        let token = self.expect_advance(&[TT::True, TT::False])?;
        let value = match token.token_type {
            TT::True => true,
            TT::False => false,
            _ => unreachable!()
        };

        Ok(Box::new(ast::Literal::Bool(value)))
    }

    fn parse_float_literal(&mut self) -> Result<BoxExpression, Error> {
        let token = self.expect_advance(&[TT::FloatNumber])?;
        let value: f64 = handle_result(token.lexeme.parse(), token.span)?;

        Ok(Box::new(ast::Literal::Float(value)))
    }

    fn parse_idetifier(&mut self) -> Result<ast::Identifier, Error> {
        let token = self.expect_advance(&[TT::Identifier])?;
        Ok(ast::Identifier(token.lexeme))
    }

    fn parse_group(&mut self) -> Result<BoxExpression, Error> {
        self.expect_advance(&[TT::Lparen])?;
        self.skip_newlines()?;
        let result = self.parse_expression()?;
        self.expect_advance(&[TT::Rparen])?;

        Ok(result)
    }

    fn expect_advance(&mut self, token_types: &[TokenType]) -> Result<Token, Error> {
        if self.current_token_type_is(token_types) {
            Ok(self.advance()?)
        } else {
            Err(make_error(
                UnexpectedTokenError {
                    current: self.current_token.clone(),
                    expected: token_types.to_vec(),
                },
                self.current_token.span
            ))
        }
    }

    fn skip_terminal(&mut self) -> Result<(), Error> {
        self.skip_tokens(&[
            TokenType::Semicolon,
            TokenType::NewLine,
        ])
    }

    fn skip_newlines(&mut self) -> Result<(), Error> {
        self.skip_tokens(&[
            TokenType::NewLine,
        ])
    }

    fn skip_tokens(&mut self, token_types: &[TokenType]) -> Result<(), Error> {
        while self.current_token_type_is(token_types) {
            self.advance()?;
        }

        Ok(())
    }

    fn current_token_type_is(&self, token_types: &[TokenType]) -> bool {
        token_types.iter().any(|tt| *tt == self.current_token.token_type)
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

fn make_error<E: Into<Error> + 'static>(error: E, span: Option<Span>) -> Error {
    error.into().with_span(span)
}

fn handle_result<T, E: Into<Error> + 'static>(result: Result<T, E>, span: Option<Span>) -> Result<T, Error> {
    result.map_err(|err| make_error(err, span))
}