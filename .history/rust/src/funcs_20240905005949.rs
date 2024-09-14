use std::any::Any;

use crate::ast;

fn add(node: &ast::ASTNode) -> f64 {
   let mut sum: f64 = 0.0;
    for n in node.children_nodes {
        sum += n.execute::<f64>() 
    }
    sum
}

pub fn lisp_func_token_to_rust(lisp_func_token: String) -> Fn {
    match lisp_func_token {
        "+" => add
    }
}