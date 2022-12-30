use crate::engine::value_keys::KeyValues;
use crate::game::Game;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Tsify, Serialize, Deserialize, Clone, Debug)]
pub struct Status {
    pub tick: u32,
    pub inventory_size: f64,
    pub current_health: f64,
    pub max_health: f64,
    pub current_health_percentage: f64,
    pub playtime_health_earned: f64,
    pub starting_health: f64,
    pub health_drain: f64,
    pub is_dead: bool,
    pub reincarnation: u32,
    pub waiting: bool,
}

const BASE_STARTING_HEALTH: f64 = 10.0;

impl Status {
    pub fn new() -> Status {
        Status {
            inventory_size: 10.0,
            current_health: BASE_STARTING_HEALTH,
            max_health: BASE_STARTING_HEALTH,
            current_health_percentage: 100.0,
            playtime_health_earned: 0.0,
            starting_health: BASE_STARTING_HEALTH,
            health_drain: 0.0,
            tick: 0,
            is_dead: false,
            reincarnation: 0,
            waiting: false,
        }
    }

    pub fn add_health(&mut self, health: f64) {
        self.current_health += health;
        self.max_health += health;
    }

    pub fn calculate_starting_health(&mut self, old_status: &Status) {
        self.playtime_health_earned = old_status.playtime_health_earned;
        self.playtime_health_earned += old_status.max_health - old_status.starting_health;
        self.starting_health = BASE_STARTING_HEALTH + self.playtime_health_earned / 20.0;
        self.max_health = self.starting_health;
        self.current_health = self.starting_health;
    }
}

pub fn base_values(game: &mut Game) {
    let inter = &mut game.intermediate_state;
    inter.add_base(KeyValues::InventorySize, 10.0, "Base Value");
    // inter.add_base(KeyValues::HealthDrain, 0.01, "Base Health Drain");
}
