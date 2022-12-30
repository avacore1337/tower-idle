use crate::game::Game;
use crate::types::*;
use serde::Serialize;
use std::mem::{self, MaybeUninit};
use strum::IntoEnumIterator;
use tsify::Tsify;

#[derive(Tsify, Serialize, Clone)]
pub struct WItem {
    pub name: ItemTypes,
    pub display_name: &'static str,
    pub description: &'static str,
    // pub effect_description: &'static str,
    pub can_use: bool,
    // pub xp_req_modifier: f64,
    // pub icon: Icon,
}

pub fn translate_item(item: ItemTypes) -> WItem {
    match item {
        ItemTypes::MetalScrap => WItem {
            name: item,
            display_name: "Metal piece",
            description: "Mostly some rusty nails",
            can_use: false,
        },
        ItemTypes::Stone => WItem {
            name: item,
            display_name: "Stone",
            description: "A piece of stone",
            can_use: false,
        },
        ItemTypes::Wood => WItem {
            name: item,
            display_name: "Wood",
            description: "Wood, useful for making things",
            can_use: false,
        },
        ItemTypes::Crystal => WItem {
            name: item,
            display_name: "Crystal",
            description: "A Magical crystal",
            can_use: false,
        },
        ItemTypes::Flint => WItem {
            name: item,
            display_name: "Flint",
            description: "A sharp piece of flint",
            can_use: false,
        },
        ItemTypes::Stick => WItem {
            name: item,
            display_name: "Stick",
            description: "A stick, with this you can't break any bones",
            can_use: false,
        },
        ItemTypes::Poison => WItem {
            name: item,
            display_name: "Poison",
            description: "A deadly poison",
            can_use: false,
        },
        ItemTypes::Log => WItem {
            name: item,
            display_name: "log",
            description: "A large log",
            can_use: false,
        },
        ItemTypes::Fur => WItem {
            name: item,
            display_name: "Fur",
            description: "Valuable goods",
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
