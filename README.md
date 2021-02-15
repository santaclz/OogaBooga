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
7. Use GCC to convert assembly to an executable

# Current progress

Currently the program is able to recognize tokens from supplied file and separate which of them are part of a function body.
It then parses the function body and recognizes which type of statements are inside it. I'm currently working on step 4.

Inside examples/simple_exp.ga

```
$ cat examples/simple_exp.ga 
numba main <numba env1, numba env2> 
[
	shout < "Ooga Booga! 2+2 is ";
	numba a;
	eat > a;

	numba b;
	b=4;

	rock r = "banana";
	stone s = 'b';
	hungry food = yes;
	
	roll c in r finger i [
		shout < i;
	]

	spin b == 0 [
		begone 1;
	]

	if food [
		shout < "4";
	]
	
	begone 0;
]
```

Program output:

```
$ cargo run examples/simple_exp.ga

Tokens:
[Token { ttype: Type, tvalue: "numba" }, Token { ttype: ID, tvalue: "main" }, Token { ttype: Lb, tvalue: "<" }, Token { ttype: Type, tvalue: "numba" }, Token { ttype: ID, tvalue: "env1" }, Token { ttype: Type, tvalue: "numba" }, Token { ttype: ID, tvalue: "env2" }, Token { ttype: Rb, tvalue: ">" }, Token { ttype: Lcb, tvalue: "[" }, Token { ttype: Print, tvalue: "shout" }, Token { ttype: Lb, tvalue: "<" }, Token { ttype: Str, tvalue: "\"Ooga Booga! 2+2 is \"" }, Token { ttype: Semicolon, tvalue: ";" }, Token { ttype: Type, tvalue: "numba" }, Token { ttype: ID, tvalue: "a" }, Token { ttype: Semicolon, tvalue: ";" }, Token { ttype: Input, tvalue: "eat" }, Token { ttype: Rb, tvalue: ">" }, Token { ttype: ID, tvalue: "a" }, Token { ttype: Semicolon, tvalue: ";" }, Token { ttype: Type, tvalue: "numba" }, Token { ttype: ID, tvalue: "b" }, Token { ttype: Semicolon, tvalue: ";" }, Token { ttype: ID, tvalue: "b" }, Token { ttype: Equal, tvalue: "=" }, Token { ttype: Int, tvalue: "4" }, Token { ttype: Semicolon, tvalue: ";" }, Token { ttype: Type, tvalue: "rock" }, Token { ttype: ID, tvalue: "r" }, Token { ttype: Equal, tvalue: "=" }, Token { ttype: Str, tvalue: "\"banana\"" }, Token { ttype: Semicolon, tvalue: ";" }, Token { ttype: Type, tvalue: "stone" }, Token { ttype: ID, tvalue: "s" }, Token { ttype: Equal, tvalue: "=" }, Token { ttype: Char, tvalue: "\'b\'" }, Token { ttype: Semicolon, tvalue: ";" }, Token { ttype: Type, tvalue: "hungry" }, Token { ttype: ID, tvalue: "food" }, Token { ttype: Equal, tvalue: "=" }, Token { ttype: Bool, tvalue: "yes" }, Token { ttype: Semicolon, tvalue: ";" }, Token { ttype: For, tvalue: "roll" }, Token { ttype: ID, tvalue: "c" }, Token { ttype: ID, tvalue: "in" }, Token { ttype: ID, tvalue: "r" }, Token { ttype: ID, tvalue: "finger" }, Token { ttype: ID, tvalue: "i" }, Token { ttype: Lcb, tvalue: "[" }, Token { ttype: Print, tvalue: "shout" }, Token { ttype: Lb, tvalue: "<" }, Token { ttype: ID, tvalue: "i" }, Token { ttype: Semicolon, tvalue: ";" }, Token { ttype: Rcb, tvalue: "]" }, Token { ttype: While, tvalue: "spin" }, Token { ttype: ID, tvalue: "b" }, Token { ttype: Equal, tvalue: "=" }, Token { ttype: Equal, tvalue: "=" }, Token { ttype: Int, tvalue: "0" }, Token { ttype: Lcb, tvalue: "[" }, Token { ttype: Ret, tvalue: "begone" }, Token { ttype: Int, tvalue: "1" }, Token { ttype: Semicolon, tvalue: ";" }, Token { ttype: Rcb, tvalue: "]" }, Token { ttype: If, tvalue: "if" }, Token { ttype: ID, tvalue: "food" }, Token { ttype: Lcb, tvalue: "[" }, Token { ttype: Print, tvalue: "shout" }, Token { ttype: Lb, tvalue: "<" }, Token { ttype: Equal, tvalue: "=" }, Token { ttype: Int, tvalue: "4" }, Token { ttype: Semicolon, tvalue: ";" }, Token { ttype: Semicolon, tvalue: ";" }, Token { ttype: Rcb, tvalue: "]" }, Token { ttype: Ret, tvalue: "begone" }, Token { ttype: Int, tvalue: "0" }, Token { ttype: Semicolon, tvalue: ";" }, Token { ttype: Rcb, tvalue: "]" }]

AST:
[Node { stype: Print, svalue: [Token { ttype: Print, tvalue: "shout" }, Token { ttype: Lb, tvalue: "<" }, Token { ttype: Str, tvalue: "\"Ooga Booga! 2+2 is \"" }] }, Node { stype: Declare, svalue: [Token { ttype: Type, tvalue: "numba" }, Token { ttype: ID, tvalue: "a" }] }, Node { stype: Input, svalue: [Token { ttype: Input, tvalue: "eat" }, Token { ttype: Rb, tvalue: ">" }, Token { ttype: ID, tvalue: "a" }] }, Node { stype: Declare, svalue: [Token { ttype: Type, tvalue: "numba" }, Token { ttype: ID, tvalue: "b" }] }, Node { stype: Assign, svalue: [Token { ttype: ID, tvalue: "b" }, Token { ttype: Equal, tvalue: "=" }, Token { ttype: Int, tvalue: "4" }] }, Node { stype: Init, svalue: [Token { ttype: Type, tvalue: "rock" }, Token { ttype: ID, tvalue: "r" }, Token { ttype: Equal, tvalue: "=" }, Token { ttype: Str, tvalue: "\"banana\"" }] }, Node { stype: Init, svalue: [Token { ttype: Type, tvalue: "stone" }, Token { ttype: ID, tvalue: "s" }, Token { ttype: Equal, tvalue: "=" }, Token { ttype: Char, tvalue: "\'b\'" }] }, Node { stype: Init, svalue: [Token { ttype: Type, tvalue: "hungry" }, Token { ttype: ID, tvalue: "food" }, Token { ttype: Equal, tvalue: "=" }, Token { ttype: Bool, tvalue: "yes" }] }, Node { stype: For, svalue: [Token { ttype: For, tvalue: "roll" }, Token { ttype: ID, tvalue: "c" }, Token { ttype: ID, tvalue: "in" }, Token { ttype: ID, tvalue: "r" }, Token { ttype: ID, tvalue: "finger" }, Token { ttype: ID, tvalue: "i" }, Token { ttype: Lcb, tvalue: "[" }, Token { ttype: Print, tvalue: "shout" }, Token { ttype: Lb, tvalue: "<" }, Token { ttype: ID, tvalue: "i" }, Token { ttype: Semicolon, tvalue: ";" }] }, Node { stype: While, svalue: [Token { ttype: While, tvalue: "spin" }, Token { ttype: ID, tvalue: "b" }, Token { ttype: Equal, tvalue: "=" }, Token { ttype: Equal, tvalue: "=" }, Token { ttype: Int, tvalue: "0" }, Token { ttype: Lcb, tvalue: "[" }, Token { ttype: Ret, tvalue: "begone" }, Token { ttype: Int, tvalue: "1" }, Token { ttype: Semicolon, tvalue: ";" }] }, Node { stype: If, svalue: [Token { ttype: If, tvalue: "if" }, Token { ttype: ID, tvalue: "food" }, Token { ttype: Lcb, tvalue: "[" }, Token { ttype: Print, tvalue: "shout" }, Token { ttype: Lb, tvalue: "<" }, Token { ttype: Equal, tvalue: "=" }, Token { ttype: Int, tvalue: "4" }, Token { ttype: Semicolon, tvalue: ";" }, Token { ttype: Semicolon, tvalue: ";" }] }, Node { stype: Return, svalue: [Token { ttype: Ret, tvalue: "begone" }, Token { ttype: Int, tvalue: "0" }] }]
```
