#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
    // single character tokens
    /// (
    LeftParen,
    /// )
    RightParen,
    /// {
    LeftBrace,
    /// }    
    RightBrace,
    /// ,
    Comma,
    Dot,
    Minus,
    Plus,
    // ;
    SemiColon,
    Slash,
    Star,

    // one or two character tokens
    // !
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literals
    Identifier,
    String,
    Number,

    // keywords
    And,
    Class,
    Else,
    False,
    Func,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}
