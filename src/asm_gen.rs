pub use crate::ast::{Token, TokenType, StType, Node};

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
    mov rax, 60;
    mov rdi, 0;
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
    // TODO if reassigning are equal dont increment
    let mut var_stack_count: u32 = 4;

    for node in &ast {
        if node.stype == StType::Init {
            var_stack_count += 4;
        }
    }

    // Loop through nodes and generate code
    for node in ast {
        asm_code_str += match node.stype {
            StType::Assign => assign_asm(node.svalue, &mut var_stack_count),
            StType::Init => init_asm(node.svalue, &mut var_stack_count),
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

fn assign_asm(tokens: Vec<Token>, var_stack_count: &mut u32) -> String {

    // Get value from assign statement
    let val = &tokens[2].tvalue;

    // Move value on stack
    let ret_str = format!("    mov dword [rbp-{}],{};\n", *var_stack_count, val);
    *var_stack_count -= 4;
    ret_str
}

fn init_asm(tokens: Vec<Token>, var_stack_count: &mut u32) -> String {

    // Get value from init statement
    let val = tokens[3].tvalue;

    // Move value on stack
    let ret_str = format!("    mov dword [rbp-{}],{};\n", *var_stack_count, val);
    *var_stack_count -= 4;
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
