use crate::game::Game;
use crate::types::*;
use serde::Serialize;
use std::mem::{self, MaybeUninit};
use strum::IntoEnumIterator;
use tsify::Tsify;

#[derive(Tsify, Serialize, Clone)]
pub struct WItem {
    pub name: ItemTypes,
    pub description: &'static str,
    // pub effect_description: &'static str,
    pub display_name: &'static str,
    pub can_use: bool,
    // pub xp_req_modifier: f64,
    // pub icon: Icon,
}

pub fn translate_item(item: ItemTypes) -> WItem {
    match item {
        ItemTypes::MetalScrap => WItem {
            name: item,
            description: "Mostly some rusty nails",
            display_name: "Metal piece",
            can_use: false,
        },
        ItemTypes::Stone => WItem {
            name: item,
            description: "A piece of stone",
            display_name: "Stone",
            can_use: false,
        },
        ItemTypes::Wood => WItem {
            name: item,
            description: "Wood, useful for making things",
            display_name: "Wood",
            can_use: false,
        },
        ItemTypes::Crystal => WItem {
            name: item,
            description: "A Magical crystal",
            display_name: "Crystal",
            can_use: false,
        },
        ItemTypes::Flint => WItem {
            name: item,
            description: "A sharp piece of flint",
            display_name: "Flint",
            can_use: false,
        },
        ItemTypes::Stick => WItem {
            name: item,
            description: "A stick, with this you can't break any bones",
            display_name: "Stick",
            can_use: false,
        },
    }
}

pub fn should_be_visible_item(item_type: ItemTypes, game: &Game) -> bool {
    let item = &game.state.items[item_type as usize];
    item.amount > 0
}

pub fn consume_item(item_type: ItemTypes, game: &mut Game) {
    let item = game.state.get_mut_item(item_type);
    let witem = game.world.get_witem(item_type);
    if witem.can_use && item.amount > 0 {
        item.amount -= 1;
        match item_type {
            _ => {}
        }
    }
}

pub fn get_items() -> [WItem; ITEM_SIZE] {
    let mut items: [MaybeUninit<WItem>; ITEM_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    for name in ItemTypes::iter() {
        items[name as usize].write(translate_item(name));
    }
    unsafe { mem::transmute(items) }
}
