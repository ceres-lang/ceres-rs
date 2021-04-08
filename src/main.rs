mod core;
use crate::core::Lexer;

fn main() {
    let mut lexer = Lexer::new(r#"def message : str = "hello" + 54;"#);
    lexer.scan();
}
