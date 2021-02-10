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

// Break string into tokens
pub fn tokenizer(raw_code: &str) -> Vec<(Token, &str)> {
    let mut tokens: Vec<(Token, &str)> = Vec::new();
    
    // Transfrom words into tokens
    for word in raw_code.split_whitespace() {
        // Check if word contains brackets or operators and split
        tokens.extend(word_to_token(slice_syntax(word)));
    }

    tokens
}


fn word_to_token(words: Vec<&str>) -> Vec<(Token, &str)> {
    // DEBUG
    //println!("{:?}", words);

    let mut tokens_vec: Vec<(Token, &str)> = Vec::new();

    for word in words {
        match word {
            "int"|"char"|"string"|"bool" => tokens_vec.push((Token::Type, word)),
            "(" => tokens_vec.push((Token::Lb, word)),
            ")" => tokens_vec.push((Token::Rb, word)),
            "{" => tokens_vec.push((Token::Lcb, word)),
            "}" => tokens_vec.push((Token::Rcb, word)),
            ";" => tokens_vec.push((Token::Semicolon, word)),
            "=" => tokens_vec.push((Token::Assign, word)),
            "+"|"-"|"/"|"*"|"%"|"^"|"|"|"&" => tokens_vec.push((Token::Operator, word)),
            "return" => tokens_vec.push((Token::Ret, word)),
            "" => (),
            _ => {
                    // Check if number
                    let num = word.parse::<f64>();
                    let is_num;
                    match num {
                        Ok(_ok) => is_num = true,
                        Err(_e) => is_num = false,
                    }

                    // Check if string or char
                    if is_num || word.contains('"') || word.contains("'") || word == "true" || word == "false" {
                        tokens_vec.push((Token::Value, word));
                    } else {
                        // If everything fails it's an ID
                        tokens_vec.push((Token::ID, word));
                    }
            }
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

    if separator != '0' {
        // Slice word
        // Would rise panic if unwrap fails, which should not happen
        let i: usize = word.find(separator).unwrap();

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
