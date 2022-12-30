use crate::icon::IconType;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use tsify::Tsify;

#[derive(Tsify, Serialize, Deserialize, Clone, Debug)]
pub struct ActionEntry {
    pub action_key: String,
    pub display_name: String,
    pub skill_icon: IconType,
    pub category_icon: IconType,
    pub amount: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ActionQueue {
    pub queue: VecDeque<ActionEntry>,
}

impl Default for ActionQueue {
    fn default() -> ActionQueue {
        ActionQueue::new()
    }
}

impl ActionQueue {
    pub fn new() -> ActionQueue {
        ActionQueue { queue: VecDeque::new() }
    }

    pub fn lower_action_count(&mut self, index: u32) {
        self.queue.remove(index as usize);
    }

    pub fn append_action(&mut self, entry: ActionEntry) {
        self.queue.push_back(entry);
    }

    pub fn preppend_action(&mut self, entry: ActionEntry) {
        self.queue.push_front(entry);
    }

    pub fn get_first(&self) -> Option<&ActionEntry> {
        self.queue.front()
    }

    pub fn get_first_mut(&mut self) -> Option<&mut ActionEntry> {
        self.queue.front_mut()
    }

    pub fn delete_first(&mut self) {
        self.queue.pop_front();
    }

    pub fn clear(&mut self) {
        self.queue.clear();
    }

    pub fn is_empty(&mut self) -> bool {
        self.queue.is_empty()
    }
}
