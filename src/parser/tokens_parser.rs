use std::mem;

use crate::{
    ast, lexer::Tokens, span::*, token::*
};

use super::error::*;

type TT = TokenType;
type BoxStatement = Box<dyn ast::Statement>;
type BoxExpression = Box<dyn ast::Expression>;

const EXPRESSION_START_TTS: [TT; 9] = [
    TT::IntNumber, TT::String, TT::True, TT::False, TT::FloatNumber, TT::Identifier, TT::Lparen, TT::Minus, TT::Not
];

const TERMINAL_TTS: [TT; 2] = [TT::NewLine, TT::Semicolon];

#[derive(Debug)]
pub(super) struct TokensParser<'a> {
    tokens: Tokens<'a>,
    current_token: Token,
    peek_token: Token,
    node_id_gen: ast::NodeIdGen,
}

impl<'a> TokensParser<'a> {
    pub(super) fn new(tokens: Tokens<'a>) -> Self {
        Self {
            tokens,
            current_token: Default::default(),
            peek_token: Default::default(),
            node_id_gen: Default::default(),
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
                self.parse_statement()?.into()
            );
        }

        Ok(
            ast::Ast::new(statements)
        )
    }

    fn parse_statement(&mut self) -> Result<BoxStatement, Error> {
        match self.current_token_type() {
            TT::Let => self.parse_let_statement().map(Into::into),
            TT::Return => self.parse_retrun_statement().map(Into::into),
            _ => {
                if self.current_token_type_is(&EXPRESSION_START_TTS) {
                    self.parse_expression_statement().map(Into::into)
                } else {
                    let mut expected = vec![TT::Let, TT::Return];
                    expected.extend(EXPRESSION_START_TTS);

                    Err(Error::new(
                        ErrorKind::ExpectStatement(
                            UnexpectedTokenError {
                                token: self.current_token.clone(),
                                expected
                            }
                        ),
                        self.current_span()
                    ))
                }
            }
        }
    }

    fn parse_let_statement(&mut self) -> Result<BoxStatement, Error> {
        let token = self.expect_advance(&[TT::Let])?;
        let identifier = self.parse_idetifier()?;
        
        let expression = if self.current_token_type_is(&[TokenType::Assign]) {
            self.advance()?;
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.parse_terminal()?;
        let statement = ast::Let::new(identifier, expression);

        Ok(self.make_statement_node(statement, Some(token)))
    }

    fn parse_retrun_statement(&mut self) -> Result<BoxStatement, Error> {
        let token = self.expect_advance(&[TT::Return])?;
        let expression = if self.current_token_type_is(&EXPRESSION_START_TTS) {
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.parse_terminal()?;

        let statement = ast::Return::new(expression);

        Ok(self.make_statement_node(statement, Some(token)))
    }

    fn parse_expression_statement(&mut self) -> Result<BoxStatement, Error> {
        let token = self.current_token.clone();
        let expression = self.parse_expression()?;
        self.parse_terminal()?;
        let statement = ast::Expr::new(expression);

        Ok(self.make_statement_node(statement, Some(token)))
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
            let token = result.token().cloned();
            let expression = ast::Binary{ left: result, op, right };

            result = self.make_expression_node(expression, token)
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
            let token = result.token().cloned();
            let expression = ast::Binary{ left: result, op, right };

            result = self.make_expression_node(expression, token)
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
            let expression = ast::Unary{op, right: result};
            result = self.make_expression_node(expression, Some(unary_token))
        }

        Ok(result)
    }

    fn parse_primary(&mut self) -> Result<BoxExpression, Error> {
        match self.current_token_type() {
            TT::IntNumber => self.parse_int_literal(),
            TT::String => self.parse_string_literal(),
            TT::True | TT::False => self.parse_bool_literal(),
            TT::FloatNumber => self.parse_float_literal(),
            TT::Identifier => self.parse_idetifier(),
            TT::Lparen => self.parse_group(),
            _ => {
                Err(Error::new(
                    ErrorKind::ExpectExpression(
                        UnexpectedTokenError {
                            token: self.current_token.clone(),
                            expected: EXPRESSION_START_TTS.to_vec()
                        }
                    ),
                    self.current_span()
                ))
            }
        }
    }

    fn parse_int_literal(&mut self) -> Result<BoxExpression, Error> {
        let token = self.expect_advance(&[TT::IntNumber])?;
        let value: i64 = handle_result(token.lexeme.parse(), token.span)?;
        let expression = ast::Literal::Int(value);

        Ok(self.make_expression_node(expression, Some(token)))
    }

    fn parse_string_literal(&mut self) -> Result<BoxExpression, Error> {
        let token = self.expect_advance(&[TT::String])?;
        let value = &token.lexeme[1..token.lexeme.len() - 1];
        let value = handle_result(unescape_string(value), token.span)?;
        let expression = ast::Literal::Str(value);

        Ok(self.make_expression_node(expression, Some(token)))
    }

    fn parse_bool_literal(&mut self) -> Result<BoxExpression, Error> {
        let token = self.expect_advance(&[TT::True, TT::False])?;
        let value = match token.token_type {
            TT::True => true,
            TT::False => false,
            _ => unreachable!()
        };
        let expression = ast::Literal::Bool(value);

        Ok(self.make_expression_node(expression, Some(token)))
    }

    fn parse_float_literal(&mut self) -> Result<BoxExpression, Error> {
        let token = self.expect_advance(&[TT::FloatNumber])?;
        let value: f64 = handle_result(token.lexeme.parse(), token.span)?;
        let expression = ast::Literal::Float(value);

        Ok(self.make_expression_node(expression, Some(token)))
    }

    fn parse_idetifier(&mut self) -> Result<BoxExpression, Error> {
        let token = self.expect_advance(&[TT::Identifier])?;
        let expression = ast::Identifier(token.lexeme.clone());
        
        Ok(self.make_expression_node(expression, Some(token)))
    }

    fn parse_group(&mut self) -> Result<BoxExpression, Error> {
        self.expect_advance(&[TT::Lparen])?;
        self.skip_newlines()?;
        let result = self.parse_expression()?;
        self.expect_advance(&[TT::Rparen])?;

        Ok(result)
    }

    fn parse_terminal(&mut self) -> Result<(), Error> {
        match self.current_token_type() {
            TT::Semicolon | TT::NewLine => { self.advance()?; },
            TT::Rbrace | TT::Eof => {},
            _ => {
                return Err(Error::new(
                    ErrorKind::ExpectTerminal(
                        UnexpectedTokenError {
                            token: self.current_token.clone(),
                            expected: TERMINAL_TTS.to_vec()
                        }
                    ),
                     self.current_span()
                ))
            }
        }

        Ok(())
    }

    fn expect_advance(&mut self, token_types: &[TokenType]) -> Result<Token, Error> {
        if self.current_token_type_is(token_types) {
            Ok(self.advance()?)
        } else {
            Err(make_error(
                UnexpectedTokenError {
                    token: self.current_token.clone(),
                    expected: token_types.to_vec(),
                },
                self.current_span()
            ))
        }
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

    #[inline]
    fn current_token_type(&self) -> TokenType {
        self.current_token.token_type
    }

    #[inline]
    fn current_span(&self) -> Option<Span> {
        self.current_token.span
    }

    fn make_statement_node<T: ast::Statement + 'static>(&mut self, kind: T, token: Option<Token>) -> BoxStatement {
        let id = self.node_id_gen.next_id();
        ast::Node::new(id, kind, token).into()
    }

    fn make_expression_node<T: ast::Expression + 'static>(&mut self, kind: T, token: Option<Token>) -> BoxExpression {
        let id = self.node_id_gen.next_id();
        ast::Node::new(id, kind, token).into()
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