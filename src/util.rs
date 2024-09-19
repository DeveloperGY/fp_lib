#![allow(unused)]

use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct IDGen {
    current_id: usize,
    available_ids: VecDeque<usize>,
}

impl IDGen {
    pub fn new() -> Self {
        Self {
            current_id: 0,
            available_ids: VecDeque::new(),
        }
    }

    pub fn get_id(&mut self) -> Result<usize, String> {
        if self.current_id == usize::MAX {
            return Err("Max ID count reached!".into());
        }

        match self.available_ids.pop_front() {
            Some(val) => Ok(val),
            None => {
                let id = self.current_id;
                self.current_id += 1;
                Ok(id)
            }
        }
    }

    pub fn return_id(&mut self, id: usize) {
        if id < self.current_id && !self.available_ids.contains(&id) {
            self.available_ids.push_back(id);
        }
    }

    pub fn reset(&mut self) {
        self.current_id = 0;
        self.available_ids.clear();
    }
}
