// use crate::engine::value_keys::KeyValues;
use crate::game::Game;
use crate::icon::{Icon, IconType};
use crate::types::*;
use serde::Serialize;
use std::mem::{self, MaybeUninit};
use strum::IntoEnumIterator;
use tsify::Tsify;

#[derive(Tsify, Serialize, Clone)]
pub struct WSkill {
    pub name: SkillTypes,
    pub description: &'static str,
    // pub effect_description: &'static str,
    // pub display_name: &'static str,
    pub xp_req_modifier: f64,
    pub icon: Icon,
}

impl WSkill {
    pub fn get_skills_gains(&self, game: &mut Game) {
        let skill_state = &mut game.state.skills[self.name as usize];

        game.intermediate_state.add_multiplier(
            self.name.into(),
            1.05_f64.powf(skill_state.level),
            "level",
        );
        game.intermediate_state.add_multiplier(
            self.name.into(),
            1.05_f64.powf(skill_state.talent),
            "talent",
        );
    }
}

pub fn translate_skill(skill: SkillTypes) -> WSkill {
    match skill {
        SkillTypes::Farming => WSkill {
            name: skill,
            description: "Farming. Inside a Tower?",
            icon: IconType::Farming.into(),
            xp_req_modifier: 1.0,
        },
        SkillTypes::Woodcutting => WSkill {
            name: skill,
            description: "You're a lumberjack and you're ok.",
            icon: IconType::Woodcutting.into(),
            xp_req_modifier: 1.0,
        },
        SkillTypes::Mining => WSkill {
            name: skill,
            description: "The hit rock with thing skill",
            icon: IconType::Mining.into(),
            xp_req_modifier: 1.0,
        },
        SkillTypes::Alchemy => WSkill {
            name: skill,
            description: "It's magic, with a healthy dose of science mixed in. sort of.",
            icon: IconType::Alchemy.into(),
            xp_req_modifier: 1.0,
        },
        SkillTypes::Crafting => WSkill {
            name: skill,
            description: "Trinkets, tables and axes. You can craft anything",
            icon: IconType::Crafting.into(),
            xp_req_modifier: 1.0,
        },
        SkillTypes::Agility => WSkill {
            name: skill,
            description: "Your ability to sneak around a tower is vital to your survival",
            icon: IconType::Agility.into(),
            xp_req_modifier: 1.0,
        },
        SkillTypes::Fishing => WSkill {
            name: skill,
            description: "TODO",
            icon: IconType::Fishing.into(),
            xp_req_modifier: 1.0,
        },
        SkillTypes::Cooking => WSkill {
            name: skill,
            description: "TODO",
            icon: IconType::Cooking.into(),
            xp_req_modifier: 1.0,
        },
        SkillTypes::Hunting => WSkill {
            name: skill,
            description: "Hunting wild animals",
            icon: IconType::Hunting.into(),
            xp_req_modifier: 1.0,
        },
        SkillTypes::Fighting => WSkill {
            name: skill,
            description: "Your ability to hit stuff",
            icon: IconType::Fighting.into(),
            xp_req_modifier: 1.0,
        },
    }
}

pub fn should_be_visible_skill(skill_type: SkillTypes, game: &Game) -> bool {
    let skill = &game.state.skills[skill_type as usize];
    match skill_type {
        SkillTypes::Agility => true,
        SkillTypes::Woodcutting => true,
        SkillTypes::Mining => true,
        SkillTypes::Crafting => true,
        _ => skill.level > 1.0 || skill.level_current_xp > 0.0,
    }
}

pub fn get_skills() -> [WSkill; SKILL_SIZE] {
    let mut skills: [MaybeUninit<WSkill>; SKILL_SIZE] =
        unsafe { MaybeUninit::uninit().assume_init() };
    for name in SkillTypes::iter() {
        skills[name as usize].write(translate_skill(name));
    }
    unsafe { mem::transmute(skills) }
}
