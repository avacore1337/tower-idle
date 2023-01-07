use super::skill_history::{get_skill_history, SkillHistory};
use super::State;
use crate::types::*;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Tsify, Serialize, Deserialize, Clone, Debug)]
pub struct RoundHistory {
    pub skills: [SkillHistory; SKILL_SIZE],
    pub mana_gained: f64,
}

impl From<&State> for RoundHistory {
    fn from(state: &State) -> Self {
        RoundHistory {
            skills: get_skill_history(state),
            mana_gained: state.status.playtime_health_earned,
        }
    }
}
