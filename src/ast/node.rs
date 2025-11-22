use crate::{
    token::Token,
    span::Span,
};
use super::{Expression, Statement, ExpressionVisitor, StatementVisitor};

#[derive(Debug, Clone, Copy)]
pub struct NodeId(pub u32);

#[derive(Debug)]
pub struct Node<T> {
    pub id: NodeId,
    pub kind: T,
    pub token: Option<Token>
}

impl<T: Expression + 'static> Into<Box<dyn Expression>> for Node<T> {
    fn into(self) -> Box<dyn Expression> {
        Box::new(self)
    }
}

impl<T: Statement + 'static> Into<Box<dyn Statement>> for Node<T> {
    fn into(self) -> Box<dyn Statement> {
        Box::new(self)
    }
}

impl<T: Expression> Expression for Node<T> {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) {
        self.kind.accept(visitor);
    }

    fn token(&self) -> Option<&Token> {
        self.token.as_ref()
    }

    fn span(&self) -> Option<Span> {
        self.token.as_ref().map(|t| t.span).flatten()
    }

    fn id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl<T: Statement> Statement for Node<T> {
    fn accept(&self, visitor: &mut dyn StatementVisitor) {
        self.kind.accept(visitor);
    }

    fn token(&self) -> Option<&Token> {
        self.token.as_ref()
    }

    fn span(&self) -> Option<Span> {
        self.token.as_ref().map(|t| t.span).flatten()
    }

    fn id(&self) -> Option<NodeId> {
        Some(self.id)
    }
}

impl<T> Node<T> {
    pub fn new(id: NodeId, kind: T, token: Option<Token>) -> Self {
        Self {
            id,
            kind,
            token
        }
    }
}

#[derive(Debug)]
pub struct NodeIdGen {
    next: u32
}

impl Iterator for NodeIdGen {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_id())
    }
}

impl Default for NodeIdGen {
    fn default() -> Self {
        Self { next: 1 }
    }
}

impl NodeIdGen {
    pub fn new(next: u32) -> Self {
        Self { next }
    }

    pub fn next_id(&mut self) -> NodeId {
        let id = self.next;
        self.next += 1;
        NodeId(id)
    }
}
