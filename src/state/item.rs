use crate::types::*;
use serde::{Deserialize, Serialize};
use std::mem::{self, MaybeUninit};
use strum::IntoEnumIterator;
use tsify::Tsify;

#[derive(Tsify, Serialize, Deserialize, Clone, Debug)]
pub struct Item {
    pub name: ItemTypes,
    pub amount: u32,
    pub max_amount: u32,
    pub is_visible: bool,
}

impl Item {
    pub fn new(item: ItemTypes) -> Item {
        Item {
            name: item,
            amount: 0,
            max_amount: 10,
            is_visible: false,
        }
    }

    pub fn is_maxed(&self) -> bool {
        self.amount >= self.max_amount
    }
}

pub fn get_items() -> [Item; ITEM_SIZE] {
    let mut items: [MaybeUninit<Item>; ITEM_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    for name in ItemTypes::iter() {
        items[name as usize].write(Item::new(name));
    }
    unsafe { mem::transmute(items) }
}
