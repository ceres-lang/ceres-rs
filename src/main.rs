mod core;
use crate::core::Lexer;

fn main() {
    let mut lexer = Lexer::new("def x : int = 3;");
    lexer.scan();
}
