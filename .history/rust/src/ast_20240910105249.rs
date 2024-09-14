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
    Value(Value),
    Token(String)
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
    data: HashMap<String, Value>
}
impl Declarations {
    fn get_value(&self, token: String) -> Value {
        match self.data.get(&token) {
            Some(v) => v.clone(),
            None => panic!("Cannot find var in scope")
        }
    }
}

#[allow(dead_code)]
impl Node {
    pub fn execute(&self) -> Value {
        match &self.expr {
            Expr::Value(val) => return val.clone(),
            Expr::Procedure(p) => {
                let func = funcs::lisp_func_token_to_rust(p.func_token.clone());
                return func(&self);
            Expr::Token(token)
            }
        }

    }
}