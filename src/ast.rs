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
    Complement,
    Lb,
    Rb,
    Lcb,
    Rcb,
    Ret,
    Print,
    Input,
    If,
    While,
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
    Declare,    // chr s;

    InitInt,    // num a = 3;
    InitVar,    // num b = a;
    InitChar,   // chr a = 'A';
    InitBool,   // bool a = no;
    InitStr,    // str a = "hello";

    ExprInitInt,    // num a = 3 + 2;
    ExprInitVar,    // num a = b + c;
    ExprInitChar,   // chr a = 'A' + 'B';
    ExprInitBool,   // bool a = no & yes;
    ExprInitStr,    // str a = "hello" + " world";

    AssignInt,  // a = 3;
    AssignVar,  // a = b;
    AssignChar, // a = 'A';
    AssignBool, // a = yes;
    AssignStr,  // a = "hello";

    ExprAssignInt,  // a = 3 + 2;
    ExprAssignVar,  // a = b + c;
    ExprAssignChar, // a = 'A' + 'B';
    ExprAssignBool, // a = no & yes;
    ExprAssignStr,  // a = "hello" + " world";

    Print,      // say < "hello";
    Input,      // eat > a;
    If,         // if true [ ]
    While,      // while true [ ]
    Return,     // ret 2;
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
