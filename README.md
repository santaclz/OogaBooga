# About

Compiler written in Rust for my programming language called OogaBooga.

# Why?

The purpose of this project is to learn how compiler works and to get better at understanding assembly code. Also, this is my first Rust program, I just jumped straight into making a compiler with it, so far it's a great learning experience.

# About OogaBooga

OogaBooga syntax is very similar to C.

### Data types
```
numba 	= signed int
hungry 	= boolean
rock 	= string
stone 	= char
```
### I/O commands
```
shout	= prints to screen
eat	= takes input and saves it to a variable
```
### Statements
```
if			= if
roll c in r finger i	= for c in r (with i as index) -> loops through string similar to python
spin			= while loop
begone			= return
```
### Brackets
```
Square brackets [] are used instead of braces {}
and greater than and less than signs <> instead of parentheses ().
```

# Steps

1. Read in the file
2. Write lexer that takes code and returns list of lexemes (tokens)
3. Define Abstract syntax tree (AST)
4. Write parser that takes tokens and returns AST (and also throws compile errors)
5. Generate code for given AST in assembly (x86_64)
6. Write assembly to file
7. Use NASM to convert assembly to an object file and `ld -m elf_x86_64` to executable

# Current progress

Currently the program is able to recognize tokens from supplied file and separate which of them are part of a function body (Step 2). It then parses the function body and recognizes which type of statements are inside it. 
Then it creates vector of structs Node (Step 4). Struct Node contains three fields: stype (statement type), svalue (tokens) and sbody (if statement has a block of code like if, for or while statement, then content of that code block is stored here). 
The program then loops through that vector and generates assembly code (Step 5). I'm currently working on translating all OogaBooga statements into assembly code. Then I will implement loops and functions.

Program output:

```
$ cargo run examples/assembly/simple_assembly.ga

Raw code:

numba main<> [

	numba a;
	a = 1;
	numba b = 2;

	begone 0;
]

Assembly x86-64:

global _start
    section .text

    _start:
        push rbp;
        mov rbp,rsp;
    	mov dword [rbp-8],1;
	mov dword [rbp-4],2;
	mov eax,0;
        pop rbp;
        ret;
```
