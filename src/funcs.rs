use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::collections::HashMap;

use crate::ast::{self, ASTNode, BaseProcedure, Declarations, Expression, Literal, Number, Procedure, Symbol, UserProcedure, Value};
use crate::racket_error::{Error, create_error};

pub fn add(declarations: &mut Declarations, operands: Vec<ASTNode>) -> Result<Value, Error> {
    let mut sum: Decimal = dec!(0.0);
    for expr in operands {
        match expr.execute(declarations) {
            Value::Number(Number{value: n}) => sum += n,
            _ => return Err(create_error("cannot add non-numbers")),
        }
    }
    return Ok(Value::Number(Number { value: sum }));
}
pub fn sub(declarations: &mut Declarations, operands: Vec<ASTNode>) -> Result<Value, Error> {
    let mut sum: Decimal = dec!(0.0);
    let mut is_first = true;
    for expr in operands {
        match expr.execute(declarations) {
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
pub fn mult(declarations: &mut Declarations, operands: Vec<ASTNode>) -> Result<Value, Error> {
    let mut product: Decimal = dec!(1.0);
    for expr in operands {
        product *= match expr.execute(declarations) {
            Value::Number( Number {value: n} ) => n,
            _ => return Err(create_error("cannot mult non-numbers"))
        }
    }
    return Ok(Value::Number(Number {value: product} ));
}

pub fn div(declarations: &mut Declarations, operands: Vec<ASTNode>) -> Result<Value, Error> {
    let mut quotient: Decimal = dec!(0);
    let mut num: Decimal;
    let mut is_first = true;
    for expr in &operands {
        num = match expr.execute(declarations) {
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

pub fn rust_lambda(declarations: &mut Declarations, operands: Vec<ASTNode>) -> Result<Value, Error> {
    if operands.len() != 2 { return Err(create_error(&format!("Iwvalid Syntax - Expected 2 \n Got {}", operands.len()))) }

    let ASTNode { pos:_, expr: boxed_expr, ..} = &operands[0];
    if let Expression::Literal(Literal::Symbol(arg_id)) = boxed_expr.as_ref() {
        return Ok(Value::Proc(Procedure::UserProc( UserProcedure::new(operands[1].clone(), arg_id.clone()) )));
    }
    else {
        return Err(create_error("Bad Argument"));
    }
}

pub fn symbol_to_function(lisp_func: &Symbol) -> Option<Value> {
    if let Some(func) = get_base_global_scope().get(lisp_func) {
        return Some(func.clone())
    }
    else {
        return None
    }
}

pub fn get_base_global_scope() -> HashMap<Symbol, Value> {
    let mut hm: HashMap<Symbol, Value> = HashMap::new();
    hm.insert(Symbol::new("+"), Value::Proc(Procedure::BaseProc(BaseProcedure::new(add))));
    hm.insert(Symbol::new("-"), Value::Proc(Procedure::BaseProc(BaseProcedure::new(sub))));
    hm.insert(Symbol::new("*"), Value::Proc(Procedure::BaseProc(BaseProcedure::new(mult))));
    hm.insert(Symbol::new("/"), Value::Proc(Procedure::BaseProc(BaseProcedure::new(div))));
    hm.insert(Symbol::new("lambda"), Value::Proc(Procedure::BaseProc(BaseProcedure::new(rust_lambda))));
    hm
}

