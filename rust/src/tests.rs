#![allow(unused_imports, dead_code)]

use crate::ast::*;
use crate::parse::*;
use std::collections::HashMap;

pub fn simple_node(expr: Expression) -> ASTNode {
    return ASTNode {pos: (0, 0),
                    expr: Box::new(expr),
                    declarations: Declarations {declr: HashMap::new()}}
}

#[cfg(test)]
#[test]
fn passing_test() {
    assert_eq!(1, 1)
}

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
