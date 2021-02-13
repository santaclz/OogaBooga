# WiredC

Compiler written in Rust for my version of C called WiredC.

# Why?

The purpose of this project is to learn how compiler works and to get better at understanding low level stuff.

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
It then parses the function body and recognizes which type of statements are inside it. I'm currently somewhere between step 3. and 4.

Inside examples/simple_exp.wc

```
$ cat examples/simple_exp.wc      
int main(int a, int b)
{
	print("Hello World! 2+2 is ");
	int a = 2+2;

	int b;
	b=4;
	
	return 0;
} this should get ignored
```

Program output:

```
$ cargo run examples/simple_exp.wc
func_body:

[Token { ttype: Print, tvalue: "print" }, Token { ttype: Lb, tvalue: "(" }, Token { ttype: Str, tvalue: "\"Hello World! 2+2 is \"" }, Token { ttype: Rb, tvalue: ")" }, Token { ttype: Semicolon, tvalue: ";" }, Token { ttype: Type, tvalue: "int" }, Token { ttype: ID, tvalue: "a" }, Token { ttype: Equal, tvalue: "=" }, Token { ttype: Int, tvalue: "2" }, Token { ttype: Plus, tvalue: "+" }, Token { ttype: Int, tvalue: "2" }, Token { ttype: Semicolon, tvalue: ";" }, Token { ttype: Type, tvalue: "int" }, Token { ttype: ID, tvalue: "b" }, Token { ttype: Semicolon, tvalue: ";" }, Token { ttype: ID, tvalue: "b" }, Token { ttype: Equal, tvalue: "=" }, Token { ttype: Int, tvalue: "4" }, Token { ttype: Semicolon, tvalue: ";" }, Token { ttype: Ret, tvalue: "return" }, Token { ttype: Int, tvalue: "0" }, Token { ttype: Semicolon, tvalue: ";" }]


ast:

[Print, Init, Declare, Assign, Return]
```
