use core::panic;
use std::collections::HashMap;

use crate::{funcs, parse};
#[derive(Clone)]
pub struct Procedure {
    pub func_token: String,
    pub args: Vec<Node>
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(f64),
    String(String)
}

#[derive(Clone)]
pub enum Expr {
    Procedure(Procedure),
    Value(Value),
    Token(String),
    Asignments(Vec<String>)
}

#[derive(Clone)]
pub struct Node {
    pub expr: Expr,
    declarations: Declarations
}

pub fn create_node(expr: Expr, declarations: Declarations) -> Node {
    return Node {
        expr,
        declarations
    };
}
pub fn create_val_node(val: Value) -> Node {
    create_node(Expr::Value(val), get_empty_declarations())
}

pub fn create_ast(command: &str) -> Node {
    let tokens = parse::get_tokens(command);
    println!("{:?}", tokens);
    let func_tok = tokens[1].clone();
    let arg1: f64 = tokens[2].parse().unwrap();
    let arg2: f64 = tokens[3].parse().unwrap();
    return Node {
        expr: Expr::Procedure(Procedure {
            func_token: func_tok,
            args: vec![
                create_val_node(Value::Number(arg1)),
                create_val_node(Value::Number(arg2))] }),
        declarations: get_empty_declarations()
    };
}

#[derive(Debug, PartialEq, Eq)]
pub struct Error {
    pub reason: String
}

#[derive(Clone)]
pub struct Declarations {
    pub data: HashMap<String, Value>
}
impl Declarations {
    fn get_value(&self, token: &str) -> Result<Value, Error> {
        match self.data.get(token) {
            Some(v) => Ok(v.clone()),
            None => Err(Error{ reason:"Cannot find var in scope".to_string()})
        }
    }
}

pub fn get_empty_declarations() -> Declarations {
    Declarations {
        data: HashMap::new()
    }
}

impl Node {
    pub fn execute(&self) -> Result<Value, Error> {
        match &self.expr {
            Expr::Value(val) => return Ok(val.clone()),
            Expr::Procedure(p) => {
                let func = funcs::lisp_func_token_to_rust(p.func_token.clone());
                return func(&self);
            }
            Expr::Token(token) => { Ok(self.declarations.get_value(&token.clone()))?}
            Expr::Asignments(_) => { Err(Error{ reason:"canot execute assignments expression".to_string()})}
        }

    }

    pub fn get_proc(&self) -> Procedure {
        return match &self.expr {
        Expr::Procedure(p) => p.clone(),
        _ => panic!("trying to apply function to a value or token")
        };
    }

}
