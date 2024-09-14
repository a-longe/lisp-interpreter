use std::collections::HashMap;
use std::any::Any;

#[allow(dead_code)]
pub struct ASTNode{
    pub func_token: String,
    children_nodes: Vec<ASTNode>,
    declarations: HashMap<String, Box<dyn Any>>
}

#[allow(dead_code)]
impl ASTNode {
    fn is_leaf(&self) -> bool{
        return self.children_nodes.len() < 1;
    }
}