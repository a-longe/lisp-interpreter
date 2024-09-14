use std::collections::HashMap;

#[allow(dead_code)]
pub struct ASTNode<T> {
    func_token: String,
    children_nodes: Vec<ASTNode>,
    declarations: HashMap<String, T>
}

#[allow(dead_code)]
impl ASTNode {
    fn is_leaf(&self) -> bool{
        return self.children_nodes.len() < 1;
    }
}