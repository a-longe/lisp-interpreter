use std::collections::HashMap;
use std::any::Any;

use crate::funcs;
#[derive(Clone)]
pub struct Procedure {}

#[derive(Clone)]
pub enum Value {}

#[derive(Clone)]
pub enum Expr {
    Procedure(Procedure),
    Value(Value)
}

#[derive(Clone)]
pub struct Node {
    args: Vec<Expr>,
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
impl Procedure {
    pub fn execute(&self) -> Value {
        let func = funcs::lisp_func_token_to_rust(self.func_symbol);
        return func(&self);
    }
}