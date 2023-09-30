mod node;
mod node_builder;
mod tree_base;
mod node_id_generator;

use node_id_generator::NodeIdGenerator;
use node::TreeNode;
use tree_base::TreeBase;
use node_builder::TreeNodeBuilder;

/// A general tree data structure
/// 
/// For a thread-safe implementation use [`ATree`] instead
pub struct Tree<T> {
    nodes: Vec<TreeNode<T>>,
    id_gen: NodeIdGenerator,
    root_node_id: usize
}

impl<T> Tree<T> {
    pub fn new(root_value: T) -> (Self, usize) {
        let mut id_gen = NodeIdGenerator::new();

        let mut nodes = Vec::new();

        let root_id = id_gen.get_id().unwrap();
        let root_node = TreeNode::new(root_id, root_value, None);

        nodes.push(root_node);

        (
            Self {
                nodes: nodes,
                id_gen,
                root_node_id: root_id
            },
            root_id
        )
    }
}

impl<T> TreeBase<T> for Tree<T> {
    fn get_root_node_id(&self) -> usize {
        self.root_node_id
    }

    fn insert(&mut self, parent_id: usize, value: T) -> Option<usize> {
        let is_valid_parent = parent_id < self.nodes.len();

        if !is_valid_parent {
            return None;
        }
        
        let node_id = if let Some(id) = self.id_gen.get_id() {
            id
        }
        else {
            return None
        };

        let mut tree_node_builder = TreeNodeBuilder::new();

        let node = tree_node_builder.node_id(node_id)
            .parent_id(Some(parent_id))
            .build(value)
            .unwrap();

        self.nodes[parent_id].children_ids.push(node_id);

        if node_id == self.nodes.len() {
            self.nodes.push(node);
        }
        else {
            self.nodes[node_id] = node;
        }

        Some(node_id)
    }

    fn get_node_value(&self, node_id: usize) -> Option<&T> {
        let is_valid_id = node_id < self.nodes.len();

        if is_valid_id {
            Some(&self.nodes[node_id].value)
        }
        else {
            None
        }
    }

    fn get_node_value_mut(&mut self, node_id: usize) -> Option<&mut T> {
        let is_valid_id = node_id < self.nodes.len();

        if is_valid_id {
            Some(&mut self.nodes[node_id].value)
        }
        else {
            None
        }
    }

    fn get_node_children_ids(&self, node_id: usize) -> Option<&[usize]> {
        let is_valid_id = node_id < self.nodes.len();

        if is_valid_id {
            Some(&self.nodes[node_id].children_ids[..])
        }
        else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Tree, tree_base::TreeBase};
    
    #[test]
    fn tree() {
        let (mut tree, root_node) = Tree::new(0);

        let node_0 = tree.insert(root_node, 1).unwrap();

        let node_1 = tree.insert(node_0, 2).unwrap();

        let node_2 = tree.insert(root_node, 3).unwrap();

        let children = tree.get_node_children_ids(root_node).unwrap();

        for child_id in children {
            let value = tree.get_node_value(*child_id).unwrap();

            println!("Id: {}\nValue: {}\n\n", child_id, value);
        }
    }
}