pub fn get_string_between_parenteses(starting_parentheses_index: usize, s: &str) -> &str {
    let mut opening_parens = 0;
    for (i, c) in s[starting_parentheses_index..].chars().enumerate() {
        println!("{}", c);
        match c {
            '(' => opening_parens += 1,
            ')' => opening_parens -= 1,
            _ => continue,
        }
        if opening_parens == 0 {
            return &s[starting_parentheses_index..=i + starting_parentheses_index];
        }
    }
    panic!("invalid string / starting index")
}
