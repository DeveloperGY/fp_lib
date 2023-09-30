use super::TreeNode;

pub struct TreeNodeBuilder {
    parent_id: Option<usize>,
    node_id: Option<usize>
}

impl TreeNodeBuilder {
    pub fn new() -> Self {
        Self {
            parent_id: None,
            node_id: None
        }
    }

    pub fn parent_id(&mut self, id: Option<usize>) -> &mut Self {
        self.parent_id = id;
        self
    }

    pub fn node_id(&mut self, id: usize) -> &mut Self {
        self.node_id = Some(id);
        self
    }

    pub fn build<T>(&self, value: T) -> Option<TreeNode<T>> {
        if let Some(node_id) = self.node_id {
            let node = TreeNode::new(node_id, value, self.parent_id);
            Some(node)
        }
        else {
            None
        }
    }
}