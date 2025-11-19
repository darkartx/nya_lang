use super::*;

#[derive(Debug)]
pub struct Let {
    identifier: Identifier,
    expression: Option<Box<dyn Expression>>,
}

impl Statement for Let {
    fn accept(&self, visitor: &mut dyn StatementVisitor) {
        visitor.visit_let(self);
    }
}

impl Let {
    pub fn new(identifier: Identifier, expression: Option<Box<dyn Expression>>) -> Self {
        Self { identifier, expression }
    }

    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }

    pub fn expression(&self) -> Option<&dyn Expression> {
        self.expression.as_deref()
    }
}
