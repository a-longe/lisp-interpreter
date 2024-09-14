use std::any::Any;

use crate::ast;

fn add(node: &ast::ASTNode) -> f64 {
    let mut sum: Box<dyn Any> = 0.0;
    for n in node.children_nodes {
        sum += 
    }
}

pub fn lisp_func_token_to_rust(lisp_func_token: String) -> Fn {
    match lisp_func_token {
        "+" => add
    }
}