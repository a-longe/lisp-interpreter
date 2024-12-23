use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::ast::{self, ASTNode, Number, Value};

pub fn add(operands: Vec<ASTNode>) -> Option<Value> {
    let mut sum: Decimal = dec!(0.0);
    for expr in operands {
        match expr.execute() {
            Value::Number(Number{value: n}) => sum += n,
            _ => return None
        }
    }
    return Some(Value::Number(Number { value: sum }));
}
pub fn sub(operands: Vec<ASTNode>) -> Option<Value> {
    let mut sum: Decimal = dec!(0.0);
    let mut is_first = true;
    for expr in operands {
        match expr.execute() {
            Value::Number( Number {value: n}) => {
                if is_first {
                    sum = n;
                    is_first = false;
                }
                else {
                    sum -= n;
                }
            },
            _ => return None
        }
    }
    return Some(Value::Number(Number {value:sum}));
}
pub fn mult(operands: Vec<ASTNode>) -> Option<Value> {
    let mut product: Decimal = dec!(1.0);
    for expr in operands {
        product *= match expr.execute() {
            Value::Number( Number {value: n} ) => n,
            _ => return None
        }
    }
    return Some(Value::Number(Number {value: product} ));
}

pub fn div(operands: Vec<ASTNode>) -> Option<Value> {
    let mut quotient: Decimal = dec!(0);
    let mut num: Decimal;
    let mut is_first = true;
    for expr in &operands {
        num = match expr.execute() {
            Value::Number( Number {value: n} ) => n,
            _ => dec!(0) };
        if num == dec!(0) {
            return None
        }
        if is_first {
            quotient = num;
            is_first = false;
        }
        else { quotient /= num; }
    }
    if operands.len() == 1 { return Some(Value::Number( Number { value:dec!(1)/quotient } ))}
    return Some(Value::Number(Number {value:quotient}));
}

//fn rust_let(node: &Node) -> Result<Value, ast::Error> {
//    let proc = node.get_proc();
//    if proc.args.len() != 2 { return Err( Error{ reason:"Invalid Syntax: Let must have only 2 args".to_string()}); }
//    let assignments: Vec<String> = match &proc.args[0].expr {
//        Expr::Asignments(a) => a.clone(),
//        _ => panic!("first arg of let must be of type Assignments")
//    };
//    return proc.args[1].execute();
//}

pub fn symbol_to_function(lisp_func_token: String) -> impl Fn(Vec<ASTNode>) -> Option<Value> {
    match lisp_func_token.as_str() {
        "+" => add,
        "-" => sub,
        "*" => mult,
        "/" => div,
        _ => panic!("not a valid function")
    }
}
