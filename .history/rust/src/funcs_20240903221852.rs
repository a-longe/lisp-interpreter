use crate::ast;

fn add<T>(node: &ast::ASTNode) -> T {
    for n in 
}

pub fn lisp_func_token_to_rust(lisp_func_token: String) -> Fn {
    match lisp_func_token {
        "+" => add
    }
}