use super::area::{get_areas, WArea};
use super::collection::{get_collections, WCollection};
use super::crafting::{get_craftings, WCrafting};
use super::exploration::{get_explorations, WExploration};
use crate::types::*;
use serde::Serialize;
use std::mem::{self, MaybeUninit};
use std::vec::Vec;
use strum::IntoEnumIterator;
use tsify::Tsify;

#[derive(Tsify, Serialize, Clone)]
pub struct WFloor {
    pub name: FloorTypes,
    pub floor_index: usize,
    pub explorations: Vec<WExploration>,
    pub areas: Vec<WArea>,
    pub collections: Vec<WCollection>,
    pub craftings: Vec<WCrafting>,
    // pub description: &'static str,
    // pub display_name: &'static str,
}

pub fn translate_floor(floor: FloorTypes) -> WFloor {
    WFloor {
        name: floor,
        floor_index: floor as usize,
        explorations: get_explorations(floor),
        areas: get_areas(floor),
        collections: get_collections(floor),
        craftings: get_craftings(floor),
        // description: "TODO",
    }
}

pub fn get_floors() -> [WFloor; FLOOR_SIZE] {
    let mut floors: [MaybeUninit<WFloor>; FLOOR_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    for name in FloorTypes::iter() {
        floors[name as usize].write(translate_floor(name));
    }
    unsafe { mem::transmute(floors) }
}
