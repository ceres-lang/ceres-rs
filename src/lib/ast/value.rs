use crate::token::Token;
use crate::ast::Visitor;

/// Stores the type associated with the ValueNode
#[derive(Debug)]
pub enum ValueType {
    Integer,
    Bool,
    StringLit,
    CharLit
}

#[derive(Debug)]
pub struct ValueNode<T> {
    value_type: ValueType,
    pub value: T
}

impl<T> ValueNode<T> {
    pub fn new(value_type: ValueType, value: T) -> ValueNode<T> {
        ValueNode {
            value_type: value_type,
            value: value 
        }
    }
}