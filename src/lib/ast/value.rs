/// Stores the type associated with the ValueNode
#[derive(Debug, PartialEq)]
pub enum ValueType {
    Integer,
    Bool,
    StringLit,
    CharLit
}

#[derive(Debug, PartialEq)]
pub struct ValueNode<T> {
    value_type: ValueType,
    pub value: T
}

/// A node that stores primitive values (integers, strings, booleans) etc
impl<T> ValueNode<T> {
    pub fn new(value_type: ValueType, value: T) -> ValueNode<T> {
        ValueNode {
            value_type: value_type,
            value: value 
        }
    }
}