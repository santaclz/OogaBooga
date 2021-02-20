pub mod ast;
mod parser;
mod token;
mod asm_gen;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args[1].clone();
    let raw_code = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let ast = parser::parse_prog(token::tokenizer(&raw_code));

    println!("Raw code:\n\n{}", raw_code);
    //println!("Tokens:\n{:?}\n", token::tokenizer(&raw_code));
    //println!("AST:\n{:?}", ast);
    println!("Assembly x86-64:\n\n{}", asm_gen::gen_asm(ast));
}
