use crate::types::*;
use serde::{Deserialize, Serialize};
use std::mem::{self, MaybeUninit};
use strum::IntoEnumIterator;
use tsify::Tsify;

use super::{skill::Skill, State};

#[derive(Tsify, Serialize, Deserialize, Clone, Debug)]
pub struct SkillHistory {
    pub name: SkillTypes,
    pub talent: f64,
    pub starting_talent: f64,
    pub talent_delta: f64,
    pub is_visible: bool,
}

impl SkillHistory {
    pub fn new(skill: SkillTypes) -> SkillHistory {
        SkillHistory {
            name: skill,
            talent: 0.0,
            starting_talent: 0.0,
            talent_delta: 0.0,
            is_visible: false,
        }
    }
}

impl From<&Skill> for SkillHistory {
    fn from(skill: &Skill) -> Self {
        SkillHistory {
            name: skill.name,
            talent: skill.talent,
            starting_talent: skill.starting_talent,
            talent_delta: skill.talent - skill.starting_talent,
            is_visible: skill.is_visible,
        }
    }
}

pub fn get_skill_history(state: &State) -> [SkillHistory; SKILL_SIZE] {
    let mut skills: [MaybeUninit<SkillHistory>; SKILL_SIZE] =
        unsafe { MaybeUninit::uninit().assume_init() };
    for skill in SkillTypes::iter() {
        skills[skill as usize].write((&state.skills[skill as usize]).into());
    }
    unsafe { mem::transmute(skills) }
}
