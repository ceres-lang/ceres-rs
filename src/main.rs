use ceres::lexer::Lexer;
use ceres::parser::Parser;

fn main() {
    let lexer = Lexer::new(r#"hello_world123 69 "hello world""#);

    let tokens = lexer.scan();

    // for tok in tokens {
    //     // Print the tokens
    //     println!("{:#?}", tok);
    // }

    let mut p = Parser::new(tokens);
    
    let i = p.parse_ident();
    println!("{:?}", i);

    let n = p.parse_number();

    println!("{:?}", n);

    let m = p.parse_string();
    println!("{:?}", m);
}
