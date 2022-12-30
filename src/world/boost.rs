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
    // pub effect_description: &'static str,
    pub display_name: &'static str,
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

        let inter = &mut game.intermediate_state;
        match self.name {
            BoostTypes::Axe => {
                inter.add_multiplier(KeyValues::Woodcutting, 1.5, self.display_name);
            }
            BoostTypes::Altar => {
                inter.add_multiplier(KeyValues::HealthDrain, 0.5, self.display_name);
            }
            BoostTypes::Spear => {
                inter.add_multiplier(KeyValues::Fighting, 1.5, self.display_name);
            }
            BoostTypes::PoisonTippedSpear => {
                inter.add_multiplier(KeyValues::Fighting, 1.5, self.display_name);
            }
            BoostTypes::BetterAxe => {
                inter.add_multiplier(KeyValues::Woodcutting, 1.5, self.display_name);
            }
        }
    }
}
pub fn translate_boost(boost: BoostTypes) -> WBoost {
    match boost {
        BoostTypes::Axe => WBoost {
            name: boost,
            description: "Boosts Woodcutting",
            display_name: "Simple Axe",
        },
        BoostTypes::Altar => WBoost {
            name: boost,
            description: "An Altar",
            display_name: "Halves Mana loss",
        },
        BoostTypes::Spear => WBoost {
            name: boost,
            description: "Spear",
            display_name: "Boosts Fighting",
        },
        BoostTypes::PoisonTippedSpear => WBoost {
            name: boost,
            description: "Poison Tipped Spear",
            display_name: "Boosts Fighting",
        },
        BoostTypes::BetterAxe => WBoost {
            name: boost,
            description: "Boosts Woodcutting even more",
            display_name: "A sharp flint Axe",
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
