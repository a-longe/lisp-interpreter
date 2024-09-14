use std::collections::HashMap;

use crate::funcs;
#[derive(Clone)]
pub struct Procedure {
    pub func_token: String,
    pub args: Vec<Node>
}

#[derive(Clone)]
pub enum Value {
    Number(f64),
    String(String)
}

#[derive(Clone)]
pub enum Expr {
    Procedure(Procedure),
    Value(Value)
}

#[derive(Clone)]
pub struct Node {
    pub expr: Expr,
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
impl Declarations {
    fn get_value(&self, ) ->
}

#[allow(dead_code)]
impl Expr {
    pub fn execute(&self) -> Value {
        match self {
            Expr::Value(val) => return *val,
            Expr::Procedure(p) => {
                let func = funcs::lisp_func_token_to_rust(p.func_token);
                return func(&self);
            }
        }

    }
}