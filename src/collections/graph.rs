use crate::util::IDGen;

#[derive(Debug, Clone)]
pub struct Graph<T> {
    nodes: Vec<Option<T>>,
    adj_mat: Vec<Vec<bool>>, // change to Option<usize> for weights TODO: Add in weighted graph

    id_gen: IDGen
}

impl<T> Graph<T> {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            adj_mat: vec![],

            id_gen: IDGen::new()
        }
    }

    /**
     * Returns a node id in the form of a [`usize`]
     */
    pub fn add_node(&mut self, val: T) -> Result<usize, String> {
        if let Ok(id) = self.id_gen.get_id() {
            if id == self.nodes.len() { // Need more space for node
                self.nodes.push(Some(val));
                self.adj_mat.iter_mut()
                    .for_each(|edges| {edges.push(false)});

                let mut new_node_edges = vec![];
                (0..self.nodes.len()).into_iter()
                    .for_each(|_| {new_node_edges.push(false)});
                self.adj_mat.push(new_node_edges);

                Ok(id)
            }
            else if id < self.nodes.len() { // we have space for node
                self.nodes[id] = Some(val);
                Ok(id)
            }
            else { // Getting this should be impossible with the current implementation
                panic!("It should not be possible for you to get this so if you did, congratulate yourself!");
            }
        }
        else {
            Err("Max Nodes Reached!".into())
        }
    }

    /**
     * Verifies that the node exists
     */
    fn validate_node_id(&self, node_id: usize) -> bool {
        if node_id < self.nodes.len() {
            !matches!(self.nodes[node_id], None)
        }
        else {
            false
        }
    }

    pub fn remove_node(&mut self, node_id: usize) {
        if self.validate_node_id(node_id) {
            self.nodes[node_id] = None;
            self.adj_mat[node_id].iter_mut()
                .for_each(|edge| {*edge = false});
            self.adj_mat.iter_mut()
                .for_each(|edge_list| {edge_list[node_id] = false});

            self.id_gen.return_id(node_id);
        }
    }

    pub fn add_edge(&mut self, src_node: usize, dest_node: usize) {
        if !self.validate_node_id(src_node) || !self.validate_node_id(dest_node) {return};

        self.adj_mat[src_node][dest_node] = true;
    }

    pub fn add_dual_edge(&mut self, node_0: usize, node_1: usize) {
        if !self.validate_node_id(node_0) || !self.validate_node_id(node_1) {return}; 

        self.add_edge(node_0, node_1);
        self.add_edge(node_1, node_0);
    }

    pub fn remove_edge(&mut self, src_node: usize, dest_node: usize) {
        if !self.validate_node_id(src_node) || !self.validate_node_id(dest_node) {return};

        self.adj_mat[src_node][dest_node] = false;
    }

    pub fn remove_dual_edge(&mut self, node_0: usize, node_1: usize) {
        if !self.validate_node_id(node_0) || !self.validate_node_id(node_1) {return}; 

        self.remove_edge(node_0, node_1);
        self.remove_edge(node_1, node_0);
    }

    /**
     * Returns a vector of the ids of the nodes the given node is connected to
     */
    pub fn connected_nodes(&self, node_id: usize) -> Option<Vec<usize>> {
        if !self.validate_node_id(node_id) {return None};

        let mut connected_nodes = vec![];

        (0..self.adj_mat[node_id].len()).into_iter()
            .for_each(|id| {if self.adj_mat[node_id][id] {connected_nodes.push(id)}});

        Some(connected_nodes)
    }

    /**
     * Returns a reference to the value of a node
     */
    pub fn get(&self, node_id: usize) -> Option<&T> {
        if !self.validate_node_id(node_id) {return None};

        self.nodes[node_id].as_ref()
    }

    /**
     * Returns a mutable reference to the value of a node
     */
    pub fn get_mut(&mut self, node_id: usize) -> Option<&mut T> {
        if !self.validate_node_id(node_id) {return None};

        self.nodes[node_id].as_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::Graph;

    #[test]
    fn social_network() {
        let mut social_network: Graph<String> = Graph::new();

        let john = social_network.add_node("John Stalberg".into()).unwrap();
        let mary = social_network.add_node("Mary Poppins".into()).unwrap();
        social_network.add_dual_edge(john, mary);

        let johns_friends = social_network.connected_nodes(john);

        // println!("Johns Friends:");

        let mut johns_friends_names = vec![];

        if let Some(friends) = johns_friends {
            friends.iter()
                .for_each(|id| {
                    let name = social_network.get(*id).unwrap();
                    johns_friends_names.push(name.clone());
                    // println!("\t{}", name);
                });
        }

        assert_eq!(johns_friends_names, [String::from("Mary Poppins")]);
    }
}