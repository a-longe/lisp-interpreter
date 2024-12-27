#![allow(unused_imports, dead_code)]

use rust_decimal_macros::dec;

use crate::ast::*;
use crate::parse::*;
use crate::funcs::*;
use std::collections::HashMap;

pub fn simple_node(expr: Expression) -> ASTNode {
    return ASTNode {pos: (0, 0),
                    expr: Box::new(expr),
                    declarations: Box::new(Declarations::new())};
}

#[cfg(test)]
#[test]
fn exec_literal_number_basic() {
    let expr = Expression::Literal(Literal::Number(Number {value: dec!(5.4)}));
    let n = simple_node(expr);
    assert_eq!(n.execute(), Value::Number(Number{value: dec!(5.4)}));
}

#[test]
fn exec_literal_basic_number_ne() {
    let expr = Expression::Literal(Literal::Number(Number {value: dec!(5.4)}));
    let n = simple_node(expr);
    assert_ne!(n.execute(), Value::Number(Number{value: dec!(5.3)}));
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
    Value::Number(Number { value: dec!(2.0) }));
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
    Value::Number(Number { value: dec!(2.0) }));
}

#[test]
fn create_ast_and_exec_with_negative_literals_number() {
    assert_eq!(create_ast(get_tokens("-1")).execute(),
    Value::Number(Number { value: dec!(-1.0) }));
}

#[test]
fn create_ast_and_exec_with_negative_literals_zero_number() {
    assert_eq!(create_ast(get_tokens("-0")).execute(),
    Value::Number(Number { value: dec!(0.0) }));
}

#[test]
fn create_ast_and_exec_with_decimal_literals_number() {
    assert_eq!(create_ast(get_tokens("1.45")).execute(),
    Value::Number(Number { value: dec!(1.45) }));
}

#[test]
fn create_ast_and_exec_with_negative_decimal_literals_number() {
    assert_eq!(create_ast(get_tokens("-1.45")).execute(),
    Value::Number(Number { value: dec!(-1.45) }));
}

#[test]
fn create_ast_and_exec_with_fractional_addition() {
    assert_eq!(create_ast(get_tokens("(+ 1.5 2.5)")).execute(),
    Value::Number(Number { value: dec!(4.0) }));
}

#[test]
fn create_ast_and_exec_with_negative_fractional_addition() {
    assert_eq!(create_ast(get_tokens("(+ -1.5 2.5)")).execute(),
    Value::Number(Number { value: dec!(1.0) }));
}

#[test]
fn create_ast_and_exec_with_subtraction() {
    assert_eq!(create_ast(get_tokens("(- 2 1)")).execute(),
    Value::Number(Number { value: dec!(1.0) }));
}

#[test]
fn create_ast_and_exec_with_subtraction_2() {
    assert_eq!(create_ast(get_tokens("(- 1 2)")).execute(),
    Value::Number(Number { value: dec!(-1.0) }));
}

#[test]
fn create_ast_and_exec_with_subtraction_negative_numbers() {
    assert_eq!(create_ast(get_tokens("(- -1 -2)")).execute(),
    Value::Number(Number { value: dec!(1.0) }));
}

#[test]
fn create_ast_and_exec_with_multiplication() {
    assert_eq!(create_ast(get_tokens("(* 5 1)")).execute(),
    Value::Number(Number { value: dec!(5.0) }));
}

#[test]
fn create_ast_and_exec_addition_with_one_arg() {
    assert_eq!(create_ast(get_tokens("(+ 2)")).execute(),
    Value::Number(Number { value: dec!(2.0) }));
}

#[test]
fn create_ast_and_exec_subtraction_with_one_arg() {
    assert_eq!(create_ast(get_tokens("(- -2)")).execute(),
    Value::Number(Number { value: dec!(-2) }));
}

#[test]
fn create_ast_and_exec_multiplication_with_one_arg() {
    assert_eq!(create_ast(get_tokens("(* 2)")).execute(),
    Value::Number(Number { value: dec!(2) }));
}

#[test]
fn create_ast_and_exec_fractional_subtraction() {
    assert_eq!(create_ast(get_tokens("(- 10.2 2.4)")).execute(),
    Value::Number(Number { value: dec!(7.8) }));
}

#[test]
fn create_ast_and_exec_fractional_multiplication() {
    assert_eq!(create_ast(get_tokens("(* 1.5 2)")).execute(),
    Value::Number(Number { value: dec!(3.0) }));
}

#[test]
fn create_ast_and_exec_multiplication_by_zero() {
    assert_eq!(create_ast(get_tokens("(* 1.5 0 2)")).execute(),
    Value::Number(Number { value: dec!(0) }));
}

#[test]
fn create_ast_and_exec_fractional_multiplication_lt_one() {
    assert_eq!(create_ast(get_tokens("(* 0.5 2.2)")).execute(),
    Value::Number(Number { value: dec!(1.1) }));
}

#[test]
fn create_ast_and_exec_division() {
    assert_eq!(create_ast(get_tokens("(/ 1 2)")).execute(),
    Value::Number(Number { value: dec!(0.5) }));
}

#[test]
fn create_ast_and_exec_division_one_arg() {
    assert_eq!(create_ast(get_tokens("(/ 2)")).execute(),
    Value::Number(Number { value: dec!(0.5) }));
}

#[test]
fn create_ast_and_exec_division_with_fractions() {
    assert_eq!(create_ast(get_tokens("(/ 1 0.5)")).execute(),
    Value::Number(Number { value: dec!(2) }));
}

#[test]
fn create_ast_and_exec_division_gt_two_args() {
    assert_eq!(create_ast(get_tokens("(/ 12 2 3)")).execute(),
    Value::Number(Number { value: dec!(2) }));
}

#[test]
#[should_panic]
fn create_ast_and_exec_division_by_zero() {
    assert_eq!(create_ast(get_tokens("(/ 12 2 0 1)")).execute(),
    Value::Number(Number { value: dec!(2) }));
}

#[test]
fn create_ast_basic_let() {
    create_ast(get_tokens("(let ((x 2)) 2)"));
}

#[test]
fn create_ast_basic_let_with_assignment() {
    create_ast(get_tokens("(let ((x 2)) x)"));
}

#[test]
fn exec_literal_symbol_without_assignment () {
        assert_eq!(create_ast(get_tokens("x")).execute(),
            Value::Symbol(Symbol {value: "x".to_string()}));
}

#[test]
fn exec_literal_symbol_with_assignment () {
        let mut a = create_ast(get_tokens("x"));
        a.declarations.add(Symbol {value: "x".to_string()},
            Value::Number( Number {value: dec!(2)}));
        assert_eq!(a.execute(), Value::Number(Number {value: dec!(2)}));

}

#[test]
fn create_ast_and_exec_let() {
    assert_eq!(create_ast(get_tokens("(let ((x 2)) x)")).execute(),
    Value::Number(Number { value: dec!(2) }));
}

#[test]
fn create_ast_and_exec_let_with_arithmatic() {
    assert_eq!(create_ast(get_tokens("(let ((x 2)) (* 2 x))")).execute(),
    Value::Number(Number { value: dec!(4) }));
}

#[test]
fn create_ast_and_exec_let_nested() {
    assert_eq!(create_ast(get_tokens("(let ((x 2)) (let ((y 2)) (* x y)))")).execute(),
    Value::Number(Number { value: dec!(4) }));
}

