// Make value.rs accessible by other files
pub mod value;

#[derive(Debug, PartialEq)]
pub struct IdentNode {
    pub name: String 
}

impl IdentNode {
    pub fn new(name: String) -> Self {
        IdentNode { name: name }
    }
}

/// Implement the visitor pattern for the AST
/// https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html
pub trait Visitor<T> {
    fn visit_value(&mut self, node: value::ValueNode<T>) -> T;
}
