pub fn get_closing_paren_index(starting_parentheses_index: usize, tokens: Vec<String>) -> usize {
    let mut opening_parens = 0;
    for (i, t) in tokens[starting_parentheses_index..tokens.len()].iter().enumerate() {
        match t.as_str() {
            "(" => opening_parens += 1,
            ")" => opening_parens -= 1,
            _ => continue,
        }
        if opening_parens == 0 {
            return i + starting_parentheses_index;
        }
    }
    panic!("invalid string / starting index")
}

pub fn get_tokens(string: &str) -> Vec<String> {
    let replace1 = string.replace("(", "( ");
    let replace2 = replace1.replace(")", " )").clone();
    let split = replace2.split(" ");
    let mut tokens = Vec::new();
    for t in split {
        tokens.push(t.to_string())
    }
    tokens
}
