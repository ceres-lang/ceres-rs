// Lexer for Ceres
use std::vec;
use std::char;
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

    /// Advances the position in the lexer
    fn advance(&mut self) {
        self.pos += 1
    }

    fn peek(&self) -> char {
        // TODO: maybe don't die but rather return an optional?
        self.src.chars().nth(self.pos).expect("reached eof")
    }

    /// Pushes `right` if the next character in the stream is `next_char`, else pushes
    /// `left` as a token.
    fn push_if_next_ahead(&mut self, next_char: char, left: Token, right: Token) {
        if self.peek_next(1) == next_char {
            self.tokens.push(right);
        }
        else {
            self.tokens.push(left);
        }
    }

    /// Returns the next character in the stream `ahead` times ahead
    fn peek_next(&self, ahead: usize) -> char {
        self.src.chars().nth(self.pos + ahead).expect("reached eof")
    }

    fn is_eof(&self) -> bool { 
        self.pos >= self.src.len()
    }

    fn scan_string(&mut self) {
        let mut lexeme = String::new();

        self.advance();
        while self.peek() != '"' {
            lexeme.push(self.peek());
            self.advance();
        }

        self.tokens.push(Token::StringLit(lexeme));
    }

    fn scan_number(&mut self) {
        // TODO: add support for 0x and 0b

        let mut lexeme = String::new();

        while self.peek().is_ascii_digit() {
            lexeme.push(self.peek());
            self.advance();
        }

        let value = lexeme.parse().expect("invalid integer literal");
        self.tokens.push(Token::IntLit(value));
    }

    fn scan_kw_or_ident(&mut self) {
        let mut lexeme = String::new();

        while self.peek().is_ascii_alphanumeric() || self.peek() == '_' {
            lexeme.push(self.peek());
            self.advance();
        }

        if KEYWORDS.iter().any(|&s| s == lexeme) {
            // Lexeme is a keyword
            match lexeme.as_str() {
                "case" => self.tokens.push(Token::Case),
                "const" => self.tokens.push(Token::Const),
                "def" => self.tokens.push(Token::Def),
                "if" => self.tokens.push(Token::If),
                "elif" => self.tokens.push(Token::Elif),
                "else" => self.tokens.push(Token::Else),
                "for" => self.tokens.push(Token::For),
                "fn" => self.tokens.push(Token::Fn),
                _ => {}
            }
        }
        else if TYPE_KEYWORDS.iter().any(|&s| s == lexeme) {
            match lexeme.as_str() {
                "int" => self.tokens.push(Token::TypeInt),
                "char" => self.tokens.push(Token::TypeChar),
                "bool" => self.tokens.push(Token::TypeBool),
                "str" => self.tokens.push(Token::TypeStr),
                _ => {}
            }
        }
        else {
            self.tokens.push(Token::Identifier(lexeme));
        }
    }

    pub fn scan(&mut self) {
        println!("src: {}", self.src);

        while !self.is_eof() {
            match self.peek() {
                '+' => self.tokens.push(Token::Plus),
                '-' => self.tokens.push(Token::Minus),
                '*' => self.tokens.push(Token::Star),
                '/' => {
                    // Ignore comments
                    if self.peek_next(1) == '/' {
                        while self.peek() != '\n' {
                            self.advance();
                        }
                    }
                    else {
                        self.tokens.push(Token::Slash);
                    }
                }
                '.' => self.tokens.push(Token::Dot),
                ',' => self.tokens.push(Token::Comma),
                '(' => self.tokens.push(Token::LeftParen),
                ')' => self.tokens.push(Token::RightParen),
                '{' => self.tokens.push(Token::LeftBrace),
                '}' => self.tokens.push(Token::RightBrace),
                ':' => self.tokens.push(Token::Colon),
                ';' => self.tokens.push(Token::Semicolon),
                '!' => self.push_if_next_ahead('=', Token::Not, Token::NotEquals),
                '=' => self.push_if_next_ahead('=', Token::Equals, Token::EqualsEquals),
                '<' => self.push_if_next_ahead('=', Token::Less, Token::LesserEqual),
                '>' => self.push_if_next_ahead('=', Token::Greater, Token::GreaterEqual),
                '"' => self.scan_string(),
                _ => {
                    // Ignore any useless whitespace
                    if self.peek().is_whitespace() {
                        self.advance();
                    }

                    if self.peek().is_ascii_alphabetic() {
                        self.scan_kw_or_ident();
                    }
                    else if self.peek().is_ascii_digit() {
                        self.scan_number();
                    }
                }
            }

            self.advance();
        }

        for tok in &self.tokens {
            println!("{:#?}", tok);
        }
    }
}