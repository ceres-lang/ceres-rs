use crate::token::Token;

use crate::ast::value;

/// Implement the visitor pattern for the AST
/// https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html
pub trait Visitor<T> {
    fn visit_value(&mut self, node: ValueNode) -> T;
}