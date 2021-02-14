pub use crate::ast::{Token, TokenType, StType};
use std::process;

// Parse function returns AST view of the program
// TODO: make nice verbose errors!
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
        eprintln!("Missing: < \nFound instead: {}", tok.tvalue);
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
        eprintln!("Missing: [ \nFound instead: {}", tok.tvalue);
        process::exit(1);
    }
    tok = token_iter.next().unwrap();

    // Create vector of Tokens and pass it to parse_func
    // for further processing of function body
    let mut token_func_body: Vec<Token> = Vec::new();

    // Loop until end of function
    // Ignore ] sign if another [ is opened
    let mut br_count: u32 = 1;

    while br_count != 0 {
        token_func_body.push(*tok);
        tok = token_iter.next().unwrap();

        if tok.ttype == TokenType::Lcb { br_count += 1; }
        else if tok.ttype == TokenType::Rcb { br_count -= 1; }
    }

    // DEBUG
    //println!("\n\nfunc_body:\n\n{:?}", token_func_body);
    let parsed_func_body: Vec<StType> = parse_func(token_func_body);

    branch.extend(parsed_func_body);
    
    branch
}

fn parse_func_params(tokens: Vec<Token>) {
    // Parse func parameters

}

fn parse_func(tokens: Vec<Token>) -> Vec<StType> {
    // Parse func body
    let mut func_ast: Vec<StType> = Vec::new();

    // Tokens to pass to parse_stat function
    let mut stat_tokens: Vec<Token> = Vec::new();

    // Separate tokens into statements
    // Create variable inside_block that keeps track of depth of code blocks []
    let mut inside_block: u32 = 0;

    for tok in tokens {
        if tok.ttype == TokenType::Lcb {

            inside_block += 1;
                
        } else if tok.ttype == TokenType::Rcb {

            inside_block -= 1;

        }

        // Gather all tokens until semicolon, then pass them to parse_stat
        // Also make sure to include all code blocks as a single statement
        if (tok.ttype != TokenType::Semicolon && tok.ttype != TokenType::Rcb)
            || inside_block != 0 {

            stat_tokens.push(tok);
        } else {
            // DEBUG
            //println!("\n\nstat_tokens\n{:?}", stat_tokens);
            func_ast.push(parse_stat(stat_tokens.clone()));
            stat_tokens.clear();
        }
    }

    func_ast
}

fn parse_stat(tokens: Vec<Token>) -> StType {

    let mut token_iter = tokens.iter();

    // tok will loop through all tokens
    //let mut tok: &Token = token_iter.next().unwrap();

    match token_iter.next().unwrap().ttype {

        // Statement starts with type
        TokenType::Type => {
            match token_iter.next().unwrap().ttype {

                // Next element is ID
                TokenType::ID => {
                    match token_iter.next() {
                        Some(t) => {

                            // ID followed by equal sign
                            if t.ttype == TokenType::Equal {

                                // Statement is Init unless type does not match
                                match token_iter.next().unwrap().ttype {
                                    TokenType::Char => StType::Init,
                                    TokenType::Str => StType::Init,
                                    TokenType::Int => StType::Init,
                                    TokenType::Bool => StType::Init,
                                    _ => {
                                        eprintln!("Error parsing init statement {:?}", tokens);
                                        process::exit(1);
                                    }
                                }
                            } else {
                                eprintln!("Expected: =\nIn: {:?}", tokens);
                                process::exit(1);
                            }
                        }

                        // If next element is empty it's a declare statement
                        None => { StType::Declare }
                    }
                }

                _ =>  {
                    eprintln!("Error element ID not found!");
                    process::exit(1);
                }
            }
        }

        // Assign statement
        TokenType::ID => {
            if token_iter.next().unwrap().ttype == TokenType::Equal {
                    
                match token_iter.next().unwrap().ttype {
                    TokenType::Char => StType::Assign,
                    TokenType::Str => StType::Assign,
                    TokenType::Int => StType::Assign,
                    TokenType::Bool => StType::Assign,
                    _ => {
                        eprintln!("Error parsing assign statement {:?}", tokens);
                        process::exit(1);
                    },
                }
            } else {
                // The statement starts with ID but is not Assign
                eprintln!("Error invalid statement {:?}", tokens);
                process::exit(1);
            }
        }

        // Print statement
        // Starts with print keyword followed by ( string )
        TokenType::Print => {
            if token_iter.next().unwrap().ttype == TokenType::Lb {
                if token_iter.next().unwrap().ttype == TokenType::Str {

                    StType::Print

                } else {
                    eprintln!("Missing string from print\nFound instead: {:?}", tokens);
                    process::exit(1);
                }
            } else {
                eprintln!("Missing: <\nFound: {:?}", tokens);
                process::exit(1);
            }
        }

        // Input statement
        TokenType::Input => {
            if token_iter.next().unwrap().ttype == TokenType::Rb {
                if token_iter.next().unwrap().ttype == TokenType::ID {
                    StType::Input
                } else {
                    eprintln!("Missing variable from input at: {:?}", tokens);
                    process::exit(1);
                }
            } else {
                eprintln!("Missing > from input at: {:?}", tokens);
                process::exit(1);
            }
        }

        // If statement
        TokenType::If => { StType::If }

        // While loop
        TokenType::While => { StType::While }

        // For loop
        TokenType::For => { StType::For }

        // Return statement
        TokenType::Ret => { StType::Return }

        _ => {
            eprintln!("Error invalid statement!\n{:?}", tokens);
            process::exit(1);
        }

    }
}
