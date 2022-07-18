use super::{round_history::RoundHistory, State};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use tsify::Tsify;
// use crate::types::*;

const MAX_HISTORY: usize = 5;

#[derive(Tsify, Serialize, Deserialize, Clone, Debug)]
pub struct History {
    pub skill_history: VecDeque<RoundHistory>,
    pub current_round: RoundHistory,
    pub total_ticks: u32,
}

impl History {
    pub fn new(state: &State) -> History {
        History {
            skill_history: VecDeque::new(),
            current_round: state.into(),
            total_ticks: 0,
        }
    }

    pub fn pre_death_clear(&mut self) {}

    pub fn rebirth_update(&mut self, state: &State) {
        self.update_history(state);
        self.add_history(state.into());
        self.total_ticks += state.status.tick;
    }

    pub fn add_history(&mut self, round: RoundHistory) {
        self.skill_history.push_front(round);
        if self.skill_history.len() > MAX_HISTORY {
            self.skill_history.pop_back();
        }
    }

    pub fn update_history(&mut self, state: &State) {
        self.current_round = state.into();
    }
}
