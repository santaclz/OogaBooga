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
#[derive(Debug, PartialEq)]
pub enum StType {
    Declare,    // rock s;
    Init,       // numba a = 3;
    Assign,     // a = 3;
    Print,      // shout < "hello";
    Input,      // eat > a;
    If,         // if true [ ]
    While,      // spin true [ ]
    Return,     // begone 2;
}

// Statement with its type and vector of Tokens
// Loops contain vector of Nodes
#[derive(Debug)]
pub struct Node<'a> {
    pub stype: StType,
    pub svalue: Vec<Token<'a>>,
    pub sbody: Option<Vec<Node<'a>>>,
}

impl<'a> Node<'a> {
    // Create new note without body
    pub fn new(typ: StType, val: Vec<Token<'a>>) -> Self {
        Node {
            stype: typ,
            svalue: val,
            sbody: None,
        }
    }

    // Create new node with body
    pub fn new_block(typ: StType, val: Vec<Token<'a>>, body: Option<Vec<Node<'a>>>) -> Self {
        Node {
            stype: typ,
            svalue: val,
            sbody: body,
        }
    }
}
