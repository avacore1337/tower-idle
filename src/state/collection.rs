use crate::types::*;
use crate::world::collection::get_collections as get_world_collections;
use serde::{Deserialize, Serialize};
use std::vec::Vec;
use tsify::Tsify;

#[derive(Tsify, Serialize, Deserialize, Clone, Debug)]
pub struct Collection {
    pub name: AllCollects,
    pub current_xp: f64,
    pub completion_percentage: f64,
    pub completion_count: u32,
    pub round_completions: u32,
    pub ticks_to_complete: u32,
    pub ticks_spent: u32,
    pub is_visible: bool,
    pub has_seen: bool,
    pub priority: u32,
    pub is_automatable: bool,
    pub is_newly_automatable: bool,
}

impl Collection {
    pub fn new(name: AllCollects) -> Collection {
        Collection {
            name,
            current_xp: 0.0,
            completion_percentage: 0.0,
            completion_count: 0,
            round_completions: 0,
            ticks_to_complete: 0,
            ticks_spent: 0,
            is_visible: false,
            has_seen: false,
            priority: 2,
            is_automatable: false,
            is_newly_automatable: false,
        }
    }

    pub fn add_completion(&mut self, amount: u32) {
        self.completion_count += amount;
        self.round_completions += amount;
    }
}

pub fn get_collections(floor: FloorTypes) -> Vec<Collection> {
    let mut collections = Vec::new();
    for collection in get_world_collections(floor) {
        collections.push(Collection::new(collection.name));
    }
    // for collection in
    collections
}
