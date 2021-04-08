// Lexer for Ceres
use std::vec;
use lazy_static::lazy_static;

use crate::core::Token;

lazy_static! {
    static ref TYPE_KEYWORDS: Vec<&'static str> = vec![
        "int",
        "bool",
        "char",
        "str"
    ];

    // Reserved keywords
    static ref KEYWORDS: Vec<&'static str> = vec![
        "case",
        "const",
        "def",
        "if",
        "elif",
        "else",
        "exit",
        "for",
        "fn",
        "while",
    ];
}

pub struct Lexer {
    pos: usize,
    src: String,
    tokens: Vec<Token>
}

impl Lexer {
    pub fn new(src: &str) -> Lexer {
        Lexer {
            pos: 0,
            src: src.into(),
            tokens: vec![],
        }
    }

    fn advance(&mut self) {
        self.pos += 1
    }

    fn peek(&self) -> Option<char> {
        // TODO: maybe don't die but rather return an optional?
        self.src.chars().nth(self.pos)
    }

    pub fn is_eof(&self) -> bool { 
        self.pos >= self.src.len()
    }

    pub fn scan(&mut self) {
        println!("src: {}", self.src);

        while !self.is_eof() {
            println!("{}", self.peek().unwrap());
        }
    }
}


