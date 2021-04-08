#[derive(Debug)]
pub enum Token {
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Dot,
    Comma,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Colon,
    Semicolon,
    Not,
    Equals,
    EqualsEquals,
    NotEquals,
    Less,
    Greater,
    LesserEqual,
    GreaterEqual,

    // Reserved Keywords
    Case,
    Const,
    Def,
    If,
    Elif,
    Else,
    For,
    Fn,
    While,

    TypeInt,
    TypeBool,
    TypeChar,
    TypeStr,

    // Types
    Identifier(String),
    IntLit(i32),
    StringLit(String),
    CharLit(char)
}