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