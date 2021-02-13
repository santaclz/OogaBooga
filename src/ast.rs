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
    Declare,    // string s;
    Init,       // int a = 3;
    Assign,     // a = 3;
    Print,      // print("hello");
    If,         // if (true) { }
    While,      // while (true) { }
    For,        // for (int i = 0; i < n; i++) { }
    Return,     // return 2;
}
