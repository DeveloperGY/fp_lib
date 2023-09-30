use std::collections::VecDeque;

pub struct NodeIdGenerator {
    next_id: usize,
    available_ids: VecDeque<usize>
}

impl NodeIdGenerator {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            available_ids: VecDeque::new()
        }
    }

    pub fn get_id(&mut self) -> Option<usize> {
        let is_id_available = !self.available_ids.is_empty();
        let can_generate_id = self.next_id < usize::MAX;

        if is_id_available {
            Some(self.available_ids.pop_front().unwrap())
        }
        else {
            if can_generate_id {
                let id = self.next_id;
                self.next_id += 1;
                Some(id)
            }
            else {
                None
            }
        }
    }

    pub fn return_id(&mut self, id: usize) {
        let is_valid_id = id < self.next_id;
        let is_already_returned = self.available_ids.contains(&id);
        
        if is_valid_id && !is_already_returned {
            self.available_ids.push_back(id);
        }
    }
}