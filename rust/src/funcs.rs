use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::ast::{self, ASTNode, Expression, Literal, Number, Value};
use crate::racket_error::*;

pub fn add(operands: Vec<ASTNode>) -> Result<Value, Error> {
    let mut sum: Decimal = dec!(0.0);
    for expr in operands {
        match expr.execute() {
            Value::Number(Number{value: n}) => sum += n,
            _ => return Err(create_error("cannot add non-numbers")),
        }
    }
    return Ok(Value::Number(Number { value: sum }));
}
pub fn sub(operands: Vec<ASTNode>) -> Result<Value, Error> {
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
            _ => return Err(create_error("cannot sub non-numbers"))
        }
    }
    return Ok(Value::Number(Number {value:sum}));
}
pub fn mult(operands: Vec<ASTNode>) -> Result<Value, Error> {
    let mut product: Decimal = dec!(1.0);
    for expr in operands {
        product *= match expr.execute() {
            Value::Number( Number {value: n} ) => n,
            _ => return Err(create_error("cannot mult non-numbers"))
        }
    }
    return Ok(Value::Number(Number {value: product} ));
}

pub fn div(operands: Vec<ASTNode>) -> Result<Value, Error> {
    let mut quotient: Decimal = dec!(0);
    let mut num: Decimal;
    let mut is_first = true;
    for expr in &operands {
        num = match expr.execute() {
            Value::Number( Number {value: n} ) => n,
            _ => return Err(create_error("cannot div non-numbers"))
        };
        if num == dec!(0) && !is_first {
            return Err(create_error("cannot divide by zero"))
        }
        if is_first {
            quotient = num;
            is_first = false;
        }
        else { quotient /= num; }
    }
    if operands.len() == 1 { return Ok(Value::Number( Number { value:dec!(1)/quotient } ))}
    return Ok(Value::Number(Number {value:quotient}));
}

pub fn rust_let(mut operands: Vec<ASTNode>) -> Result<Value, Error> {
    if operands.len() != 2 { return Err(create_error("let function can only take two operands")); }
    let mut new_def = (*operands[0].declarations).clone();
    let mut assignments: Vec<ASTNode> = Vec::new();
    match *operands[0].expr.clone() {
        Expression::ProcedureCall(p) => {
            // p is the inner assignment proc call
            // so both the operator and operands are assignments
            assignments.push(p.clone().operator);
            for o in &p.operands {
                assignments.push(o.clone());
            }
        },
        _ => return Err(create_error("invalid assignment syntax"))
    }
    let body = &mut operands[1];
    // update new_def with new assignment
    for assignment in assignments {
        match *assignment.expr {
            Expression::ProcedureCall(p) => {
                if p.operands.len() != 1 { return Err(create_error("invalid assignment syntax: variable can only be assigned to one value")); }
                match p.operator.execute() {
                    Value::Symbol(s) => {
                        new_def.add(s, p.operands[0].execute());
                        body.declarations = Box::new(new_def);
                        return Ok(body.execute());
                    },
                    _ => return Err(create_error("Cannot name variable any type other than Symbol"))
                }
            },
            _ => return Err(create_error("invalid assignment syntax: cannot have assignments be a literal"))
        };
    }
    return Err(create_error("should never reach, no assignments to parse"))
}

pub fn symbol_to_function(lisp_func_token: String) -> impl Fn(Vec<ASTNode>) -> Result<Value, Error> {
    match lisp_func_token.as_str() {
        "+" => add,
        "-" => sub,
        "*" => mult,
        "/" => div,
        "let" => rust_let,
        _ => panic!("not a valid function")
    }
}
