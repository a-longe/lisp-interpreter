use std::any::Any;

use crate::ast;

fn add(node: &ast::ASTNode) -> Box<dyn Any> {
    node.
}

pub fn lisp_func_token_to_rust(lisp_func_token: String) -> Fn {
    match lisp_func_token {
        "+" => add
    }
}