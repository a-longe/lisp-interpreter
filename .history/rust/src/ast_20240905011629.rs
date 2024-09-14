use std::collections::HashMap;
use std::any::Any;
use crate::funcs

#[allow(dead_code)]
pub struct ASTNode{
    pub func_token: String,
    pub children_nodes: Vec<ASTNode>,
    pub declarations: HashMap<String, Box<dyn Any>>
}

#[allow(dead_code)]
impl ASTNode {
    fn is_leaf(&self) -> bool{
        return self.children_nodes.len() < 1;
    }

    fn execute(&self) -> Value {

    }
}