use std::any::Any;

use crate::ast::{self, ASTNode};

fn add(node: &ast::ASTNode) -> f64 {
   let mut sum: f64 = 0.0;
    for n in node.children_nodes {
        sum += n.execute::<f64>() 
    }
    sum
}

fn sub(node: &ast::ASTNode) -> f64 {
   let mut sum: f64 = 0.0;
    for n in &node.children_nodes[1..] {
        sum += n.to_owned().ex 
    }
    sum
}

pub fn lisp_func_token_to_rust(lisp_func_token: String) -> impl Fn(ASTNode) -> _ {
    match lisp_func_token {
        "+" => add,
        "-" => sub
    }
}