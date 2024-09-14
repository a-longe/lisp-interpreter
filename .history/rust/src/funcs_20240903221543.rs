pub mod ast;

fn add(node: ast::ASTNode) -> _{}

pub fn lisp_func_token_to_rust(lisp_func_token: String) -> Fn {
    match lisp_func_token {
        "+" => add
    }
}