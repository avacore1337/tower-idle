use super::collection::{get_collections, Collection};
use super::crafting::{get_craftings, Crafting};
use super::exploration::{get_explorations, Exploration};
use crate::types::*;
use serde::{Deserialize, Serialize};
use std::mem::{self, MaybeUninit};
use strum::IntoEnumIterator;
use tsify::Tsify;

#[derive(Tsify, Serialize, Deserialize, Clone, Debug)]
pub struct Floor {
    pub name: FloorTypes,
    pub is_visible: bool,
    pub has_seen: bool,
    pub explorations: Vec<Exploration>,
    pub collections: Vec<Collection>,
    pub craftings: Vec<Crafting>,
}

impl Floor {
    pub fn new(floor: FloorTypes) -> Floor {
        Floor {
            name: floor,
            is_visible: false,
            has_seen: false,
            explorations: get_explorations(floor),
            collections: get_collections(floor),
            craftings: get_craftings(floor),
        }
    }
}

pub fn get_floors() -> [Floor; FLOOR_SIZE] {
    let mut floors: [MaybeUninit<Floor>; FLOOR_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    for name in FloorTypes::iter() {
        floors[name as usize].write(Floor::new(name));
    }
    unsafe { mem::transmute(floors) }
}
