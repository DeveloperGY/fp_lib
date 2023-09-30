pub trait TreeBase<T> {
    fn get_root_node_id(&self) -> usize;

    fn insert(&mut self, parent_id: usize, value: T) -> Option<usize>;

    fn get_node_value(&self, node_id: usize) -> Option<&T>;
    fn get_node_value_mut(&mut self, node_id: usize) -> Option<&mut T>;

    fn get_node_children_ids(&self, node_id: usize) -> Option<&[usize]>;

    fn remove(&mut self, node_id: usize);
}