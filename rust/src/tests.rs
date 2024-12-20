#![allow(unused_imports, dead_code)]

use crate::ast::*;
use crate::parse::*;
use std::collections::HashMap;

pub fn simple_node(expr: Expression) -> ASTNode {
    return ASTNode {pos: (0, 0),
                    expr: Box::new(expr),
                    declarations: Box::new(Declarations::new())};
}

#[cfg(test)]
#[test]
fn exec_literal_number_basic() {
    let expr = Expression::Literal(Literal::Number(Number {value: 5.4}));
    let n = simple_node(expr);
    assert_eq!(n.execute(), Value::Number(Number{value: 5.4}));
}

#[test]
fn exec_literal_basic_number_ne() {
    let expr = Expression::Literal(Literal::Number(Number {value: 5.4}));
    let n = simple_node(expr);
    assert_ne!(n.execute(), Value::Number(Number{value: 5.3}));
}

#[test]
fn exec_literal_bool_basic() {
    let expr = Expression::Literal(Literal::Boolean(Boolean {value: false}));
    let n = simple_node(expr);
    assert_eq!(n.execute(), Value::Boolean(Boolean{value: false}));
}

#[test]
fn exec_literal_basic_bool_ne() {
    let expr = Expression::Literal(Literal::Boolean(Boolean{value: true}));
    let n = simple_node(expr);
    assert_ne!(n.execute(), Value::Boolean(Boolean{value: false}));
}

#[test]
fn get_tokens_basic() {
    assert_eq!(get_tokens("(+ 1 1)"), vec!["(", "+", "1", "1", ")"]);
}

#[test]
fn get_expression_inside_start_at_zero() {
    assert_eq!(get_tokens_inside(0, &get_tokens("(+ 1 1)")),
    vec!["(", "+", "1", "1", ")"]);
}

#[test]
fn get_expression_inside_start_at_not_zero() {
    assert_eq!(get_tokens_inside(3, &get_tokens("(* 2 (+ 1 1))")),
    vec!["(", "+", "1", "1", ")"]);
}

#[test]
fn get_expression_inside_start_nested() {
    assert_eq!(get_tokens_inside(0, &get_tokens("(* 2 (+ 1 1))")),
    vec!["(", "*", "2", "(", "+", "1", "1", ")", ")"]);
}

#[test]
fn get_args_from_tokens_basic() {
    assert_eq!(split_tokens_into_args(get_tokens("(+ 2 (+ 1 1))")),
    vec![vec!["+"], vec!["2"], vec!["(", "+", "1", "1", ")"]]);
}

#[test]
fn get_args_from_tokens_nested_before_last() {
    assert_eq!(split_tokens_into_args(get_tokens("(+ (+ 1 1) 2)")),
    vec![vec!["+"], vec!["(", "+", "1", "1", ")"], vec!["2"]]);
}

#[test]
fn create_ast_and_exec_with_single_literal_number() {
    assert_eq!(create_ast(get_tokens("2")).execute(),
    Value::Number(Number { value: 2.0 }));
}

#[test]
fn create_ast_and_exec_with_single_literal_boolean() {
    assert_eq!(create_ast(get_tokens("#true")).execute(),
    Value::Boolean(Boolean { value: true }));
}

#[test]
fn create_ast_and_exec_with_single_literal_symbol() {
    assert_eq!(create_ast(get_tokens("let")).execute(),
    Value::Symbol(Symbol { value: "let".to_string() }));
}

#[test]
fn create_ast_and_exec_with_simplest_proc_call() {
    assert_eq!(create_ast(get_tokens("(+ 1 1)")).execute(),
    Value::Number(Number { value: 2.0 }));
}

