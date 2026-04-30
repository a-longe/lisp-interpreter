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
    panic!("Parentheses Do Not Match")
}

pub fn get_tokens_inside(starting_index: usize, tokens: &Vec<String>) -> Vec<String> {
    return tokens[starting_index..=get_closing_paren_index(starting_index, tokens.clone())].to_vec();
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

// takes the tokens in a expression and splits them into the arguments
// of the procedure call
// ex.
// (+ 2 (+ 1 1))
// -> ["(", "+", "2", "(", "+", "1", "1", ")", ")"]
// -> [["+"], ["2"], ["(", "+", "1", "1", ")"]]
pub fn split_tokens_into_args(tokens: Vec<String>) -> Vec<Vec<String>> {
    let mut args = Vec::new();
    let mut i = 1;
    while i < tokens.len()-1 {
        let mut arg: Vec<String> = Vec::new();
        if tokens[i] == "(" {
            arg = get_tokens_inside(i, &tokens);
            i=get_closing_paren_index(i, tokens.clone())+1;
        }
        else {
            arg.push(tokens[i].clone());
            i+=1;
        }
        args.push(arg);
    }
    return args;
}
