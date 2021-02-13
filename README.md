# WiredC

Compiler written in Rust for my version of C called WiredC.

# Why?

The purpose of this project is to learn how compiler works and to get better at understanding assembly code. Also, this is my first Rust program, I just jumped straight into making a compiler with it, so far it's a great learning experience.

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
It then parses the function body and recognizes which type of statements are inside it. I'm currently working on step 3 and 4.

Inside examples/return_0.wc

```
$ cat examples/return_0.wc      
int main() {
	if (a == b); 
	while (x != 3);
	for a in abc;
	print("string with 2+2 spaces ");
	return 0;
}
```

Program output:

```
$ cargo run examples/return_0.wc

Tokens:
[Token { ttype: Type, tvalue: "int" }, Token { ttype: ID, tvalue: "main" }, Token { ttype: Lb, tvalue: "(" }, Token { ttype: Rb, tvalue: ")" }, Token { ttype: Lcb, tvalue: "{" }, Token { ttype: If, tvalue: "if" }, Token { ttype: Lb, tvalue: "(" }, Token { ttype: ID, tvalue: "a" }, Token { ttype: Equal, tvalue: "=" }, Token { ttype: Equal, tvalue: "=" }, Token { ttype: ID, tvalue: "b" }, Token { ttype: Rb, tvalue: ")" }, Token { ttype: Semicolon, tvalue: ";" }, Token { ttype: While, tvalue: "while" }, Token { ttype: Lb, tvalue: "(" }, Token { ttype: ID, tvalue: "x" }, Token { ttype: ID, tvalue: "!" }, Token { ttype: Equal, tvalue: "=" }, Token { ttype: Int, tvalue: "3" }, Token { ttype: Rb, tvalue: ")" }, Token { ttype: Semicolon, tvalue: ";" }, Token { ttype: For, tvalue: "for" }, Token { ttype: ID, tvalue: "a" }, Token { ttype: ID, tvalue: "in" }, Token { ttype: ID, tvalue: "abc" }, Token { ttype: Semicolon, tvalue: ";" }, Token { ttype: Print, tvalue: "print" }, Token { ttype: Lb, tvalue: "(" }, Token { ttype: Str, tvalue: "\"string with 2+2 spaces \"" }, Token { ttype: Rb, tvalue: ")" }, Token { ttype: Semicolon, tvalue: ";" }, Token { ttype: Ret, tvalue: "return" }, Token { ttype: Int, tvalue: "0" }, Token { ttype: Semicolon, tvalue: ";" }, Token { ttype: Rcb, tvalue: "}" }]

AST:
[If, While, For, Print, Return]
```
