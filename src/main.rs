mod core;
use crate::core::Lexer;

fn main() {
    let lexer = Lexer::new(r#"def x : int = 50 + "hello world""#);
    let tokens = lexer.scan();

    for tok in tokens {
        // Print the tokens
        println!("{:#?}", tok);
    }
}
