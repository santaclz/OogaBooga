pub use crate::ast::{Token, TokenType};

// Break string into tokens
pub fn tokenizer(raw_code: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    
    // Transfrom words into tokens

    // Split all words unless they are enclosed in quotes
    let mut raw_code_split: Vec<&str> = Vec::new();

    for (i, mut w) in raw_code.split('"').enumerate() {

        // All odd elements of splitted code are strings
        if i % 2 == 0 {
            // Push all elements to vec
            for ww in w.split_whitespace() {
                raw_code_split.push(ww);
            }
        } else {
            // Otherwise push entire string onto vec
            // Find string in raw_code and add quotes from start-1 to end+1
            // This is needed because we are operating on references and quotes are needed for
            // further string processing
            let start_str = raw_code.find(w).unwrap();
            w = &raw_code[start_str - 1 .. start_str + w.len() + 1];
            raw_code_split.push(w);
        }
    }

    for word in raw_code_split {
        
        // Check if word contains brackets or operators and split
        tokens.extend(word_to_token(slice_syntax(word)));

    }

    tokens
}


fn word_to_token(words: Vec<&str>) -> Vec<Token> {
    // DEBUG
    //println!("\n\nword_to_token:\n\n{:?}", words);

    let mut tokens_vec: Vec<Token> = Vec::new();

    // Match struct token and put it into vector
    for word in words {
        let tok: Option<Token> = match word {
            "num"|"chr"|"str"|"bool" => Some(Token { ttype: TokenType::Type, tvalue: word }),
            "<" => Some(Token { ttype: TokenType::Lb, tvalue: word }),
            ">" => Some(Token { ttype: TokenType::Rb, tvalue: word }),
            "[" => Some(Token { ttype: TokenType::Lcb, tvalue: word }),
            "]" => Some(Token { ttype: TokenType::Rcb, tvalue: word }),
            ";" => Some(Token { ttype: TokenType::Semicolon, tvalue: word }),
            "=" => Some(Token { ttype: TokenType::Equal, tvalue: word }),
            "+" => Some(Token { ttype: TokenType::Plus, tvalue: word }),
            "-" => Some(Token { ttype: TokenType::Minus, tvalue: word }),
            "*" => Some(Token { ttype: TokenType::Mult, tvalue: word }),
            "/" => Some(Token { ttype: TokenType::Div, tvalue: word }),
            "%" => Some(Token { ttype: TokenType::Mod, tvalue: word }),
            "~" => Some(Token { ttype: TokenType::Complement, tvalue: word }),
            "ret" => Some(Token { ttype: TokenType::Ret, tvalue: word }),
            "say" => Some(Token { ttype: TokenType::Print, tvalue: word }),
            "eat" => Some(Token { ttype: TokenType::Input, tvalue: word }),
            "if" => Some(Token { ttype: TokenType::If, tvalue: word }),
            "while" => Some(Token { ttype: TokenType::While, tvalue: word }),
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
                    else if word == "yes" || word == "no" { Some(Token { ttype: TokenType::Bool, tvalue: word }) }
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

    // If the given input contains whitespace it's a string
    if word.contains(" ")       { vec!(word); }

    else if word.contains("<")       { separator = '<';  }
    else if word.contains(">")  { separator = '>';  }
    else if word.contains("[")  { separator = '[';  }
    else if word.contains("]")  { separator = ']';  }
    else if word.contains(";")  { separator = ';';  }
    else if word.contains("=")  { separator = '=';  }
    else if word.contains("+")  { separator = '+';  }
    else if word.contains("-")  { separator = '-';  }
    else if word.contains("*")  { separator = '*';  }
    else if word.contains("/")  { separator = '/';  }
    else if word.contains("^")  { separator = '^';  }
    else if word.contains("%")  { separator = '%';  }
    else if word.contains("~")  { separator = '~';  }
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
