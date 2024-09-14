use std::collections::HashMap;
use std::any::Any;

#[derive(Clone)]
pub enum Expr {
    Symbol(String),
    Number(f64),
    List(Vec<Expr>)
}

#[derive(Debug)]
enum Error {
    Reason(String)
}

#[derive(Clone)]
struct Declarations {
    data: HashMap<String, Expr>
}