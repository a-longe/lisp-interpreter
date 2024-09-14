use std::any::Any;

use crate::ast;

fn add(node: &ast::ASTNode) -> f64 {
    let mut sum: f64 = 0.0;
    for n in node.children_nodes {
        sum += n.execute() as f64;
    }
    return sum
}

pub fn lisp_func_token_to_rust(lisp_func_token: String) -> for<'a> fn(&'a ASTNode) -> Box<dyn Any>{
    match lisp_func_token {
        "+" => add
    }
}