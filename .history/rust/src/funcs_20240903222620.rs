use std::any::Any;

use crate::ast;

fn add<T>(node: &ast::ASTNode) -> Any {
    for n in node.to_owned().
}

pub fn lisp_func_token_to_rust(lisp_func_token: String) -> Fn {
    match lisp_func_token {
        "+" => add
    }
}