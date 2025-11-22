use super::*;

#[derive(Debug)]
pub struct Identifier(pub String);

impl Into<Box<dyn Expression>> for Identifier {
    fn into(self) -> Box<dyn Expression> {
        Box::new(self)
    }
}

impl ToString for Identifier {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl Expression for Identifier {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) {
        visitor.visit_identifier(self);
    }
}

#[derive(Debug)]
pub enum Literal {
    Int(i64),
    Str(String),
    Bool(bool),
    Float(f64),
}

impl Into<Box<dyn Expression>> for Literal {
    fn into(self) -> Box<dyn Expression> {
        Box::new(self)
    }
}

impl Expression for Literal {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) {
        visitor.visit_literal(self);
    }
}

#[derive(Debug)]
pub struct Binary {
    pub left: Box<dyn Expression>,
    pub op: BinaryOp,
    pub right: Box<dyn Expression>,
}

impl Into<Box<dyn Expression>> for Binary {
    fn into(self) -> Box<dyn Expression> {
        Box::new(self)
    }
}

impl Expression for Binary {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) {
        visitor.visit_binary(self)
    }
}

#[derive(Debug)]
pub enum BinaryOp {
    Plus,
    Minus,
    Eq,
    Neq,
    And,
    Or,
    Gt,
    Gte,
    Lt,
    Lte,
    Mult,
    Div,
    Mod,
}

#[derive(Debug)]
pub struct Unary {
    pub op: UnaryOp,
    pub right: Box<dyn Expression>,
}

impl Into<Box<dyn Expression>> for Unary {
    fn into(self) -> Box<dyn Expression> {
        Box::new(self)
    }
}

impl Expression for Unary {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) {
        visitor.visit_unary(self)
    }
}

#[derive(Debug)]
pub enum UnaryOp {
    Minus,
    Not,
}
