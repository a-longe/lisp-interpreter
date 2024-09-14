use crate::ast::{self, Node, Value, Expr, Procedure};

fn add(node: &Procedure) -> Value {
    let mut sum: f64 = 0.0;
    for n in &node.args {
        match n.expr.execute() {
            Value::Number(num) => sum += num,
            _ => {panic!("cannot add non-numbers")}
        }
    }
    Value::Number(sum)
}

fn sub(node: &Procedure) -> Value {
    let mut sum: f64 = 0.0;
    let mut is_first: bool = true;
    for n in &node.args {
        if is_first { sum -= n.expr.execute(); }
        else { sum += n.execute(); }
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