use crate::{action_queue::ActionEntry, game::Game};
pub use area::{AllAreas, F1Areas, F2Areas, F3Areas};
pub use collection::{
    AllCollects, F1Collects, F2Collects, F3Collects, ALL_COLLECTIONS_SIZE, FIRST_COLLECTIONS_SIZE,
    SECOND_COLLECTIONS_SIZE, THIRD_COLLECTIONS_SIZE,
};
pub use crafting::{
    AllCrafts, F1Crafts, F2Crafts, F3Crafts, ALL_CRAFTINGS_SIZE, FIRST_CRAFTINGS_SIZE,
    SECOND_CRAFTINGS_SIZE, THIRD_CRAFTINGS_SIZE,
};
pub use exploration::{
    AllExplors, F1Explors, F2Explors, F3Explors, ALL_EXPLORATIONS_SIZE, FIRST_EXPLORATIONS_SIZE,
    SECOND_EXPLORATIONS_SIZE, THIRD_EXPLORATIONS_SIZE,
};
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, mem::variant_count};
use strum::EnumIter;
use wasm_bindgen::prelude::*;

pub mod area;
pub mod collection;
pub mod crafting;
pub mod exploration;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum SkillTypes {
    Woodcutting,
    Crafting,
    Agility,
    Alchemy,
    Mining,
    Fighting,
    Farming,
    Fishing,
    Cooking,
    Hunting,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum BoostTypes {
    Axe,
    BetterAxe,
    Altar,
    Spear,
    // PoisonTippedSpear,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum ItemTypes {
    MetalScrap,
    Stone,
    Wood,
    Crystal,
    Flint,
    Stick,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum FloorTypes {
    Starting,
    Second,
    Third,
}

impl From<AllCrafts> for FloorTypes {
    fn from(crafting: AllCrafts) -> Self {
        match crafting {
            AllCrafts::First(_) => FloorTypes::Starting,
            AllCrafts::Second(_) => FloorTypes::Second,
            AllCrafts::Third(_) => FloorTypes::Third,
        }
    }
}

impl From<AllExplors> for FloorTypes {
    fn from(exploration: AllExplors) -> Self {
        match exploration {
            AllExplors::First(_) => FloorTypes::Starting,
            AllExplors::Second(_) => FloorTypes::Second,
            AllExplors::Third(_) => FloorTypes::Third,
        }
    }
}

impl From<AllAreas> for FloorTypes {
    fn from(area: AllAreas) -> Self {
        match area {
            AllAreas::First(_) => FloorTypes::Starting,
            AllAreas::Second(_) => FloorTypes::Second,
            AllAreas::Third(_) => FloorTypes::Third,
        }
    }
}

impl From<AllCollects> for FloorTypes {
    fn from(exploration: AllCollects) -> Self {
        match exploration {
            AllCollects::First(_) => FloorTypes::Starting,
            AllCollects::Second(_) => FloorTypes::Second,
            AllCollects::Third(_) => FloorTypes::Third,
        }
    }
}

pub const SKILL_SIZE: usize = variant_count::<SkillTypes>();
pub const FLOOR_SIZE: usize = variant_count::<FloorTypes>();
pub const ITEM_SIZE: usize = variant_count::<ItemTypes>();
pub const BOOST_SIZE: usize = variant_count::<BoostTypes>();

impl From<AllCollects> for ItemTypes {
    fn from(collection: AllCollects) -> Self {
        match collection {
            AllCollects::First(x) => x.into(),
            AllCollects::Second(x) => x.into(),
            AllCollects::Third(x) => x.into(),
        }
    }
}

impl From<F1Collects> for ItemTypes {
    fn from(collection: F1Collects) -> Self {
        match collection {
            F1Collects::MetalScrap => ItemTypes::MetalScrap,
            F1Collects::Rock => ItemTypes::Stone,
            F1Collects::Wood => ItemTypes::Wood,
            F1Collects::Crystal => ItemTypes::Crystal,
            _ => panic!(),
        }
    }
}

impl From<F2Collects> for ItemTypes {
    fn from(collection: F2Collects) -> Self {
        match collection {
            F2Collects::Stick => ItemTypes::Stick,
            F2Collects::Flint => ItemTypes::Flint,
            _ => panic!(),
        }
    }
}

impl From<F3Collects> for ItemTypes {
    fn from(collection: F3Collects) -> Self {
        match collection {
            // F3Collects::Test => ItemTypes::Stick,
            _ => panic!(),
        }
    }
}

pub trait Recordable {
    fn to_record_key(&self) -> String;
}

pub trait Prio {
    fn get_action_entry(self, game: &Game) -> ActionEntry;
    fn get_user_priority(self, game: &Game) -> u32;
    fn get_base_priority(self, game: &Game) -> BasePriority;
    fn get_doable(self, game: &Game) -> bool;
    fn get_automatable(self, game: &Game) -> bool;
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum BasePriority {
    Material,
    Exploration,
    Crafting,
    Mana,
}

#[derive(Clone, Debug)]
pub struct Priority {
    pub is_doable: bool,
    pub is_automatable: bool,
    pub user_priority: u32,
    pub base_priority: BasePriority,
    pub action_entry: ActionEntry,
}

impl PartialEq for Priority {
    fn eq(&self, other: &Self) -> bool {
        self.is_doable == other.is_doable
            && self.is_automatable == other.is_automatable
            && self.user_priority == other.user_priority
            && self.base_priority == other.base_priority
    }
}

impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.is_doable
                .partial_cmp(&other.is_doable)?
                .then(self.is_automatable.partial_cmp(&other.is_automatable)?)
                .then(self.user_priority.partial_cmp(&other.user_priority)?)
                .then(self.base_priority.partial_cmp(&other.base_priority)?),
        )
    }
}

impl Priority {
    pub fn new<T: Prio + Copy>(actionable: T, game: &Game) -> Self {
        Priority {
            is_doable: actionable.get_doable(game),
            is_automatable: actionable.get_automatable(game),
            action_entry: actionable.get_action_entry(game),
            user_priority: actionable.get_user_priority(game),
            base_priority: actionable.get_base_priority(game),
        }
    }
}
