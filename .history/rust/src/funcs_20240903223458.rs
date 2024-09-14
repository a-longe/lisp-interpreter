use std::any::Any;

use crate::ast;

fn add<T>(node: &ast::ASTNode) -> Box<dyn Any> {
    let mut sum: T;
    for n in node.children_nodes {
        sum += n.execute() 
    }
    sum
}

pub fn lisp_func_token_to_rust(lisp_func_token: String) -> Fn {
    match lisp_func_token {
        "+" => add
    }
}