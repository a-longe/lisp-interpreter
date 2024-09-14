use std::any::Any;

use crate::ast;

fn add(node: &ast::ASTNode) -> Box<dyn Any> {
    for n in node.children {}
}

pub fn lisp_func_token_to_rust(lisp_func_token: String) -> Fn {
    match lisp_func_token {
        "+" => add
    }
}