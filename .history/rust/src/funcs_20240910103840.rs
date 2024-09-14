use crate::ast::{Value, Procedure, Node, Expr};

fn add(node: &Node) -> Value {
    let proc = match &node.expr {
        Expr::Procedure(p) => p,
        Expr::Value(_) => panic!("trying to apply add function to a value")
    };
    let mut sum: f64 = 0.0;
    for n in &proc.args {
        sum += match n.expr.execute() {
            Value::Number(num) => num,
            _ => {panic!("cannot add non-numbers")}
        }
    }
    Value::Number(sum)
}

fn sub(node: &Node) -> Value {
    let proc = match &node.expr {
        Expr::Procedure(p) => p,
        Expr::Value(_) => panic!("trying to apply sub function to a value")
    };
    let mut sum: f64 = 0.0;
    let mut is_first: bool = true;
    let mut num: f64;
    for n in &proc.args {
        num = match n.expr.execute() {
            Value::Number(num) => num,
            _ => panic!("cannot add non-numbers")
        };
        if is_first { 
            sum += num;
            is_first = false 
        }
        else { 
            sum -= num;
        }
    }
    Value::Number(sum) 
}

fn mult(node: &Node) -> Value{
    let mut product: f64 = 1.0;
    for n in &proc.args {
        product *= match n.expr.execute() {
            Value::Number(num) => num,
            _ => panic!("cannot mult non-numbers")
        }
    }
    return Value::Number(product);
}

fn div(node: &Node) -> Value{
    let mut quotient: f64;
    let mut num: f64;
    let mut is_first = true;
    for n in &proc.args {
        num = match n.expr.execute() {
            Value::Number(num) => num,
            _ => panic!("cannot mult non-numbers")}
        if is_first { quotient = num; }
        else { quotient /= num; }
    }
    return Value::Number(quotient);
}

pub fn lisp_func_token_to_rust(lisp_func_token: String) -> impl Fn(&Procedure) -> Value {
    match lisp_func_token.as_str() {
        "+" => add,
        "-" => sub,
        "*" => mult,
        "/" => div,
        _ => panic!("not a valid function")
    }
}