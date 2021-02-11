pub use crate::ast::{Token, TokenType};

// Break string into tokens
pub fn tokenizer(raw_code: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    
    // Transfrom words into tokens
    for word in raw_code.split_whitespace() {
        // Check if word contains brackets or operators and split
        tokens.extend(word_to_token(slice_syntax(word)));
    }

    tokens
}


fn word_to_token(words: Vec<&str>) -> Vec<Token> {
    // DEBUG
    //println!("{:?}", words);

    let mut tokens_vec: Vec<Token> = Vec::new();

    // Match struct token and put it into vector
    for word in words {
        let tok: Option<Token> = match word {
            "int"|"char"|"string"|"bool" => Some(Token { ttype: TokenType::Type, tvalue: word }),
            "(" => Some(Token { ttype: TokenType::Lb, tvalue: word }),
            ")" => Some(Token { ttype: TokenType::Rb, tvalue: word }),
            "{" => Some(Token { ttype: TokenType::Lcb, tvalue: word }),
            "}" => Some(Token { ttype: TokenType::Rcb, tvalue: word }),
            ";" => Some(Token { ttype: TokenType::Semicolon, tvalue: word }),
            "=" => Some(Token { ttype: TokenType::Equal, tvalue: word }),
            "+" => Some(Token { ttype: TokenType::Plus, tvalue: word }),
            "-" => Some(Token { ttype: TokenType::Minus, tvalue: word }),
            "*" => Some(Token { ttype: TokenType::Mult, tvalue: word }),
            "/" => Some(Token { ttype: TokenType::Div, tvalue: word }),
            "%" => Some(Token { ttype: TokenType::Mod, tvalue: word }),
            "return" => Some(Token { ttype: TokenType::Ret, tvalue: word }),
            "" => None,
            _ => {
                    // Check if signed int
                    let num = word.parse::<i64>();
                    let is_num;
                    match num {
                        Ok(_ok) => is_num = true,
                        Err(_e) => is_num = false,
                    }

                    // Check if int, str, char or bool
                    if is_num                   { Some(Token { ttype: TokenType::Int, tvalue: word }) }
                    else if word.contains('"')  { Some(Token { ttype: TokenType::Str, tvalue: word }) }
                    else if word.contains("'")  { Some(Token { ttype: TokenType::Char, tvalue: word }) }
                    else if word == "true" || word == "false" { Some(Token { ttype: TokenType::Bool, tvalue: word }) }
                    else {
                        // If everything fails it's an ID
                        Some(Token { ttype: TokenType::ID, tvalue: word })
                    }
            }
        };

        // If token is empty do nothing
        match tok {
            Some(t) => tokens_vec.push(t),
            None => (),
        }
    }
    tokens_vec
}


// Slice strings recursively without whitespaces for token processing
fn slice_syntax(word: &str) -> Vec<&str> {
    let mut separator: char = '0';

    if word.contains("(")  { separator = '(';  }
    else if word.contains(")")  { separator = ')';  }
    else if word.contains("{")  { separator = '{';  }
    else if word.contains("}")  { separator = '}';  }
    else if word.contains(";")  { separator = ';';  }
    else if word.contains("=")  { separator = '=';  }
    else if word.contains("+")  { separator = '+';  }
    else if word.contains("-")  { separator = '-';  }
    else if word.contains("*")  { separator = '*';  }
    else if word.contains("/")  { separator = '/';  }
    else if word.contains("^")  { separator = '^';  }
    else if word.contains("%")  { separator = '%';  }
    else if word.contains("|")  { separator = '|';  }
    else if word.contains("&")  { separator = '&';  }
    // Not a real token, just a separator
    else if word.contains(",")  { separator = ',';  }

    if separator != '0' {
        // Slice word
        // Would rise panic if unwrap fails, which should not happen
        let i: usize = word.find(separator).unwrap();

        let slice1 = &word[0..i];
        let slice2 = &word[i..i+1];
        let slice3 = &word[i+1..word.len()];

        let mut ret_vec: Vec<&str> = slice_syntax(slice1);

        // Ignore if separator is comma
        if separator != ',' { ret_vec.push(slice2); }

        ret_vec.extend(slice_syntax(slice3));
        
        ret_vec
    } else {
        vec!(word)
    }
}
