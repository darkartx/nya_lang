use std::mem;

use crate::{
    ast, lexer::Tokens, span::*, token::*
};

use super::error::*;

type TT = TokenType;
type BoxStatement = Box<dyn ast::Statement>;
type BoxExpression = Box<dyn ast::Expression>;

const EXPRESSION_START_TTS: [TT; 11] = [
    TT::IntNumber, TT::String, TT::True, TT::False, TT::FloatNumber, TT::Identifier, TT::Lparen, TT::Minus, TT::Not,
    TT::BitNot, TT::If,
];

const TERMINAL_TTS: [TT; 1] = [TT::Semicolon];
const ASSIGN_OP_TTS: [TT; 11] = [
    TT::Assign, TT::AssignBitAnd, TT::AssignBitOr, TT::AssignBitXor, TT::AssignDiv, TT::AssignMinus, TT::AssignMod,
    TT::AssignMult, TT::AssignPlus, TT::AssignShiftLeft, TT::AssignShiftRight
];

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
            TT::Let => self.parse_let_statement(),
            TT::Return => self.parse_retrun_statement(),
            TT::Lbrace => self.parse_block(),
            _ => {
                if self.current_token_type_is(&EXPRESSION_START_TTS) {
                    self.parse_expression_statement()
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

    fn parse_block(&mut self) -> Result<BoxStatement, Error> {
        let token = self.expect_advance(&[TT::Lbrace])?;
        let mut statements = vec![];

        while !self.current_token_type_is(&[TT::Rbrace]) {
            statements.push(self.parse_statement()?);
        }

        self.advance()?;

        let statement = ast::Block::new(statements);
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
        match self.current_token_type() {
            TT::If => self.parse_if(),
            _ => self.parse_assigment()
        }
    }

    fn parse_if(&mut self) -> Result<BoxExpression, Error> {
        let token = self.expect_advance(&[TT::If])?;
        self.expect_advance(&[TT::Lparen])?;
        let condition = self.parse_expression()?;
        self.expect_advance(&[TT::Rparen])?;
        let consequence = self.parse_statement()?;

        let alternative = if self.current_token_type_is(&[TT::Else]) {
            self.advance()?;
            Some(self.parse_statement()?)
        } else {
            None
        };

        let expression = ast::If::new(condition, consequence, alternative);
        Ok(self.make_expression_node(expression, Some(token)))
    }

    fn parse_assigment(&mut self) -> Result<BoxExpression, Error> {
        let mut result = self.parse_logic_and()?;

        if self.current_token_type_is(&ASSIGN_OP_TTS) {
            let token = self.advance()?;
            let right = self.parse_logic_or()?;
            let op = match token.token_type {
                TT::Assign => ast::BinaryOp::Assign,
                TT::AssignBitAnd => ast::BinaryOp::AssignBitAnd,
                TT::AssignBitOr => ast::BinaryOp::AssignBitOr,
                TT::AssignBitXor => ast::BinaryOp::AssignBitXor,
                TT::AssignDiv => ast::BinaryOp::AssignDiv,
                TT::AssignMinus => ast::BinaryOp::AssignMinus,
                TT::AssignMod => ast::BinaryOp::AssignMod,
                TT::AssignMult => ast::BinaryOp::AssignMult,
                TT::AssignPlus => ast::BinaryOp::AssignPlus,
                TT::AssignShiftLeft => ast::BinaryOp::AssignShiftLeft,
                TT::AssignShiftRight => ast::BinaryOp::AssignShiftRight,
                _ => unreachable!(),
            };

            result = self.make_binary_expression_node(result, op, right);
        }

        Ok(result)
    }

    fn parse_logic_or(&mut self) -> Result<BoxExpression, Error> {
        let mut result = self.parse_logic_and()?;

        if self.current_token_type_is(&[TT::Or]) {
            self.advance()?;
            let right = self.parse_logic_or()?;
            let op = ast::BinaryOp::Or;

            result = self.make_binary_expression_node(result, op, right);
        }

        Ok(result)
    }

    fn parse_logic_and(&mut self) -> Result<BoxExpression, Error> {
        let mut result = self.parse_equality()?;

        if self.current_token_type_is(&[TT::And]) {
            self.advance()?;
            let right = self.parse_logic_and()?;
            let op = ast::BinaryOp::And;

            result = self.make_binary_expression_node(result, op, right);
        }

        Ok(result)
    }

    fn parse_equality(&mut self) -> Result<BoxExpression, Error> {
        let mut result = self.parse_bit_or()?;

        if self.current_token_type_is(&[TT::Eq, TT::Neq, TT::Gt, TT::Gte, TT::Lt, TT::Lte]) {
            let token = self.advance()?;
            let right = self.parse_equality()?;
            let op = match token.token_type {
                TT::Eq => ast::BinaryOp::Eq,
                TT::Neq => ast::BinaryOp::Neq,
                TT::Gt => ast::BinaryOp::Gt,
                TT::Gte => ast::BinaryOp::Gte,
                TT::Lt => ast::BinaryOp::Lt,
                TT::Lte => ast::BinaryOp::Lte,
                _ => unreachable!(),
            };

            result = self.make_binary_expression_node(result, op, right);
        }

        Ok(result)
    }

    fn parse_bit_or(&mut self) -> Result<BoxExpression, Error> {
        let mut result = self.parse_bit_and()?;

        if self.current_token_type_is(&[TT::BitOr, TT::BitXor]) {
            let token = self.advance()?;
            let right = self.parse_bit_or()?;
            let op = match token.token_type {
                TT::BitOr => ast::BinaryOp::BitOr,
                TT::BitXor => ast::BinaryOp::BitXor,
                _ => unreachable!(),
            };

            result = self.make_binary_expression_node(result, op, right);
        }

        Ok(result)
    }

    fn parse_bit_and(&mut self) -> Result<BoxExpression, Error> {
        let mut result = self.parse_shift()?;

        if self.current_token_type_is(&[TT::BitAnd]) {
            self.advance()?;
            let right = self.parse_bit_and()?;
            let op = ast::BinaryOp::BitAnd;

            result = self.make_binary_expression_node(result, op, right);
        }

        Ok(result)
    }

    fn parse_shift(&mut self) -> Result<BoxExpression, Error> {
        let mut result = self.parse_term()?;

        if self.current_token_type_is(&[TT::ShiftLeft, TT::ShiftRight]) {
            let token = self.advance()?;
            let right = self.parse_shift()?;
            let op = match token.token_type {
                TT::ShiftLeft => ast::BinaryOp::ShiftLeft,
                TT::ShiftRight => ast::BinaryOp::ShiftRight,
                _ => unreachable!()
            };

            result = self.make_binary_expression_node(result, op, right);
        }

        Ok(result)
    }

    fn parse_term(&mut self) -> Result<BoxExpression, Error> {
        let mut result = self.parse_factor()?;

        if self.current_token_type_is(&[TT::Plus, TT::Minus]) {
            let token = self.advance()?;
            let right = self.parse_term()?;
            let op = match token.token_type {
                TT::Plus => ast::BinaryOp::Plus,
                TT::Minus => ast::BinaryOp::Minus,
                _ => unreachable!(),
            };

            result = self.make_binary_expression_node(result, op, right);
        }

        Ok(result)
    }

    fn parse_factor(&mut self) -> Result<BoxExpression, Error> {
        let mut result = self.parse_unary()?;

        if self.current_token_type_is(&[TT::Mult, TT::Div, TT::Mod]) {
            let token = self.advance()?;
            let right = self.parse_factor()?;
            let op = match token.token_type {
                TT::Mult => ast::BinaryOp::Mult,
                TT::Div => ast::BinaryOp::Div,
                TT::Mod => ast::BinaryOp::Mod,
                _ => unreachable!(),
            };

            result = self.make_binary_expression_node(result, op, right);
        }

        Ok(result)
    }

    fn parse_unary(&mut self) -> Result<BoxExpression, Error> {
        let unary_token = self.advance_if(&[TT::Minus, TT::Not, TT::BitNot])?;

        if let Some(unary_token) = unary_token {
            let expression = self.parse_unary()?;
            let op = match unary_token.token_type {
                TT::Minus => ast::UnaryOp::Minus,
                TT::Not => ast::UnaryOp::Not,
                TT::BitNot => ast::UnaryOp::BitNot,
                _ => unreachable!(),
            };
            let expression = ast::Unary{op, right: expression};
            Ok(self.make_expression_node(expression, Some(unary_token)))
        } else {
            self.parse_primary()
        }
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
        let result = self.parse_expression()?;
        self.expect_advance(&[TT::Rparen])?;

        Ok(result)
    }

    fn parse_terminal(&mut self) -> Result<(), Error> {
        match self.current_token_type() {
            TT::Semicolon => { self.advance()?; },
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

    fn make_binary_expression_node(&mut self, left: BoxExpression, op: ast::BinaryOp, right: BoxExpression) -> BoxExpression {
        let token = left.token().cloned();
        let expression = ast::Binary{ left, op, right };
        
        self.make_expression_node(expression, token)
    }

    fn advance_if(&mut self, token_type: &[TokenType]) -> Result<Option<Token>, Error> {
        if self.current_token_type_is(token_type) {
            self.advance().map(|r| Some(r))
        } else {
            Ok(None)
        }
    }

    fn advance(&mut self) -> Result<Token, Error> {
        let next_token = loop {
            let token = self.tokens.next_token()?;

            if token.token_type != TT::SingleLineComment {
                break token;
            }
        };
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