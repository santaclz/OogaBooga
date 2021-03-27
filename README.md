# About

Compiler written in Rust for my programming language called OogaBooga.

# Why?

The purpose of this project is to learn how compiler works and to get better at understanding assembly code. Also, this is my first Rust program, I just jumped straight into making a compiler with it, so far it's a great learning experience. For the sake of learning I will not be using any dependencies which would aid me in parsing the source code (RegEx, other parsers...).

## Platform

GNU/Linux x86_64

## Requirements

- nasm	= assembler
- ld	= linker
- rust	= language im working in
- cargo	= for compiling rust projects

# About OogaBooga

OogaBooga syntax is similar to C. There are no function types, only labels.

### Data types
```
num 	= signed int
bool 	= boolean (yes or no)
str 	= string
chr 	= char
```
### I/O commands
```
say	= prints to screen
eat	= takes input and saves it to a variable
```
### Statements
```
num x;			= declaration

bool a = yes;		= initialization

x = value;		= assignment

if condition [...]	= if statement

while condition [...]	= while loop

ret value;		= return statement
```
### Brackets
```
Square brackets [] are used for function and statement body 
and greater than and less than signs <> for parameters.
```
### Program structure
```
main [
	# your code here

	ret 0;
]
```

# Steps

1. Read in the file
2. Write lexer that takes code and returns list of lexemes (tokens)
3. Define Abstract syntax tree (AST)
4. Write parser that takes tokens and returns AST (and also throws compile errors)
5. Generate code for given AST in assembly (x86_64)
6. Write assembly to file
7. Use NASM to convert assembly to an object file and `ld -m elf_x86_64` to executable

# Lexer

Lexer recognizes tokens from supplied file and assigns them their type.
Ex. `say < "Ooga Booga";` is consisted from tokens: `Print`, `Lc`, `Str`, `Semicolon`.

# Parser

Parser separates the function body and recognizes which type of statements are inside it. Its goal is to group tokens from Lexer into statements.
Ex. `[Print, Lc, Str, Semicolon]` should be recognized as a print statement. This is where I implemented checking for compile errors.

# Abstract Syntax Tree

AST (for short) is a way of representing structure of source code written in programming language.
To implement it I simply created struct `Node` which holds information about a single statement (its type, tokens, value). Then I created a vector which holds all `Node`s which is my AST created from supplied source code.

# Code generation

The program goes through AST (vector of `Node`s) and for each statement generates its equivalent assembly code.

## Assembly

Before generating assembly code we need to make sure that the program exits correctly. For that purpose I'm prepending this piece of code. 
```
global _start
section .text

_start:
    call main;
    mov rdi, rax;
    mov rax, 60;
    syscall;
```
First we define a global directive `_start` which is needed for linker (ld) to know where the program starts. Then we define start of section `.text` which is used to store code.
Then in function `_start` we call main and once it finishes we make a syscall to exit the program (exit is called by placing 60 in rax register and exit status in rdi).

# Help
```
$ ./oogabooga -h
OogaBooga compiler v1.0
        
usage: oogabooga <input file> [-asm] [-obj] [-o <output file>]
    
optional arguments:
    -h              display help
    -asm            output assembly file
    -obj            output object file
    -o              output file (the default is a.out)

```

# Program output

```
$ ./oogabooga examples/simple_assembly.gg -asm

Raw code:

main [
	num a;
	a = 1;
	num b = 2;
	say < "Ooga Booga!";
	say < "Hello World!";

	ret 0;
]

Assembly x86-64:

global _start

section .text

_start:
    call main;
    mov rdi, rax;
    mov rax, 60;
    syscall;

main:
    push rbp;
    mov rbp,rsp;
    mov dword [rbp-4],1;
    mov dword [rbp-8],2;
    mov rax,1;
    mov rdi,1;
    mov rsi,msg0;
    mov rdx, msg0len;
    syscall;
    mov rax,1;
    mov rdi,1;
    mov rsi,msg1;
    mov rdx, msg1len;
    syscall;
    mov eax,0;
    pop rbp;
    ret;

section .rodata
    msg0: db "Ooga Booga!",10
    msg0len equ $-msg0
    msg1: db "Hello World!",10
    msg1len equ $-msg1
```

# Executing the code

To execute the program run:
```
$ ./a.out
```
For program in `examples/simple_assembly.gg` you should get output:
```
Ooga Booga!
Hello World!
```
