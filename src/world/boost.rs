use crate::engine::value_keys::KeyValues;
use crate::game::Game;
use crate::types::*;
use serde::Serialize;
use std::mem::{self, MaybeUninit};
use strum::IntoEnumIterator;
use tsify::Tsify;

#[derive(Tsify, Serialize, Clone)]
pub struct WBoost {
    pub name: BoostTypes,
    pub description: &'static str,
    pub display_name: &'static str,
    pub key: KeyValues,
    pub value: f64,
    // pub effect_description: &'static str,
    // pub required_tier: u32,
    // pub xp_req_modifier: f64,
    // pub icon: Icon,
}

impl WBoost {
    pub fn get_boost_gains(&self, game: &mut Game) {
        let boost = &mut game.state.boosts[self.name as usize];
        if !boost.unlocked {
            return;
        }
        game.intermediate_state
            .add_multiplier(self.key, self.value, self.display_name);
    }
}

pub fn translate_boost(boost: BoostTypes) -> WBoost {
    match boost {
        BoostTypes::Axe => WBoost {
            name: boost,
            description: "Boosts Woodcutting",
            display_name: "Simple Axe",
            key: KeyValues::Woodcutting,
            value: 1.5,
        },
        BoostTypes::Altar => WBoost {
            name: boost,
            description: "An Altar",
            display_name: "Halves Mana loss",
            key: KeyValues::HealthDrain,
            value: 0.5,
        },
        BoostTypes::Spear => WBoost {
            name: boost,
            description: "Spear",
            display_name: "Boosts Fighting",
            key: KeyValues::Fighting,
            value: 1.5,
        },
        BoostTypes::PoisonTippedSpear => WBoost {
            name: boost,
            description: "Poison Tipped Spear",
            display_name: "Boosts Fighting",
            key: KeyValues::Fighting,
            value: 1.5,
        },
        BoostTypes::BetterAxe => WBoost {
            name: boost,
            description: "Boosts Woodcutting even more",
            display_name: "A sharp flint Axe",
            key: KeyValues::Woodcutting,
            value: 1.5,
        },
    }
}

pub fn get_boosts() -> [WBoost; BOOST_SIZE] {
    let mut boosts: [MaybeUninit<WBoost>; BOOST_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    for name in BoostTypes::iter() {
        boosts[name as usize].write(translate_boost(name));
    }
    unsafe { mem::transmute(boosts) }
}
