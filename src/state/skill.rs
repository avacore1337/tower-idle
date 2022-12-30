use crate::types::*;
use serde::{Deserialize, Serialize};
use std::mem::{self, MaybeUninit};
use strum::IntoEnumIterator;
use tsify::Tsify;

#[derive(Tsify, Serialize, Deserialize, Clone, Debug)]
pub struct Skill {
    pub name: SkillTypes,
    pub level: f64,
    pub level_current_xp: f64,
    pub level_xp_required: f64,
    pub level_percent: f64,
    pub talent: f64,
    pub starting_talent: f64,
    pub talent_current_xp: f64,
    pub talent_xp_required: f64,
    pub talent_percent: f64,
    pub total_multiplier: f64,
    pub xp_rate: f64,
    pub is_visible: bool,
}

impl Skill {
    pub fn new(skill: SkillTypes) -> Skill {
        Skill {
            name: skill,
            level: 0.0,
            level_current_xp: 0.0,
            level_xp_required: 100.0,
            level_percent: 0.0,
            talent: 0.0,
            starting_talent: 0.0,
            talent_current_xp: 0.0,
            talent_xp_required: 100.0,
            talent_percent: 0.0,
            total_multiplier: 0.0,
            xp_rate: 0.0,
            is_visible: false,
        }
    }
}

pub fn get_skills() -> [Skill; SKILL_SIZE] {
    let mut skills: [MaybeUninit<Skill>; SKILL_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    for name in SkillTypes::iter() {
        skills[name as usize].write(Skill::new(name));
    }
    unsafe { mem::transmute(skills) }
}
