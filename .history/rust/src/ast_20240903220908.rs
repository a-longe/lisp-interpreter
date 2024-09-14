struct ASTNode {
    func_token: String,
    children_nodes: Vec<ASTNode>
}

impl ASTNode {
    fn is_leaf(&self) -> bool{
        return self.children_nodes.len() < 1;
    }
}