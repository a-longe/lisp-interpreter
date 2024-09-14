use crate::ast::{Value, Procedure, Node, Expr};

fn add(node: &Node) -> Value {
    let proc: Procedure = node.get_proc();
    let mut sum: f64 = 0.0;
    for n in &proc.args {
        sum += match n.execute() {
            Value::Number(num) => num,
            _ => {panic!("cannot add non-numbers")}
        }
    }
    Value::Number(sum)
}

fn sub(node: &Node) -> Value {
    let proc: Procedure = node.get_proc();
    let mut sum: f64 = 0.0;
    let mut is_first: bool = true;
    let mut num: f64;
    for n in &proc.args {
        num = match n.execute() {
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

fn mult(node: &Node) -> Value {
    let proc: Procedure = node.get_proc();
    let mut product: f64 = 1.0;
    for n in &proc.args {
        product *= match n.execute() {
            Value::Number(num) => num,
            _ => panic!("cannot mult non-numbers")
        }
    }
    return Value::Number(product);
}

fn div(node: &Node) -> Value{
    let proc: Procedure = node.get_proc();
    let mut quotient: f64 = 0.0;
    let mut num: f64;
    let mut is_first = true;
    for n in &proc.args {
        num = match n.execute() {
            Value::Number(num) => num,
            _ => panic!("cannot mult non-numbers")};
        if is_first { 
            quotient = num;
            is_first = false;
        }
        else { quotient /= num; }
    }
    return Value::Number(quotient);
}

fn rust_let(node: &Node) -> Value {
    let proc = node.get_proc();
    if proc.args.len() != 2 { panic!("Invalid Syntax: Let must have only 2 args") }
    let assignments: Vec<String> = match proc.args[0].expr {}
    Value
}

pub fn lisp_func_token_to_rust(lisp_func_token: String) -> impl Fn(&Node) -> Value {
    match lisp_func_token.as_str() {
        "+" => add,
        "-" => sub,
        "*" => mult,
        "/" => div,
        _ => panic!("not a valid function")
    }
}