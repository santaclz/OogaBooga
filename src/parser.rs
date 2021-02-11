pub use crate::ast::{Token, TokenType, StType};
use std::process;

// Parse function returns AST view of the program
pub fn parse_prog(tokens: Vec<Token>) -> Vec<StType> {

    let mut branch: Vec<StType> = Vec::new();

    // Parse tokens outside functions and pass others to parse_func
    // TODO: ^
    
    let mut token_iter = tokens.iter();

    // Check for errors

    // tok will loop through all tokens
    let mut tok: &Token = token_iter.next().unwrap();

    // Beginning of program must be function type
    if tok.ttype != TokenType::Type {
        eprintln!("Error parsing type of function {}", tok.tvalue);
        process::exit(1);
    }
    tok = token_iter.next().unwrap();
    
    // Next must follow function name
    if tok.ttype != TokenType::ID {
        eprintln!("Invalid function name {}", tok.tvalue);
        process::exit(1);
    }
    tok = token_iter.next().unwrap();

    // Next must follow left bracket
    if tok.ttype != TokenType::Lb {
        eprintln!("Missing: ( \nFound instead: {}", tok.tvalue);
        process::exit(1);
    }

    // Skip until function starts
    // TODO: implement parse_func_params
    while tok.ttype != TokenType::Rb {
        tok = token_iter.next().unwrap();
    }
    tok = token_iter.next().unwrap();

    // Check for start of func body
    if tok.ttype != TokenType::Lcb {
        eprintln!("Missing: {{ \nFound instead: {}", tok.tvalue);
        process::exit(1);
    }
    tok = token_iter.next().unwrap();

    // Create vector of Tokens and pass it to parse_func
    // for further processing of function body
    let mut token_func_body: Vec<Token> = Vec::new();

    // Loop until end of function
    // TODO: implement ignore } sign if another { is opened
    while tok.ttype != TokenType::Rcb {
        token_func_body.push(*tok);
        tok = token_iter.next().unwrap();
    }

    println!("\n\nfunc_body:\n\n{:?}", token_func_body);
    let parsed_func_body: Vec<StType> = parse_func(token_func_body);

    // Dummy code to avoid errors TEMPORARY!
    //branch.push(StType::Assign);

    branch.extend(parsed_func_body);
    
    branch
}

fn parse_func_params(tokens: Vec<Token>) {
    // Parse func parameters

}

fn parse_func(tokens: Vec<Token>) -> Vec<StType> {
    // Parse func body
    let mut func_ast: Vec<StType> = Vec::new();

    func_ast
}
