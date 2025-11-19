use super::*;

#[derive(Debug)]
pub struct Identifier(String);

impl Expression for Identifier {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) {
        visitor.visit_identifier(self);
    }
}

impl Identifier {
    pub fn new(name: String) -> Self {
        Self(name)
    }

    pub fn name(&self) -> &String {
        &self.0
    }
}

#[derive(Debug)]
pub enum Literal {
    Int(i64),
    Str(String),
    Bool(bool),
    Float(f64),
}

impl Expression for Literal {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) {
        visitor.visit_literal(self);
    }
}

