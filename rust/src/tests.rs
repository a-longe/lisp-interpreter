use crate::ast::{self, create_node, get_empty_declarations, Declarations, Procedure, Value};
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

fn create_val_node(val: Value) -> ast::Node {
    ast::create_node(ast::Expr::Value(val), ast::get_empty_declarations())
}

#[test]
fn value_ast() {
    let node = ast::create_node(ast::Expr::Value(ast::Value::Number(5.0)), ast::get_empty_declarations());
    assert_eq!(node.execute().ok().unwrap(), Value::Number(5.0))
}

#[test]
fn adding_ast() {
    let proc = Procedure {
        func_token: "+".to_string(),
        args: vec![create_val_node(Value::Number(1.0)), create_val_node(Value::Number(1.0))]
    };
    let node = ast::create_node(ast::Expr::Procedure(proc), get_empty_declarations());
    assert_eq!(node.execute().ok().unwrap(), Value::Number(2.0));
}

#[test]
fn subbing_ast() {
    let proc = Procedure {
        func_token: "-".to_string(),
        args: vec![create_val_node(Value::Number(3.0)), create_val_node(Value::Number(1.0))]
    };
    let node = ast::create_node(ast::Expr::Procedure(proc), get_empty_declarations());
    assert_eq!(node.execute().ok().unwrap(), Value::Number(2.0));
}
#[test]
fn multing_by_one_ast() {
    let proc = Procedure {
        func_token: "*".to_string(),
    args: vec![create_val_node(Value::Number(1.0)), create_val_node(Value::Number(2.0))]
    };
    let node = ast::create_node(ast::Expr::Procedure(proc), get_empty_declarations());
    assert_eq!(node.execute().ok().unwrap(), Value::Number(2.0));
}
#[test]
fn multing_ast() {
    let proc = Procedure {
        func_token: "*".to_string(),
    args: vec![create_val_node(Value::Number(2.0)), create_val_node(Value::Number(2.0))]
    };
    let node = ast::create_node(ast::Expr::Procedure(proc), get_empty_declarations());
    assert_eq!(node.execute().ok().unwrap(), Value::Number(4.0));
}
#[test]
fn multing_by_zero_ast() {
    let proc = Procedure {
        func_token: "*".to_string(),
    args: vec![create_val_node(Value::Number(2.0)), create_val_node(Value::Number(0.0))]
    };
    let node = ast::create_node(ast::Expr::Procedure(proc), get_empty_declarations());
    assert_eq!(node.execute().ok().unwrap(), Value::Number(0.0));
}
#[test]
fn dividing_ast() {
    let proc = Procedure {
        func_token: "/".to_string(),
    args: vec![create_val_node(Value::Number(2.0)), create_val_node(Value::Number(2.0))]
    };
    let node = ast::create_node(ast::Expr::Procedure(proc), get_empty_declarations());
    assert_eq!(node.execute().ok().unwrap(), Value::Number(1.0));
}
#[test]
fn factional_dividing_ast() {
    let proc = Procedure {
        func_token: "/".to_string(),
    args: vec![create_val_node(Value::Number(5.0)), create_val_node(Value::Number(2.0))]
    };
    let node = ast::create_node(ast::Expr::Procedure(proc), get_empty_declarations());
    assert_eq!(node.execute().ok().unwrap(), Value::Number(2.5));
}
#[test]
fn divide_by_zero_ast() {
    let proc = Procedure {
        func_token: "/".to_string(),
    args: vec![create_val_node(Value::Number(5.0)), create_val_node(Value::Number(0.0))]
    };
    let node = ast::create_node(ast::Expr::Procedure(proc), get_empty_declarations());
    assert_eq!(node.execute().err().unwrap(), ast::Error{ reason:"division by zero".to_string()});
}

#[test]
fn nested_ast() {
    /*
    +
   / \
  2  +
    / \
   2   2
    */
    let proc1 = Procedure {
        func_token: "+".to_string(),
        args: vec![create_val_node(Value::Number(1.0)), create_val_node(Value::Number(1.0))]
    };
    let proc2 = Procedure {
        func_token: "+".to_string(),
        args: vec![create_val_node(Value::Number(2.0)), ast::create_node(ast::Expr::Procedure(proc1), get_empty_declarations())]
    };
    let node = ast::create_node(ast::Expr::Procedure(proc2), get_empty_declarations());
    assert_eq!(node.execute().ok().unwrap(), Value::Number(4.0));
}

#[test]
fn basic_token_as_number_ast() {
    let mut declr = get_empty_declarations();
    declr.data.insert("x".to_string(), Value::Number(10.0));
    let node = ast::create_node(ast::Expr::Token("x".to_string()), declr);
    assert_eq!(node.execute().ok().unwrap(), Value::Number(10.0))
}
