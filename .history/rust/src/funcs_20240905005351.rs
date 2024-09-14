use std::any::Any;

use crate::ast;

fn add(node: &ast::ASTNode) -> T {
    sum = 0;
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