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

    for node in ast {
        asm_code_str += match node.stype {
            StType::Assign => assign_asm(node.svalue),
            StType::Init => init_asm(node.svalue),
            StType::Return => ret_asm(node.svalue),
            _ => "".to_string()
        }.as_str();
    }

    asm_code_str
}

fn assign_asm(tokens: Vec<Token>) -> String {
    let mut asm_code: String = String::new();

    // Get value from assign statement
    let val = tokens[2].tvalue;

    // Move value on stack
    // TODO manage stack and calcuate offsets!
    asm_code = format!("    mov dword [rbp-{}],{};\n", 0x8, val);

    asm_code
}

fn init_asm(tokens: Vec<Token>) -> String {
    let mut asm_code: String = String::new();

    // Get value from assign statement
    let val = tokens[3].tvalue;

    // Move value on stack
    asm_code = format!("    mov dword [rbp-{}],{};\n", 0x4, val);

    asm_code
}

fn ret_asm(tokens: Vec<Token>) -> String {
    let mut asm_code: String = String::new();

    // Get value from assign statement
    let val = tokens[1].tvalue;

    // Move value on stack
    asm_code = format!("    mov eax,{};
    pop rbp;
    ret;\n", val);

    asm_code
}
