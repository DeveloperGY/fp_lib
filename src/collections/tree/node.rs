pub struct TreeNode<T> {
    pub value: T,
    pub parent_id: Option<usize>,
    pub children_ids: Vec<usize>,
    pub node_id: usize
}

impl<T> TreeNode<T> {
    pub fn new(node_id: usize, value: T, parent_id: Option<usize>) -> Self {
        Self {
            value,
            parent_id,
            children_ids: vec![],
            node_id
        }
    }
}