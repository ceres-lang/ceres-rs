// Lexer for Ceres
use std::vec;
use std::char;
use lazy_static::lazy_static;

// use crate::stream::PeekableStream;
use crate::token::Token;

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
        "loop",
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

    fn is_eos(&self) -> bool {
        self.pos >= self.src.len()
    }

    fn peek(&self) -> char {
        if self.is_eos() { return '\0'; }
        self.src.chars().nth(self.pos).unwrap()
    }

    /// Push the token and advance the position in the stream
    fn push_advance(&mut self, token: Token) {
        self.tokens.push(token);
        self.advance();
    }

    /// Pushes `right` if the next character in the stream is `next_char`, else pushes
    /// `left` as a token.
    /// 
    /// This little manuever is gonna save us 100 lines :')
    fn push_if_next_ahead(&mut self, next_char: char, left: Token, right: Token) {
        if self.peek_next(1) == next_char {
            self.tokens.push(right);
            self.advance();
        }
        else {
            self.tokens.push(left);
            self.advance();
        }
    }

    /// Returns the next character in the stream `ahead` times ahead
    fn peek_next(&self, ahead: usize) -> char {
        self.src.chars().nth(self.pos + ahead).expect("reached eof")
    }

    fn scan_string(&mut self) {
        let mut lexeme = String::new();

        self.advance();
        while self.peek() != '"' {
            lexeme.push(self.peek());
            self.advance();
        }

        self.advance();
        self.tokens.push(Token::StringLit(lexeme));
    }

    fn scan_char_lit(&mut self) {
        let mut lexeme = String::new();

        self.advance();
        while self.peek() != '\'' {
            //println!("char lit {}", self.peek());
            lexeme.push(self.peek());
            self.advance();
        }

        let value = lexeme.parse::<char>().expect("error: character literals can only hold 1 character!");
        self.advance();
        self.tokens.push(Token::CharLit(value));
    }

    fn scan_number(&mut self) {
        // TODO: add support for 0x and 0b
        let mut lexeme = String::new();

        while self.peek().is_ascii_digit() {
            lexeme.push(self.peek());
            self.advance();
        }

        let value = lexeme.parse::<i32>().expect("error: invalid integer literal");
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
                "while" => self.tokens.push(Token::While),
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

    pub fn scan(mut self) -> Vec<Token> {
        println!("src: {}", self.src);

        while !self.is_eos() {
            match self.peek() {
                '+' => self.push_advance(Token::Plus),
                '-' => self.push_advance(Token::Minus),
                '*' => self.push_advance(Token::Star),
                '/' => {
                    // Ignore comments
                    self.advance();
                    if self.peek() == '/' {
                        while self.peek() != '\n' {
                            self.advance();
                        }
                    }
                    else {
                        self.tokens.push(Token::Slash);
                    }
                }
                '.' => self.push_advance(Token::Dot),
                ',' => self.push_advance(Token::Comma),
                '(' => self.push_advance(Token::LeftParen),
                ')' => self.push_advance(Token::RightParen),
                '{' => self.push_advance(Token::LeftBrace),
                '}' => self.push_advance(Token::RightBrace),
                ':' => self.push_advance(Token::Colon),
                ';' => self.push_advance(Token::Semicolon),
                '!' => self.push_if_next_ahead('=', Token::Not, Token::NotEquals),
                '=' => self.push_if_next_ahead('=', Token::Equals, Token::EqualsEquals),
                '<' => self.push_if_next_ahead('=', Token::Less, Token::LesserEqual),
                '>' => self.push_if_next_ahead('=', Token::Greater, Token::GreaterEqual),
                '\'' => self.scan_char_lit(),
                '"' => self.scan_string(),
                _ => {
                    // Ignore any useless whitespace
                    if self.peek().is_whitespace() || self.peek() == '\t' || self.peek() == '\n' {
                        self.advance();
                    }
                    else if self.peek().is_ascii_alphabetic() {
                        self.scan_kw_or_ident();
                    }
                    else if self.peek().is_ascii_digit() {
                        self.scan_number();
                    }

                }
            }
        }

        self.tokens
    }
}