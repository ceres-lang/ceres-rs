use std::vec;

use ceres::lexer::Lexer;
use ceres::parser::Parser;
use ceres::token::Token;

#[cfg(test)]
mod lexer_tests {
    use super::*;

    fn scan_and_return(input: &str) -> Vec<Token> {
        let lexer = Lexer::new(input);
        lexer.scan()
    }

    #[test]
    fn lexer_scan_string() {
        let tokens = scan_and_return(r#""hello world""#);
        assert_eq!(tokens, [Token::StringLit("hello world".into())]);
    }

    #[test]
    fn lexer_scan_expr() {
        let tokens = scan_and_return(r#"1 + 2 * 3"#);
        assert_eq!(tokens, [Token::IntLit(1), Token::Plus, Token::IntLit(2), Token::Star, Token::IntLit(3)]);
    }

    #[test]
    fn lexer_ignore_comment() {
        let tokens = scan_and_return("// this should be ignored\n1 + 2");
        assert_eq!(tokens, [Token::IntLit(1), Token::Plus, Token::IntLit(2)]);
    }

    #[test]
    fn lexer_ignore_whitespace() {
        let tokens = scan_and_return("    1\t       +\n\n\n      2\n\t+ 3       ");
        assert_eq!(tokens, [Token::IntLit(1), Token::Plus, Token::IntLit(2), Token::Plus, Token::IntLit(3)]);
    }

    #[test]
    fn parser_parse_number() {
        let tokens = scan_and_return("69");
        let mut p = Parser::new(tokens);

        p.parse_number();
    }
}