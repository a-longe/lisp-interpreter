use std::collections::HashMap;
use std::any::Any;

#[allow(dead_code)]
pub struct ASTNode{
    func_token: String,
    children_nodes: Vec<ASTNode>,
    declarations: HashMap<String, Any>
}

#[allow(dead_code)]
impl ASTNode {
    fn is_leaf(&self) -> bool{
        return self.children_nodes.len() < 1;
    }
}