#[cfg(test)]
use crate::parse;
#[test]
fn passing_test() {
    assert_eq!(1, 1)
}
#[test]
fn baisc_parse_get_string_in_parens() {
    assert_eq!(
        parse::get_string_between_parenteses(0, "(+ 1 1)"),
        "(+ 1 1)"
    )
}
#[test]
fn nested_parse_get_string_in_parens() {
    assert_eq!(
        parse::get_string_between_parenteses(3, "(+ (+ 1 1) 2)"),
        "(+ 1 1)"
    )
}
#[test]
fn double_nested_parse_get_string_in_parens() {
    assert_eq!(
        parse::get_string_between_parenteses(11, "(+ (+ 1 1) (+ 2 (* 2 2) 2))"),
        "(+ 2 (* 2 2) 2)"
    )
}
