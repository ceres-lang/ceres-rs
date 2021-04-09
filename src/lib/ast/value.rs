use crate::token::Token;
use crate::ast::{Node, NodeVisitor, Visitable};

/// Stores the type associated with the ValueNode
enum ValueType {
    Integer,
    Bool,
    StringLit,
    CharLit
}

pub struct ValueNode<T> {
    value_type: ValueType,
    pub value: T
}

/// Implement the visitor pattern for nodes
impl Visitable for ValueNode<i32> {
    fn accept(_visitor: NodeVisitor) {

    }
}


impl Visitable for ValueNode<String> {
    fn accept(_visitor: NodeVisitor) {
        
    }
}