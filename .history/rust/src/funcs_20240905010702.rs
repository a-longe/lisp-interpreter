use std::any::Any;

use crate::ast::{self, ASTNode};

enum Value {
    String,
    Number
}

fn add(node: &ast::ASTNode) -> Value {
   let mut sum: f64 = 0.0;
    for n in node.children_nodes {
        sum += n.execute() 
    }
    sum
}

fn sub(node: &ast::ASTNode) -> i64 {
   let mut sum: f64 = 0.0;
    for n in &node.children_nodes[1..] {
        sum += n.execute::<f64>() 
    }
    sum
}

pub fn lisp_func_token_to_rust(lisp_func_token: String) -> impl Fn(ASTNode) -> _ {
    match lisp_func_token {
        "+" => add,
        "-" => sub
    }
}