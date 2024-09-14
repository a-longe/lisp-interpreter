use std::collections::HashMap;
use std::any::Any;

use crate::funcs;
#[derive(Clone)]
struct Procedure {}

#[derive(Clone)]
enum Value {}

#[derive(Clone)]
enum Expr {
    Procedure(Procedure),
    Value(Value)
}

#[derive(Clone)]
pub struct Node {
    func_symbol: String,
    args: Vec<Expr>,
    declarations: Declarations
}

#[derive(Debug)]
enum Error {
    Reason(String)
}

#[derive(Clone)]
struct Declarations {
    data: HashMap<String, Expr>
}

#[allow(dead_code)]
impl Node {
    pub fn execute(&self) -> Value {
        func = funcs::lisp_func_token_to_rust(self.func_symbol)
    }
}