use super::*;

#[derive(Debug)]
pub struct Let {
    pub identifier: Box<dyn Expression>,
    pub expression: Option<Box<dyn Expression>>,
}

impl Into<Box<dyn Statement>> for Let {
    fn into(self) -> Box<dyn Statement> {
        Box::new(self)
    }
}

impl Statement for Let {
    fn accept(&self, visitor: &mut dyn StatementVisitor) {
        visitor.visit_let(self);
    }
}

impl Let {
    pub fn new(identifier: Box<dyn Expression>, expression: Option<Box<dyn Expression>>) -> Self {
        Self { identifier, expression }
    }
}

#[derive(Debug)]
pub struct Return {
    pub expression: Option<Box<dyn Expression>>,
}

impl Into<Box<dyn Statement>> for Return {
    fn into(self) -> Box<dyn Statement> {
        Box::new(self)
    }
}

impl Statement for Return {
    fn accept(&self, visitor: &mut dyn StatementVisitor) {
        visitor.visit_return(self)
    }
}

impl Return {
    pub fn new(expression: Option<Box<dyn Expression>>) -> Self {
        Self { expression }
    }
}

#[derive(Debug)]
pub struct Expr {
    pub expression: Box<dyn Expression>,
}

impl Into<Box<dyn Statement>> for Expr {
    fn into(self) -> Box<dyn Statement> {
        Box::new(self)
    }
}

impl Statement for Expr {
    fn accept(&self, visitor: &mut dyn StatementVisitor) {
        visitor.visit_expr(self);
    }
}

impl Expr {
    pub fn new(expression: Box<dyn Expression>) -> Self {
        Self { expression }
    }
}

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Into<Box<dyn Statement>> for Block {
    fn into(self) -> Box<dyn Statement> {
        Box::new(self)
    }
}

impl Statement for Block {
    fn accept(&self, visitor: &mut dyn StatementVisitor) {
        visitor.visit_block(self);
    }
}

impl Block {
    pub fn new(statements: Vec<Box<dyn Statement>>) -> Self {
        Self { statements }
    }
}
