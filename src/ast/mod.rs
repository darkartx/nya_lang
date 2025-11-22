pub mod statement;
pub mod expression;
pub mod node;

use std::fmt;

use crate::{
    span::Span,
    token::Token,
};

pub use statement::*;
pub use expression::*;
pub use node::*;

pub trait Expression: fmt::Debug {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor);
    fn token(&self) -> Option<&Token> { None }
    fn span(&self) -> Option<Span> { None }
    fn id(&self) -> Option<NodeId> { None }
}

pub trait ExpressionVisitor {
    fn visit_identifier(&mut self, identifier: &Identifier);
    fn visit_literal(&mut self, literal: &Literal);
    fn visit_binary(&mut self, binary: &Binary);
    fn visit_unary(&mut self, unary: &Unary);
    fn visit_if(&mut self, if_expr: &If);
}

pub trait Statement: fmt::Debug {
    fn accept(&self, visitor: &mut dyn StatementVisitor);
    fn token(&self) -> Option<&Token> { None }
    fn span(&self) -> Option<Span> { None }
    fn id(&self) -> Option<NodeId> { None }
}

pub trait StatementVisitor {
    fn visit_ast(&mut self, ast: &Ast);
    fn visit_let(&mut self, let_statement: &Let);
    fn visit_return(&mut self, return_statement: &Return);
    fn visit_expr(&mut self, expr: &Expr);
    fn visit_block(&mut self, block: &Block);
}

#[derive(Debug, Default)]
pub struct Ast {
    statements: Vec<Box<dyn Statement>>,
}

impl Statement for Ast {
    fn accept(&self, visitor: &mut dyn StatementVisitor) {
        visitor.visit_ast(self);
    }
}

impl Ast {
    pub fn new(statements: Vec<Box<dyn Statement>>) -> Self {
        Self { statements }
    }

    pub fn add_statement<S: Statement + 'static>(&mut self, statement: S) {
        let statement = Box::new(statement);
        self.statements.push(statement);
    }

    pub fn statements(&self) -> &[Box<dyn Statement>] {
        &self.statements
    }
}
