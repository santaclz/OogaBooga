pub use crate::ast::{Token, TokenType, StType, Node};
use std::process;

// Parse function returns AST view of the program
pub fn parse_prog(tokens: Vec<Token>) -> Vec<Node> {

    let mut branch: Vec<Node> = Vec::new();

    // Parse tokens outside functions and pass others to parse_block
    // TODO: ^
    
    let mut token_iter = tokens.iter();

    // Check for errors

    // tok will loop through all tokens
    let mut tok: &Token = token_iter.next().unwrap();
    
    // Start of program must be function name
    if tok.ttype != TokenType::ID {
        eprintln!("Invalid function name {}", tok.tvalue);
        process::exit(1);
    }
    let func_name = tok.tvalue;
    tok = token_iter.next().unwrap();

    if tok.ttype != TokenType::Lcb {
        eprintln!("Missing [ after function name.\nFound instead: {}", tok.tvalue);
        process::exit(1);
    }
    tok = token_iter.next().unwrap();

    // Create vector of Tokens and pass it to parse_block
    // for further processing of function body
    let mut token_func_body: Vec<Token> = Vec::new();

    // Loop until end of function
    // Ignore ] sign if another [ is opened
    let mut br_count: u32 = 1;

    while br_count != 0 {
        token_func_body.push(*tok);
        //tok = token_iter.next().unwrap();
        match token_iter.next() {
            Some(i) => tok = i,
            None    => { eprintln!("Missing ] at the end of function {}.", func_name); process::exit(1); }
        }

        if tok.ttype == TokenType::Lcb { br_count += 1; }
        else if tok.ttype == TokenType::Rcb { br_count -= 1; }
    }

    // DEBUG
    //println!("\n\nfunc_body:\n\n{:?}", token_func_body);
    let parsed_func_body: Vec<Node> = parse_block(token_func_body);

    // Check if the last statement is return
    if parsed_func_body.len() != 0 && parsed_func_body[parsed_func_body.len() - 1].stype != StType::Return {
        eprintln!("Missing return statement at the end of function {}.", func_name);
        process::exit(1);
    }

    branch.extend(parsed_func_body);
    
    branch
}

// Parse func parameters
// TODO
/*
fn parse_func_params(tokens: Vec<Token>) {

}
*/

// Parse func body : turn tokens into nodes
fn parse_block(tokens: Vec<Token>) -> Vec<Node> {

    let mut func_ast: Vec<Node> = Vec::new();

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

            // Include ]
            if tok.ttype == TokenType::Rcb {
                stat_tokens.push(tok);
            } 
            // Include semicolon
            else if tok.ttype == TokenType::Semicolon {
                stat_tokens.push(tok);
            }
            
            // DEBUG
            //println!("\n\nstat_tokens\n{:?}", stat_tokens);
            func_ast.push(parse_stat(stat_tokens.clone()));
            stat_tokens.clear();
        }
    }

    func_ast
}

// Parse single statement
// For parsing loops, parse_stat will call itself recursively
fn parse_stat(tokens: Vec<Token>) -> Node {

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
                                    TokenType::Char => Node::new(StType::InitChar, tokens),
                                    TokenType::Str => Node::new(StType::InitStr, tokens),

                                    TokenType::Int => Node::new(StType::InitInt, tokens),
                                    TokenType::Minus => Node::new(StType::InitInt, tokens),

                                    TokenType::Bool => Node::new(StType::InitBool, tokens),
                                    _ => {
                                        pretty_print_errors("Error parsing init statement".to_string(), &tokens);
                                        process::exit(1);
                                    }
                                }
                            } else {
                                pretty_print_errors("Expected: =\nIn:".to_string(), &tokens);
                                process::exit(1);
                            }
                        }

                        // If next element is empty it's a declare statement
                        None => Node::new(StType::Declare, tokens)
                    }
                }

                _ =>  {
                    eprintln!("Error element ID not found!");
                    process::exit(1);
                }
            }
        }

        // Assign statements
        TokenType::ID => {
            let token_iter_tmp = match token_iter.next() {
                Some(t) => t,
                None    => { pretty_print_errors("Statement not recognized:".to_string(), &tokens); process::exit(1); }
            };
            if token_iter_tmp.ttype == TokenType::Equal {
                    
                match token_iter.next().unwrap().ttype {
                    TokenType::Char => Node::new(StType::AssignChar, tokens),
                    TokenType::Str => Node::new(StType::AssignStr, tokens),

                    TokenType::Int => Node::new(StType::AssignInt, tokens),
                    TokenType::Minus => Node::new(StType::AssignInt, tokens),

                    TokenType::Bool => Node::new(StType::AssignBool, tokens),
                    TokenType::ID => Node::new(StType::AssignVar, tokens),
                    _ => {
                        pretty_print_errors("Error parsing assign statement".to_string(), &tokens);
                        process::exit(1);
                    },
                }
            } else {
                // The statement starts with ID but is not Assign
                pretty_print_errors("Error invalid statement".to_string(), &tokens);
                process::exit(1);
            }
        }

        // Print statement
        // Starts with print keyword followed by ( string )
        TokenType::Print => {
            if token_iter.next().unwrap().ttype == TokenType::Lb {

                // Create pointer for iter as same value needs to pass multiple checks
                let tok: &Token = token_iter.next().unwrap();

                if tok.ttype == TokenType::Str || tok.ttype == TokenType::ID {

                    Node::new(StType::Print, tokens)

                } else {
                    pretty_print_errors("Missing string from print\nFound instead:".to_string(), &tokens);
                    process::exit(1);
                }
            } else {
                pretty_print_errors("Missing: <\nFound:".to_string(), &tokens);
                process::exit(1);
            }
        }

        // Input statement
        TokenType::Input => {
            if token_iter.next().unwrap().ttype == TokenType::Rb {
                if token_iter.next().unwrap().ttype == TokenType::ID {
                    Node::new(StType::Input, tokens)
                } else {
                    pretty_print_errors("Missing variable from input at:".to_string(), &tokens);
                    process::exit(1);
                }
            } else {
                pretty_print_errors("Missing > from input at:".to_string(), &tokens);
                process::exit(1);
            }
        }

        // If statement
        TokenType::If => { 
            // Slice tokens into cond and body
            let mut scond: Vec<Token> = Vec::new();
            let mut sbody: Vec<Token> = Vec::new();

            // Put all cond tokens into scond
            let mut tok: &Token = token_iter.next().unwrap();
            let mut toko: Option<&Token> = Some(tok);

            // Gather all tokens until start of code block
            while tok.ttype != TokenType::Lcb {
                
                match toko {
                    Some(t) => {
                        scond.push(*t);
                        toko = token_iter.next();

                        match toko {
                            Some(tt)    => { tok = tt; }
                            None        => {
                                // Error if opening bracket not found
                                pretty_print_errors("Error missing ] on:".to_string(), &tokens);
                                process::exit(1);
                            }
                        }
                    }
                    
                    None => { 
                        // Missing condition from if
                        pretty_print_errors("Error missing condition on:".to_string(), &tokens);
                        process::exit(1);
                    }
                }
            }

            // Skip open bracket
            tok = token_iter.next().unwrap();

            // Put all others into sbody
            let mut br_count: u32 = 1;
            
            // If next element is closed bracket don't enter the while loop
            if tok.ttype == TokenType::Rcb {
                br_count = 0;
            }

            while br_count != 0 {
                sbody.push(*tok);
                tok = token_iter.next().unwrap();

                if tok.ttype == TokenType::Lcb { br_count += 1; }
                else if tok.ttype == TokenType::Rcb { br_count -= 1; }
            }

            // Parse sbody into Vec<Node>
            Node::new_block(StType::If, scond, Some(parse_block(sbody)))
        }

        // While loop
        TokenType::While => { 
            // Slice tokens into cond and body
            let mut scond: Vec<Token> = Vec::new();
            let mut sbody: Vec<Token> = Vec::new();

            // Put all cond tokens into scond
            let mut tok: &Token = token_iter.next().unwrap();
            let mut toko: Option<&Token> = Some(tok);

            // Gather all tokens until start of code block
            while tok.ttype != TokenType::Lcb {
                
                match toko {
                    Some(t) => {
                        scond.push(*t);
                        toko = token_iter.next();

                        match toko {
                            Some(tt)    => { tok = tt; }
                            None        => {
                                // Error if opening bracket not found
                                pretty_print_errors("Error missing ] on:".to_string(), &tokens);
                                process::exit(1);
                            }
                        }
                    }
                    
                    None => { 
                        // Missing condition from if
                        pretty_print_errors("Error missing condition on:".to_string(), &tokens);
                        process::exit(1);
                    }
                }
            }

            // Skip open bracket
            tok = token_iter.next().unwrap();

            // Put all others into sbody
            let mut br_count: u32 = 1;
            
            // If next element is closed bracket don't enter the while loop
            if tok.ttype == TokenType::Rcb {
                br_count = 0;
            }

            while br_count != 0 {
                sbody.push(*tok);
                tok = token_iter.next().unwrap();

                if tok.ttype == TokenType::Lcb { br_count += 1; }
                else if tok.ttype == TokenType::Rcb { br_count -= 1; }
            }

            // Parse sbody into Vec<Node>
            Node::new_block(StType::While, scond, Some(parse_block(sbody)))
        }

        // Return statement
        TokenType::Ret => {
            let tok: Option<&Token> = token_iter.next();
            
            match tok {
                Some(_t) => Node::new(StType::Return, tokens),
                None => {
                    pretty_print_errors("Error no value supplied to return statement on:".to_string(), &tokens);
                    process::exit(1);
                }
            }
        }

        _ => {
            pretty_print_errors("Error invalid statement!".to_string(), &tokens);
            process::exit(1);
        }

    }
}

fn pretty_print_errors(err_str: String, tokens: &Vec<Token>) {
    eprintln!("{}", err_str);

    for tok in tokens {
        eprint!("{} ", tok.tvalue);
    }
    eprintln!("");
}
