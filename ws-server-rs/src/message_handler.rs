use std::collections::VecDeque;

// This File implements Logic to store the last 5 messages per Sender

pub struct MessageHandler {
    messages: VecDeque<String>,
}

impl MessageHandler {
    pub fn new() -> Self {
        Self {
            messages: VecDeque::new(),
        }
    }

    pub fn add_message(&mut self, message: String) {
        if self.messages.len() >= 5 {
            self.messages.pop_front();
        }
        self.messages.push_back(message);
    }

    pub fn get_last_five(&self) -> VecDeque<String> {
        self.messages.clone()
    }
}
