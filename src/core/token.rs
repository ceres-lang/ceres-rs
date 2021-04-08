pub enum Token {
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Dot,
    Comma,
    Colon,
    Semicolon,
    Not,
    Equals,
    EqualsEquals,
    NotEquals,

    // Reserved Keywords
    Case,
    Const,
    Def,
    If,
    Elif,
    Else,
    For,
    Fn,

    // Types
    IntLit(i32),
    StringLit(String),
    CharLit(char)
}