use crate::ast::{Value, Procedure};

fn add(proc: &Procedure) -> Value {
    let mut sum: f64 = 0.0;
    for n in &proc.args {
        sum += match n.expr.execute() {
            Value::Number(num) => num,
            _ => {panic!("cannot add non-numbers")}
        }
    }
    Value::Number(sum)
}

fn sub(proc: &Procedure) -> Value {
    let mut sum: f64 = 0.0;
    let mut is_first: bool = true;
    for n in &proc.args {
        if is_first { 
            sum += match n.expr.execute() {
                Value::Number(num) => num,
                _ => {panic!("cannot add non-numbers")}
            };
            is_first = false 
        }
        else { 
            sum -= match n.expr.execute() {
                Value::Number(num) => num,
                _ => {panic!("cannot add non-numbers")}
            };
        }
    }
    Value::Number(sum) 
}

fn mult(proc: &Procedure) {
    let mut product: f64 = 1.0;
}

pub fn lisp_func_token_to_rust(lisp_func_token: String) -> impl Fn(&Procedure) -> Value {
    match lisp_func_token.as_str() {
        "+" => add,
        "-" => sub,
        "*" => mult,
        "/" => div
    }
}