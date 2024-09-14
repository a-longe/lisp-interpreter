use std::collections::HashMap;

use crate::funcs;
#[derive(Clone)]
pub struct Procedure {
    func_token: String,
    args: Vec<Node>
}

#[derive(Clone)]
pub enum Value {
    Number(f64)
}

#[derive(Clone)]
pub enum Expr {
    Procedure(Procedure),
    Value(Value)
}

#[derive(Clone)]
pub struct Node {
    expr: Expr,
    declarations: Declarations
}

#[derive(Debug)]
pub enum Error {
    Reason(String)
}

#[derive(Clone)]
pub struct Declarations {
    data: HashMap<String, Expr>
}

#[allow(dead_code)]
impl Expr {
    pub fn execute(&self) -> Value {
        match self {
            Expr::Value(val) => return *val;
            Expr::Procedure(p) => {
                let func = funcs::lisp_func_token_to_rust(self.func_symbol);
                return func(&self);
            }
        }

    }
}