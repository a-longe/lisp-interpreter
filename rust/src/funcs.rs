use crate::ast::{self, Expr, Node, Procedure, Value, Error};

fn add(node: &Node) -> Result<Value, ast::Error> {
    let proc: Procedure = node.get_proc();
    let mut sum: f64 = 0.0;
    for n in &proc.args {
        sum += match n.execute() {
            Ok(Value::Number(num)) => num,
            Err(e) => return Err(e),
            _ => return Err(ast::Error {reason: "cannot add non-numbers".to_string()})
        }
    }
    Ok(Value::Number(sum))
}

fn sub(node: &Node) -> Result<Value, ast::Error> {
    let proc: Procedure = node.get_proc();
    let mut sum: f64 = 0.0;
    let mut is_first: bool = true;
    let mut num: f64;
    for n in &proc.args {
        num = match n.execute() {
            Ok(Value::Number(num)) => num,
            Err(e) => return Err(e),
            _ => return Err(ast::Error {reason: "cannot sub non-numbers".to_string()})
        };
        if is_first {
            sum += num;
            is_first = false
        }
        else {
            sum -= num;
        }
    }
    Ok(Value::Number(sum))
}

fn mult(node: &Node) -> Result<Value, ast::Error>{
    let proc: Procedure = node.get_proc();
    let mut product: f64 = 1.0;
    for n in &proc.args {
        product *= match n.execute() {
            Ok(Value::Number(num)) => num,
            Err(e) => return Err(e),
            _ => return Err(Error{reason: "cannot multiply by non-numbers".to_string()}),
        }
    }
    return Ok(Value::Number(product));
}

fn div(node: &Node) -> Result<Value, ast::Error> {
    let proc: Procedure = node.get_proc();
    let mut quotient: f64 = 0.0;
    let mut num: f64;
    let mut is_first = true;
    for n in &proc.args {
        num = match n.execute() {
            Ok(Value::Number(num)) => num,
            Err(e) => return Err(e),
            _ => return Err(Error{ reason: "cannot mult non-numbers".to_string()})};
        if !is_first && num == 0.0 {
            return Err(Error { reason: "division by zero".to_string()});
        }
        if is_first {
            quotient = num;
            is_first = false;
        }
        else { quotient /= num; }
    }
    return Ok(Value::Number(quotient));
}

fn rust_let(node: &Node) -> Result<Value, ast::Error> {
    let proc = node.get_proc();
    if proc.args.len() != 2 { return Err( Error{ reason:"Invalid Syntax: Let must have only 2 args".to_string()}); }
    let assignments: Vec<String> = match &proc.args[0].expr {
        Expr::Asignments(a) => a.clone(),
        _ => panic!("first arg of let must be of type Assignments")
    };
    return proc.args[1].execute();
}
pub fn lisp_func_token_to_rust(lisp_func_token: String) -> impl Fn(&Node) -> Result<Value, ast::Error> {
    match lisp_func_token.as_str() {
        "+" => add,
        "-" => sub,
        "*" => mult,
        "/" => div,
        _ => panic!("not a valid function")
    }
}
