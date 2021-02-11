# WiredC

Compiler written in Rust for my version of C called WiredC.

# Why?

The purpose of this project is to learn how compiler works and to get better at understanding low level stuff.

# Steps

1. Write lexer that takes code and returns list of lexemes (tokens)
2. Define Abstract syntax tree (AST)
3. Write parser that takes tokens and returns AST (and also throws compile errors)
4. Generate code for given AST in assembly (x86_64)

# Current progress

Inside examples/simple_exp.wc

```
$ cat examples/simple_exp.wc      
int main(int a, int b)
{
	print("Hello World! 2+2 is ");
	int a = 2+2;
	print(a);
	
	return 0;
} this should get ignored
```

Program output:

```
$ cargo run examples/simple_exp.wc
func_body:

[Token { ttype: ID, tvalue: "print" }, Token { ttype: Lb, tvalue: "(" }, Token { ttype: Str, tvalue: "\"Hello" }, Token { ttype: ID, tvalue: "World!" }, Token { ttype: Int, tvalue: "2" }, Token { ttype: Plus, tvalue: "+" }, Token { ttype: Int, tvalue: "2" }, Token { ttype: ID, tvalue: "is" }, Token { ttype: Str, tvalue: "\"" }, Token { ttype: Rb, tvalue: ")" }, Token { ttype: Semicolon, tvalue: ";" }, Token { ttype: Type, tvalue: "int" }, Token { ttype: ID, tvalue: "a" }, Token { ttype: Equal, tvalue: "=" }, Token { ttype: Int, tvalue: "2" }, Token { ttype: Plus, tvalue: "+" }, Token { ttype: Int, tvalue: "2" }, Token { ttype: Semicolon, tvalue: ";" }, Token { ttype: ID, tvalue: "print" }, Token { ttype: Lb, tvalue: "(" }, Token { ttype: ID, tvalue: "a" }, Token { ttype: Rb, tvalue: ")" }, Token { ttype: Semicolon, tvalue: ";" }, Token { ttype: Ret, tvalue: "return" }, Token { ttype: Int, tvalue: "0" }, Token { ttype: Semicolon, tvalue: ";" }]


tokenizer:

[Token { ttype: Type, tvalue: "int" }, Token { ttype: ID, tvalue: "main" }, Token { ttype: Lb, tvalue: "(" }, Token { ttype: Type, tvalue: "int" }, Token { ttype: ID, tvalue: "a" }, Token { ttype: Type, tvalue: "int" }, Token { ttype: ID, tvalue: "b" }, Token { ttype: Rb, tvalue: ")" }, Token { ttype: Lcb, tvalue: "{" }, Token { ttype: ID, tvalue: "print" }, Token { ttype: Lb, tvalue: "(" }, Token { ttype: Str, tvalue: "\"Hello" }, Token { ttype: ID, tvalue: "World!" }, Token { ttype: Int, tvalue: "2" }, Token { ttype: Plus, tvalue: "+" }, Token { ttype: Int, tvalue: "2" }, Token { ttype: ID, tvalue: "is" }, Token { ttype: Str, tvalue: "\"" }, Token { ttype: Rb, tvalue: ")" }, Token { ttype: Semicolon, tvalue: ";" }, Token { ttype: Type, tvalue: "int" }, Token { ttype: ID, tvalue: "a" }, Token { ttype: Equal, tvalue: "=" }, Token { ttype: Int, tvalue: "2" }, Token { ttype: Plus, tvalue: "+" }, Token { ttype: Int, tvalue: "2" }, Token { ttype: Semicolon, tvalue: ";" }, Token { ttype: ID, tvalue: "print" }, Token { ttype: Lb, tvalue: "(" }, Token { ttype: ID, tvalue: "a" }, Token { ttype: Rb, tvalue: ")" }, Token { ttype: Semicolon, tvalue: ";" }, Token { ttype: Ret, tvalue: "return" }, Token { ttype: Int, tvalue: "0" }, Token { ttype: Semicolon, tvalue: ";" }, Token { ttype: Rcb, tvalue: "}" }, Token { ttype: ID, tvalue: "this" }, Token { ttype: ID, tvalue: "should" }, Token { ttype: ID, tvalue: "get" }, Token { ttype: ID, tvalue: "ignored" }]


ast:

[]
```
