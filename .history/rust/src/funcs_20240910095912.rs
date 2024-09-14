use crate::ast::{self, Node, Value, Expr};

fn add(node: &ast::Procedure) -> Value {
    let mut sum: f64 = 0.0;
    for n in &node.args {
        match n {
            Expr::Procedure(p) => {p.execute()}
        }
    }
    Value::Number(sum)
}

fn sub(node: &ast::Procedure) -> Value {
    let mut sum: f64 = 0.0;
    let mut is_first: bool = true;
    for n in node.children_nodes {
        if is_first { sum += n.execute().into(); }
        else { sum += n.execute().into(); }
    }
    Value::Number(sum) 
}

pub fn lisp_func_token_to_rust(lisp_func_token: String) -> impl Fn(&Procedure) -> Value {
    match lisp_func_token {
        "+" => add,
        "-" => sub,
        "*" => mult,
        "/" => div
    }
}