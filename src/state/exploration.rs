use crate::types::*;
use crate::world::exploration::get_explorations as get_world_explorations;
use serde::{Deserialize, Serialize};
use std::vec::Vec;
use tsify::Tsify;

#[derive(Tsify, Serialize, Deserialize, Clone, Debug)]
pub struct Exploration {
    pub name: AllExplors,
    pub current_xp: f64,
    pub completion_percentage: f64,
    pub completion_count: u32,
    pub round_completions: u32,
    pub ticks_to_complete: u32,
    pub ticks_spent: u32,
    pub is_completed: bool,
    pub is_visible: bool,
    pub has_seen: bool,
    pub priority: u32,
    pub is_automatable: bool,
    pub is_newly_automatable: bool,
}

impl Exploration {
    pub fn new(name: AllExplors) -> Exploration {
        Exploration {
            name,
            current_xp: 0.0,
            completion_percentage: 0.0,
            completion_count: 0,
            round_completions: 0,
            ticks_to_complete: 0,
            ticks_spent: 0,
            is_completed: false,
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

pub fn get_explorations(floor: FloorTypes) -> Vec<Exploration> {
    let mut explorations = Vec::new();
    for exploration in get_world_explorations(floor) {
        explorations.push(Exploration::new(exploration.name));
    }
    // for exploration in
    explorations
}
