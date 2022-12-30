use super::*;
use crate::world::collection::{can_collect, Collect};
use crate::world::crafting::{can_craft, crafting_allowed};
use crate::{action_queue::ActionEntry, game::Game};
use serde::{Deserialize, Serialize};
use std::mem::variant_count;
use strum::{EnumIter, IntoEnumIterator};
use tsify::Tsify;

// use strum::{EnumIter, IntoStaticStr};
// use num_derive::FromPrimitive;
// use num_traits::FromPrimitive;

#[derive(
    Tsify, Serialize, Deserialize, EnumIter, Default, Clone, Copy, Debug, PartialEq, PartialOrd,
)]

pub enum F1Crafts {
    #[default]
    Axe,
    DoorHandle,
    RepairAlchemy,
    Altar,
    CrushCrystal,
}

#[derive(
    Tsify, Serialize, Deserialize, EnumIter, Default, Clone, Copy, Debug, PartialEq, PartialOrd,
)]
pub enum F2Crafts {
    #[default]
    Spear,
    PoisonTippedSpear,
    BetterAxe,
    Bridge,
}

#[derive(
    Tsify, Serialize, Deserialize, EnumIter, Default, Clone, Copy, Debug, PartialEq, PartialOrd,
)]
pub enum F3Crafts {
    #[default]
    Test,
}

#[derive(Tsify, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum AllCrafts {
    First(F1Crafts),
    Second(F2Crafts),
    Third(F3Crafts),
}

impl AllCrafts {
    pub fn to_crafting_index(&self) -> usize {
        match self {
            AllCrafts::First(x) => *x as usize,
            AllCrafts::Second(x) => *x as usize,
            AllCrafts::Third(x) => *x as usize,
        }
    }

    pub fn iter() -> impl Iterator<Item = AllCrafts> {
        // AllCraftsIterator { current: None }
        F1Crafts::iter()
            .map(|x| AllCrafts::First(x))
            .chain(F2Crafts::iter().map(|x| AllCrafts::Second(x)))
            .chain(F3Crafts::iter().map(|x| AllCrafts::Third(x)))
    }
}

pub const FIRST_CRAFTINGS_SIZE: usize = variant_count::<F1Crafts>();
pub const SECOND_CRAFTINGS_SIZE: usize = variant_count::<F2Crafts>();
pub const THIRD_CRAFTINGS_SIZE: usize = variant_count::<F3Crafts>();
pub const ALL_CRAFTINGS_SIZE: usize = variant_count::<AllCrafts>();

impl Prio for AllCrafts {
    fn get_user_priority(self, game: &Game) -> u32 {
        game.state.get_crafting(self).priority
    }
    fn get_automatable(self, game: &Game) -> bool {
        game.state.get_crafting(self).is_automatable
    }
    fn get_base_priority(self, _game: &Game) -> BasePriority {
        BasePriority::Mana
    }
    fn get_doable(self, game: &Game) -> bool {
        // make can craft or can get resources and craft
        if !crafting_allowed(self, game) {
            return false;
        }
        if can_craft(self, game) {
            return true;
        }
        let wcrafting = game.world.get_wcrafting(self);
        let mut missing_items: Vec<ItemTypes> = wcrafting
            .materials
            .iter()
            .filter(|material| game.state.get_item(material.item).amount < material.amount)
            .map(|material| material.item)
            .collect();
        let wfloor = &game.world.floors[game.state.current_floor as usize];
        for wcollection in wfloor.collections.iter() {
            if let Collect::Item(item) = &wcollection.collect {
                if missing_items.contains(&item) && can_collect(wcollection.name, game) {
                    missing_items.retain(|&x| x != *item)
                }
            }
        }
        missing_items.is_empty()
    }
    fn get_action_entry(self, game: &Game) -> ActionEntry {
        game.world.get_wcrafting(self).to_action_entry(1)
    }
}

impl Recordable for AllCrafts {
    fn to_record_key(&self) -> String {
        match self {
            AllCrafts::First(x) => x.to_record_key(),
            AllCrafts::Second(x) => x.to_record_key(),
            AllCrafts::Third(x) => x.to_record_key(),
        }
    }
}

impl Recordable for F1Crafts {
    fn to_record_key(&self) -> String {
        format!("F1-B {:#?}", self)
    }
}

impl Recordable for F2Crafts {
    fn to_record_key(&self) -> String {
        format!("F2-B {:#?}", self)
    }
}

impl Recordable for F3Crafts {
    fn to_record_key(&self) -> String {
        format!("F3-B {:#?}", self)
    }
}

impl From<F1Crafts> for AllCrafts {
    fn from(e: F1Crafts) -> Self {
        AllCrafts::First(e)
    }
}

impl From<F2Crafts> for AllCrafts {
    fn from(e: F2Crafts) -> Self {
        AllCrafts::Second(e)
    }
}

impl From<F3Crafts> for AllCrafts {
    fn from(e: F3Crafts) -> Self {
        AllCrafts::Third(e)
    }
}
