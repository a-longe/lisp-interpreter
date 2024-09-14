use std::collections::HashMap;
use std::any::Any;

pub enum Value {
    String(String),
    Number(f64)
}

impl Into<f64> for Value {
    fn into(self) -> f64 {

    }
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

    pub fn execute(&self) -> Value {
        Value::Number(0.0)
    }
}