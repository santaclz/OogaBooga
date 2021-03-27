pub use crate::ast::{Token, TokenType, StType, Node};
use std::collections::HashMap;

pub fn gen_asm(ast: Vec<Node>) -> String {

    // Final code string
    let mut asm_code_str: String = String::new();
    
    // Start of file
    asm_code_str += "global _start

section .text

_start:
    call main;";

    // Exit syscall
    asm_code_str += "
    mov rdi, rax;
    mov rax, 60;
    syscall;";

    // main function
    // Prepeare stack for start of function
    asm_code_str += 
    "

main:
    push rbp;
    mov rbp,rsp;\n";

    // Loop through nodes and get all strings into str_rodata
    let mut str_rodata: Vec<&str> = Vec::new();

    for node in &ast {
        for tok in &node.svalue {
            if tok.ttype == TokenType::Str {
                str_rodata.push(tok.tvalue);
            }
        }
    }

    // Store number of variables in var_stack_count for calculating stack offsets
    // Use HashMap
    let mut var_stack_count: u32 = 4;
    let mut variable_offset_table = HashMap::new();

    for node in &ast {
        if node.stype == StType::InitInt ||
            node.stype == StType::InitVar ||
            node.stype == StType::InitChar ||
            node.stype == StType::InitBool ||
            node.stype == StType::InitStr ||
            node.stype == StType::Declare {

            // Get variable name
            for token in &node.svalue {
                if token.ttype == TokenType::ID {
                    variable_offset_table.insert(token.tvalue, var_stack_count);
                    break;
                }
            }
            
            var_stack_count += 4;
        }
    }

    // Loop through nodes and generate code
    for node in ast {
        asm_code_str += match node.stype {
            StType::AssignInt => assign_int_asm(node.svalue, &variable_offset_table),
            StType::AssignVar => assign_var_asm(node.svalue, &variable_offset_table),
            StType::InitInt => init_int_asm(node.svalue, &variable_offset_table),
            StType::Return => ret_asm(node.svalue),
            StType::Print => print_asm(node.svalue, &str_rodata),
            _ => "".to_string()
        }.as_str();
    }

    // Check if str_rodata vector is empty, if not append .rodata section
    if !str_rodata.is_empty() {

        // db "string",10 -> string with newline char
        asm_code_str += "\nsection .rodata";
        for (i, rostr) in str_rodata.iter().enumerate() {
            asm_code_str += &format!("
    {0}: db {1},10
    {0}len equ $-{0}", format!("{}{}", "msg", i.to_string().as_str()), rostr);
        }
    }

    asm_code_str
}

fn assign_var_asm<'a>(tokens: Vec<Token<'a>>, var_off_table: &'a HashMap<&'a str, u32>) -> String {

    // Get value from assign statement
    let var1 = &tokens[2].tvalue;

    // Get variable name
    let var2 = &tokens[0].tvalue;

    // Move value on stack
    let ret_str = format!("    mov eax,dword [rbp-{}];
    mov dword [rbp-{}],eax;\n", var_off_table[var1], var_off_table[var2]);
    ret_str
}

fn assign_int_asm<'a>(tokens: Vec<Token<'a>>, var_off_table: &'a HashMap<&'a str, u32>) -> String {

    // Get value from assign statement
    let val = &tokens[2].tvalue;

    // Get variable name
    let var = &tokens[0].tvalue;

    // Move value on stack
    let ret_str = format!("    mov dword [rbp-{}],{};\n", var_off_table[var], val);
    ret_str
}

fn init_int_asm<'a>(tokens: Vec<Token<'a>>, var_off_table: &'a HashMap<&'a str, u32>) -> String {

    // Get value from init statement
    let val = tokens[3].tvalue;

    // Get variable name
    let var = &tokens[1].tvalue;

    // Move value on stack
    let ret_str = format!("    mov dword [rbp-{}],{};\n", var_off_table[var], val);
    ret_str
}

fn ret_asm(tokens: Vec<Token>) -> String {

    // Get value from return statement
    let val = tokens[1].tvalue;

    // Move value on stack
    format!("    mov eax,{};
    pop rbp;
    ret;\n", val)
}

fn print_asm(tokens: Vec<Token>, str_rodata: &Vec<&str>) -> String {

    // Get value from print statement
    let val = tokens[2].tvalue;

    // Calculate names for string variable (msg0, msg1, msg2...)
    let mut val_name: String = String::new();

    for (i, rostr) in str_rodata.iter().enumerate() {
        if val == *rostr {
            val_name = format!("msg{}",i);
        }
    }

    // Move value on stack
    format!("    mov rax,1;
    mov rdi,1;
    mov rsi,{0};
    mov rdx, {0}len;
    syscall;\n", val_name)
}
