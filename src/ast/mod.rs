pub mod statement;
pub mod expression;

use std::fmt;

pub use statement::*;
pub use expression::*;

pub trait Expression: fmt::Debug {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor);
}

pub trait ExpressionVisitor {
    fn visit_identifier(&mut self, identifier: &Identifier);
    fn visit_value(&mut self, value: &Value);
}

pub trait Statement: fmt::Debug {
    fn accept(&self, visitor: &mut dyn StatementVisitor);
}

pub trait StatementVisitor {
    fn visit_ast(&mut self, ast: &Ast);
    fn visit_let(&mut self, expression: &Let);
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
