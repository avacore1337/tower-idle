#![allow(non_snake_case)]

pub mod area;
pub mod boost;
pub mod collection;
pub mod crafting;
pub mod exploration;
pub mod floor;
pub mod item;
pub mod skill;

use serbia::serbia;
use serde::Serialize;
use std::sync::Mutex;

use self::area::WArea;
use crate::action_mapping::ActionMapping;
use crate::icon::{get_icons, Icon};
use crate::types::*;
use boost::{get_boosts, WBoost};
use collection::WCollection;
use crafting::WCrafting;
use exploration::WExploration;
use floor::{get_floors, WFloor};
use item::{get_items, WItem};
use skill::{get_skills, WSkill};
use std::collections::BTreeMap;

#[serbia]
#[derive(Serialize)]
pub struct World {
    // // #[serde(serialize_with = "<[_]>::serialize")]
    // pub boost_items: [BoostItem; BOOST_ITEM_SIZE],
    pub skills: [WSkill; SKILL_SIZE],
    pub floors: [WFloor; FLOOR_SIZE],
    pub items: [WItem; ITEM_SIZE],
    pub boosts: [WBoost; BOOST_SIZE],
    #[serde(skip_serializing)]
    pub action_mapping: Mutex<ActionMapping>,
    pub icons: BTreeMap<String, Icon>,
}

impl World {
    pub fn get_wcollection(&self, collection: AllCollects) -> &WCollection {
        let wfloor = self.get_wfloor(collection.into());
        let index = collection.to_collection_index();
        &wfloor.collections[index]
    }

    pub fn get_wexploration(&self, exploration: AllExplors) -> &WExploration {
        let wfloor = self.get_wfloor(exploration.into());
        let index = exploration.to_exploration_index();
        &wfloor.explorations[index]
    }

    pub fn get_wareas(&self, areas: AllAreas) -> &WArea {
        let wfloor = self.get_wfloor(areas.into());
        let index = areas.to_area_index();
        &wfloor.areas[index]
    }

    pub fn get_wcrafting(&self, crafting: AllCrafts) -> &WCrafting {
        let wfloor = self.get_wfloor(crafting.into());
        let index = crafting.to_crafting_index();
        &wfloor.craftings[index]
    }

    pub fn get_wfloor(&self, floor_type: FloorTypes) -> &WFloor {
        &self.floors[floor_type as usize]
    }

    pub fn get_witem(&self, item_type: ItemTypes) -> &WItem {
        &self.items[item_type as usize]
    }
}

impl Default for World {
    fn default() -> World {
        World {
            skills: get_skills(),
            floors: get_floors(),
            items: get_items(),
            boosts: get_boosts(),
            action_mapping: Mutex::new(ActionMapping::default()),
            icons: get_icons(),
        }
    }
}
