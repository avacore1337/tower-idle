use crate::types::*;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

use super::{
    skill_history::{get_skill_history, SkillHistory},
    State,
};

#[derive(Tsify, Serialize, Deserialize, Clone, Debug)]
pub struct RoundHistory {
    pub skills: [SkillHistory; SKILL_SIZE],
}

impl From<&State> for RoundHistory {
    fn from(state: &State) -> Self {
        RoundHistory {
            skills: get_skill_history(state),
        }
    }
}
