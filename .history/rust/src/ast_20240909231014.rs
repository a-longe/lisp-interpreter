use std::collections::HashMap;
use std::any::Any;

impl Clone for Vec<T> {}

struct Procedure {}
enum Value {}

enum Expr {
    Procedure(Procedure),
    Value(Value)
}

#[derive(Clone)]
pub struct Node {
    func_symbol: String,
    args: Vec<Expr>
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
pub struct ASTNode{
    pub func_token: String,
    pub children_nodes: Vec<ASTNode>,
    pub declarations: HashMap<String, Box<dyn Any>>
}

#[allow(dead_code)]
impl ASTNode {
    pub fn is_leaf(&self) -> bool{
        return self.children_nodes.len() < 1;
    }

    pub fn execute(&self) -> Expr {
        Expr::Number(0.0)
    }
}