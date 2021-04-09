use crate::token::Token;

/// Base AST node
pub struct Node {

}

pub struct NodeVisitor {}

/// Implements the Visitor pattern on all AST nodes
pub trait Visitable {
    fn accept(visitor: NodeVisitor)
}



