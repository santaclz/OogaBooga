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
    Print,
    Input,
    If,
    While,
    For,
    Semicolon,
}

// Lexeme with value
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Token<'a> {
    pub ttype: TokenType,
    pub tvalue: &'a str,
}

// Statement type
#[derive(Debug)]
pub enum StType {
    Declare,    // str s;
    Init,       // num a = 3;
    Assign,     // a = 3;
    Print,      // out < "hello";
    Input,      // in > a;
    If,         // if true [ ]
    While,      // while true [ ]
    For,        // for c in s [ ]
    Return,     // exit 2;
}
