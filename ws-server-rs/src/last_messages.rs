// last_messages.rs

use std::collections::{HashMap, VecDeque};

pub struct LastMessages {
    messages: HashMap<usize, VecDeque<String>>,
    limit: usize,
}

impl LastMessages {
    pub fn new(limit: usize) -> Self {
        LastMessages {
            messages: HashMap::new(),
            limit,
        }
    }

    pub fn add_message(&mut self, sender_id: usize, message: String) {
        let sender_messages = self.messages.entry(sender_id).or_insert_with(VecDeque::new);
        if sender_messages.len() == self.limit {
            sender_messages.pop_front();
        }
        sender_messages.push_back(message);
    }

    pub fn get_messages(&self, sender_id: usize) -> Option<&VecDeque<String>> {
        self.messages.get(&sender_id)
    }
}
