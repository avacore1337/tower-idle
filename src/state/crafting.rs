use crate::types::*;
use crate::world::crafting::get_craftings as get_world_craftings;
use serde::{Deserialize, Serialize};
use std::vec::Vec;
use tsify::Tsify;

#[derive(Tsify, Serialize, Deserialize, Clone, Debug)]
pub struct Crafting {
    pub name: AllCrafts,
    pub segments_paid: u32,
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
    pub favourite: bool,
}

impl Crafting {
    pub fn new(name: AllCrafts) -> Crafting {
        Crafting {
            name,
            segments_paid: 0,
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
            favourite: true,
        }
    }

    pub fn add_completion(&mut self, amount: u32) {
        self.completion_count += amount;
        self.round_completions += amount;
    }
}

pub fn get_craftings(floor: FloorTypes) -> Vec<Crafting> {
    let mut craftings = Vec::new();
    for crafting in get_world_craftings(floor) {
        craftings.push(Crafting::new(crafting.name));
    }
    craftings
}
