// Type of lexeme
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenType {
    ID,
    Type,
    Char,
    Str,
    Int,
    Bool,
    Equal,
    Plus,
    Minus,
    Mult,
    Div,
    Mod,
    Lb,
    Rb,
    Lcb,
    Rcb,
    Ret,
    Semicolon,
}

// Lexeme with value
#[derive(Debug, Copy, Clone)]
pub struct Token<'a> {
    pub ttype: TokenType,
    pub tvalue: &'a str,
}

// Statement type
#[derive(Debug)]
pub enum StType {
    Assign,
    Print,
    If,
    While,
    For,
    Return,
}
