#![allow(unused_imports, dead_code)]
use crate::ast::{self, get_empty_declarations, create_val_node, Procedure, Value};
use crate::{ast::{Expr, Node}, parse::{self, get_tokens}};
#[cfg(test)]
#[test]
fn passing_test() {
    assert_eq!(1, 1)
}

#[test]
fn basic_get_tokens() {
    assert_eq!(parse::get_tokens("(+ 1 1)"),
        vec!["(".to_string(),
            "+".to_string(),
            "1".to_string(),
            "1".to_string(),
            ")".to_string()])
}

#[test]
fn basic_string_to_node() {
    let ast = ast::create_ast(get_tokens("(+ 1 1)"));
    assert_eq!(ast.execute().ok().unwrap(), Value::Number(2.0))
}
#[test]
fn nested_string_to_node() {
    let ast = ast::create_ast(get_tokens("(+ (* 1 2) 2)"));
    assert_eq!(ast.execute().ok().unwrap(), Value::Number(4.0))
}

#[test]
fn baisc_parse_get_closing_parentese_i() {
    assert_eq!(
        parse::get_closing_paren_index(0, get_tokens("(+ 1 1)")),
        4
    )
}
#[test]
fn nested_inner_parse_get_closing_paren_index() {
    assert_eq!(
        parse::get_closing_paren_index(2, get_tokens("(+ (+ 1 1) 2)")),
        6
    )
}
#[test]
fn nested_outer_parse_get_closing_paren_index() {
    assert_eq!(
        parse::get_closing_paren_index(0, get_tokens("(+ (+ 1 1) 2)")),
        8
    )
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
fn adding_over_two_numbers_ast() {
    let proc = Procedure {
        func_token: "+".to_string(),
        args: vec![create_val_node(Value::Number(1.0)), create_val_node(Value::Number(1.0)), create_val_node(Value::Number(2.0))]
    };
    let node = ast::create_node(ast::Expr::Procedure(proc), get_empty_declarations());
    assert_eq!(node.execute().ok().unwrap(), Value::Number(4.0));
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
fn subbing_over_two_numbers_ast() {
    let proc = Procedure {
        func_token: "-".to_string(),
        args: vec![create_val_node(Value::Number(3.0)), create_val_node(Value::Number(1.0)), create_val_node(Value::Number(0.5))]
    };
    let node = ast::create_node(ast::Expr::Procedure(proc), get_empty_declarations());
    assert_eq!(node.execute().ok().unwrap(), Value::Number(1.5));
}
#[test]
fn subbing_only_one_number_ast() {
    let proc = Procedure {
        func_token: "-".to_string(),
        args: vec![create_val_node(Value::Number(3.0))]
    };
    let node = ast::create_node(ast::Expr::Procedure(proc), get_empty_declarations());
    assert_eq!(node.execute().ok().unwrap(), Value::Number(-3.0));
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
fn dividing_multiple_numbers_ast() {
    let proc = Procedure {
        func_token: "/".to_string(),
        args: vec![create_val_node(Value::Number(8.0)), create_val_node(Value::Number(2.0)), create_val_node(Value::Number(2.0))]
    };
    let node = ast::create_node(ast::Expr::Procedure(proc), get_empty_declarations());
    assert_eq!(node.execute().ok().unwrap(), Value::Number(2.0));
}
#[test]
fn dividing_by_fraction_ast() {
    let proc = Procedure {
        func_token: "/".to_string(),
        args: vec![create_val_node(Value::Number(2.0)), create_val_node(Value::Number(0.1))]
    };
    let node = ast::create_node(ast::Expr::Procedure(proc), get_empty_declarations());
    assert_eq!(node.execute().ok().unwrap(), Value::Number(20.0));
}
#[test]
fn dividing_with_only_one_number_ast() {
    let proc = Procedure {
        func_token: "/".to_string(),
        args: vec![create_val_node(Value::Number(2.0))]
    };
    let node = ast::create_node(ast::Expr::Procedure(proc), get_empty_declarations());
    assert_eq!(node.execute().ok().unwrap(), Value::Number(0.5));
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
#[test]
fn get_expr_from_string_number_val() {
    assert_eq!(ast::get_expr_from_str("2"), Expr::Value(Value::Number(2.0)))
}
#[test]
fn get_expr_from_string_string_val() {
    assert_eq!(ast::get_expr_from_str("\"Hello, World!\""), Expr::Value(Value::String("Hello, World!".to_string())))
}
#[test]
fn get_expr_from_string_token() {
    assert_eq!(ast::get_expr_from_str("lo_x"), Expr::Token("lo_x".to_string()))
}
#[test]
fn get_expr_from_string_procedure() {
    assert_eq!(ast::get_expr_from_str("(+ 1 1)"),
        Expr::Procedure(Procedure {
            func_token: "+".to_string(),
            args: vec![
            create_val_node(Value::Number(1.0)),
            create_val_node(Value::Number(1.0))]}))
}
