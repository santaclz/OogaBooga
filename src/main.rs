pub mod ast;
mod parser;
mod token;
mod asm_gen;

use std::env;
use std::fs;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args[1].clone();
    let raw_code = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let ast = parser::parse_prog(token::tokenizer(&raw_code));

    //println!("Raw code:\n\n{}", raw_code);
    //println!("Tokens:\n{:?}\n", token::tokenizer(&raw_code));
    //println!("AST:\n{:?}", ast);
    //println!("Assembly x86-64:\n\n{}", asm_gen::gen_asm(ast));

    // Output assembly to file
    fs::write("prog.s", asm_gen::gen_asm(ast));

    // Run nasm
    let nasm_status = Command::new("nasm")
        .args(&["-felf64", "-o prog.o", "prog.s"])
        .status()
        .expect("nasm failed to start!");

    // Run ld if nasm succeeded
    if nasm_status.code().unwrap() == 0 {
        Command::new("ld")
            .args(&["-melf_x86_64", "-o", "prog", "prog.o"])
            .spawn()
            .expect("ld failed!");
    }
}
