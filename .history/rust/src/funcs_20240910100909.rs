use crate::ast::{self, Node, Value, Expr, Procedure};

fn add(node: &Procedure) -> Value {
    let mut sum: f64 = 0.0;
    for n in &node.args {
        sum += match n.expr.execute() {
            Value::Number(num) => num,
            _ => {panic!("cannot add non-numbers")}
        }
    }
    Value::Number(sum)
}

fn sub(node: &Procedure) -> Value {
    let mut sum: f64 = 0.0;
    let mut is_first: bool = true;
    for n in &node.args {
        if is_first { 
            sum += match n.expr.execute() {
                Value::Number(num) => num,
                _ => {panic!("cannot add non-numbers")}
            }
            is_first = false 
        }
        else { 
            sum -= match n.expr.execute() {
                Value::Number(num) => num,
                _ => {panic!("cannot add non-numbers")}
            } 
        }
    }
    Value::Number(sum) 
}

pub fn lisp_func_token_to_rust(lisp_func_token: String) -> impl Fn(&Procedure) -> Value {
    match lisp_func_token {
        "+".to_string() => add,
        "-".to_string() => sub,
        "*".to_string() => mult,
        "/".to_string() => div
    }
}