pub mod ast;
mod parser;
mod token;
mod asm_gen;

use std::env;
use std::fs;
use std::process::Command;

fn main() {
    // Read args and set flags
    let args: Vec<String> = env::args().collect();

    let mut out_asm_file: bool = false;
    let mut out_obj_file: bool = false;
    let mut display_help: bool = false;

    let mut out_file_name: &str = "a.out";

    for (i, arg) in args.iter().enumerate() {
        if arg == "-asm" { out_asm_file = true; }
        if arg == "-obj" { out_obj_file = true; }
        if arg == "-h" || arg == "-h" { display_help = true; }
        if arg == "-o" { out_file_name = &args[i+1] }
    }

    if display_help || args.len() == 1 {
        print_help();
    } 
    else {
        let filename = args[1].clone();
        match fs::read_to_string(filename) {

            Ok(raw_code) => {

                let ast = parser::parse_prog(token::tokenizer(&raw_code));

                // Output assembly to file
                match fs::write(&(out_file_name.to_owned() + ".s"), asm_gen::gen_asm(ast)) {
                    Ok(()) => {
                        // Run nasm
                        let nasm_status = Command::new("nasm")
                            .args(&["-f", "elf64", "-o", &(out_file_name.to_owned() + ".o"), &(out_file_name.to_owned() + ".s")])
                            .status()
                            .expect("nasm failed to start!");

                        // Run ld if nasm succeeded
                        if nasm_status.code().unwrap() == 0 {
                            let ld_status = Command::new("ld")
                                .args(&["-m", "elf_x86_64", "-o", out_file_name, &(out_file_name.to_owned() + ".o")])
                                .status()
                                .expect("ld failed!");

                            // Proceed if ld succeeded
                            if ld_status.code().unwrap() == 0 {

                                // Delete asm file if flag not set
                                if !out_asm_file {
                                    Command::new("rm")
                                        .arg(&(out_file_name.to_owned() + ".s"))
                                        .spawn()
                                        .expect("deleting assembly file failed");
                                }

                                // Delete obj file if flag not set
                                if !out_obj_file {
                                    Command::new("rm")
                                        .arg(&(out_file_name.to_owned() + ".o"))
                                        .spawn()
                                        .expect("deleting object file failed");
                                }
                            }
                        }
                    }

                    Err(_e) => { 
                        println!("Error writing to file {}", &(out_file_name.to_owned() + ".s")); 
                    }
                }
            }

            Err(_e) => { print_help(); }
        }
    }
}

fn print_help() {
    let help_str: &str = "OogaBooga compiler v1.0
        
usage: oogabooga <input file> [-asm] [-obj] [-o <output file>]
    
optional arguments:
    -h              display help
    -asm            output assembly file
    -obj            output object file
    -o              output file (the default is a.out)";

    println!("{}\n", help_str);
}
