mod token;
mod ast;

pub fn parse(tokens: Vec<Token>) {
    let mut branch: Vec<enum>;

    // Parse tokens outside functions and pass others to parse_func
    let mut tokens_iter = tokens.iter();

    if tokens_iter.next() != Token::Type {

        eprintln!("Error parsing type of function");
        process.exit(1);

    } else {
    
       branch.push(Func::Type); 
    }
}

fn parse_func(tokens: Vec<Token>) {
    // Parse func body

}
