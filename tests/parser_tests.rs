use ceres::lexer::Lexer;
use ceres::parser::Parser;
use ceres::token::Token;

// AST
use ceres::ast::IdentNode;
use ceres::ast::value::{ValueNode, ValueType};

#[cfg(test)]
mod parser_tests {
    use super::*;

    fn scan_and_return(input: &str) -> Vec<Token> {
        let lexer = Lexer::new(input);
        lexer.scan()
    }

    #[test]
    fn parser_parse_number() {
        let tokens = scan_and_return("69");
        let mut p = Parser::new(tokens);

        let r = p.parse_number();
        assert_eq!(r, ValueNode::new(ValueType::Integer, 69));
    }

    #[test]
    fn parser_parse_string() {
        let tokens = scan_and_return(r#""Hello world""#);
        let mut p = Parser::new(tokens);

        let r = p.parse_string();
        assert_eq!(r, ValueNode::new(ValueType::StringLit, "Hello world".into()));
    }

    #[test]
    fn parser_parse_valid_ident() {
        let tokens = scan_and_return(r#"validIdent"#);
        let mut p = Parser::new(tokens);

        let r = p.parse_ident();
        assert_eq!(r, IdentNode::new("validIdent".into()));
    }

    #[test]
    /// Try to parse an identifier, followed by a string literal
    fn parser_parse_ident_then_string() {
        let tokens = scan_and_return(r#"name "charlotte""#);
        let mut p = Parser::new(tokens);

        let ident = p.parse_ident();
        let string = p.parse_string();

        // Expect an identifier, followed by a string literal
        assert_eq!(ident, IdentNode::new("name".into()));
        assert_eq!(string, ValueNode::new(ValueType::StringLit, "charlotte".into()));
    }

    #[test]
    /// Try to parse an identifier, followed by an integer
    fn parser_parse_ident_then_number() {
        let tokens = scan_and_return(r#"number 123"#);
        let mut p = Parser::new(tokens);

        let ident = p.parse_ident();
        let num = p.parse_number();

        // Expect an identifier, followed by an integer
        assert_eq!(ident, IdentNode::new("number".into()));
        assert_eq!(num, ValueNode::new(ValueType::Integer, 123));
    }
}