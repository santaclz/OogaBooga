use std::fs;

#[derive(Debug)]
pub enum Token {
    ID,
    Type,
    Value,
    Assign,
    Operator,
    Lb,
    Rb,
    Lcb,
    Rcb,
    Ret,
    Semicolon,
}


fn word_to_token(words: Vec<&str>) -> Vec<Token> {
    // DEBUG
    //println!("{:?}", words);

    let mut tokens_vec: Vec<Token> = vec![];

    for word in words {
        match word {
            "int"|"char"|"string" => tokens_vec.push(Token::Type),
            "(" => tokens_vec.push(Token::Lb),
            ")" => tokens_vec.push(Token::Rb),
            "{" => tokens_vec.push(Token::Lcb),
            "}" => tokens_vec.push(Token::Rcb),
            ";" => tokens_vec.push(Token::Semicolon),
            "=" => tokens_vec.push(Token::Assign),
            "+"|"-"|"/"|"*"|"%"|"^"|"|"|"&" => tokens_vec.push(Token::Operator),
            "return" => tokens_vec.push(Token::Ret),
            "" => (),
            _ => {
                    let num = word.parse::<f64>();
                    let is_num;
                    match num {
                        Ok(_ok) => is_num = true,
                        Err(_e) => is_num = false,
                    }

                    if is_num || word.contains('"') || word.contains("'") {
                        tokens_vec.push(Token::Value);
                    } else {
                        tokens_vec.push(Token::ID);
                    }
            }
        }
    }
    tokens_vec
}

pub fn tokenizer(filename: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    // Break string into tokens
    let raw_code = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    
    // Transfrom words into tokens
    for word in raw_code.split_whitespace() {
        // Check if word contains brackets and split
        tokens.extend(word_to_token(slice_syntax(word)));
    }

    tokens
}

// Slice strings without whitespaces for token processing
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

    if separator != '0' {
        // Slice word
        let index: Option<usize> = word.find(separator);
        let i: usize;
        match index {
            Some(ref p) => i = *p,
            None => i = 0,
        }
        let slice1 = &word[0..i];
        let slice2 = &word[i..i+1];
        let slice3 = &word[i+1..word.len()];

        let mut ret_vec: Vec<&str> = slice_syntax(slice1);
        ret_vec.push(slice2);
        ret_vec.extend(slice_syntax(slice3));
        
        ret_vec
    } else {
        vec!(word)
    }
}
