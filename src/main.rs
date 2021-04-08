mod core;
use crate::core::Lexer;

fn main() {
    let mut lexer = Lexer::new(r#"def x : int = 50 + "hello world""#);
    lexer.scan();
}
