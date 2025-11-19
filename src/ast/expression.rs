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
pub enum Value {
    Int(i64),
}

impl Expression for Value {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) {
        visitor.visit_value(self);
    }
}

