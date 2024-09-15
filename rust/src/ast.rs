#![allow(unused_imports)]
use core::panic;
use std::collections::HashMap;
use crate::{funcs, parse::{self, get_closing_paren_index, get_tokens}};
#[derive(Clone, Debug, PartialEq)]
pub struct Procedure {
    pub func_token: String,
    pub args: Vec<Node>
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(f64),
    String(String)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Procedure(Procedure),
    Value(Value),
    Token(String),
    Asignments(Vec<String>)
}

#[derive(Clone, Debug, PartialEq)]
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

pub fn get_expr_from_str(string: &str) -> Expr {
    match string.chars().nth(0).expect("Cannot get value from string with len == 0") {
        '0'..='9' | '-' => Expr::Value(Value::Number(string.parse().unwrap())),
        '"' | '\'' => Expr::Value(Value::String(string.to_string()[1..string.to_string().len()-1].to_string())),
        '(' => Expr::Procedure(create_ast(get_tokens(string)).get_proc()),
        'A'..='z' => Expr::Token(string.to_string()),
        _ => panic!("string does not fit into patterns")
    }
}

pub fn create_ast(tokens: Vec<String>) -> Node {
    /*
    (+ 2 (* 1 2))
     ^
    not '(' -> grab token ; add to list

    (+ 2 (* 1 2))
       ^
    not '(' -> grab token ; add to list

    (+ 2 (* 1 2))
         ^------
    is '(' so recursively pass (* 1 2) back to command

    */
    let str_args = &tokens[1..tokens.len()-1].to_vec();
    println!("tokens: {:?}", str_args);
    let mut args: Vec<Node> = Vec::new();
    let mut func_token: String = String::new();
    let mut is_first = true;
    let mut i = 0;
    loop {
        if i >= str_args.len() { break; }
        let token = str_args[i].clone();
        println!("token: {}", token);
        if is_first {
            func_token = token.clone();
            is_first = false;
            i += 1;
            continue;
        }
        match token.as_str() {
            "(" => args.push(create_ast(str_args[i..=parse::get_closing_paren_index(i, str_args.clone())].to_vec())),
            _ => args.push(Node { expr: get_expr_from_str(&token), declarations: get_empty_declarations()})
        }
        i += match token.as_str() {
            "(" => parse::get_closing_paren_index(i, str_args.clone()),
            _ => 1
        }
    }
    println!("{:?}", args[0]);
    let expr = Expr::Procedure(Procedure { func_token, args });

    return Node {
        expr,
        declarations: get_empty_declarations()
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct Error {
    pub reason: String
}

#[derive(Clone, Debug, PartialEq)]
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
