#![allow(non_snake_case)]

pub mod boost;
pub mod collection;
pub mod crafting;
pub mod exploration;
pub mod floor;
pub mod history;
pub mod item;
pub mod round_history;
pub mod skill;
pub mod skill_history;
pub mod status;
use crate::types::*;
use boost::{get_boosts, Boost};
use collection::Collection;
use crafting::Crafting;
use exploration::Exploration;
use floor::{get_floors, Floor};
use item::{get_items, Item};
use serbia::serbia;
use serde::{Deserialize, Serialize};
use skill::{get_skills, Skill};
use status::Status;
use strum::IntoEnumIterator;
// use strum::IntoEnumIterator;

#[serbia]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct State {
    pub items: [Item; ITEM_SIZE],
    pub skills: [Skill; SKILL_SIZE],
    pub floors: [Floor; FLOOR_SIZE],
    pub current_floor: FloorTypes,
    pub current_skill: SkillTypes,
    pub current_area: AllAreas,
    pub boosts: [Boost; BOOST_SIZE],
    pub status: Status,
    pub messages: Vec<String>,
}

impl Default for State {
    fn default() -> State {
        let mut state = State {
            skills: get_skills(),
            floors: get_floors(),
            items: get_items(),
            current_floor: FloorTypes::Starting,
            current_skill: SkillTypes::Agility,
            current_area: AllAreas::default(),
            boosts: get_boosts(),
            status: Status::new(),
            messages: vec![],
        };
        state.set_current_floor(FloorTypes::Starting);
        state
    }
}

impl State {
    pub fn set_current_floor(&mut self, floor_type: FloorTypes) {
        self.current_floor = floor_type;
        self.floors[floor_type as usize].is_visible = true;
        self.floors[floor_type as usize].has_seen = true;
        match floor_type {
            FloorTypes::Starting => {}
            FloorTypes::Second => {
                self.get_mut_item(ItemTypes::Stone).amount = 0;
                self.get_mut_item(ItemTypes::Wood).amount = 0;
            }
            FloorTypes::Third => {}
        }
        for name in ItemTypes::iter() {
            let item = self.get_mut_item(name);
            if item.amount == 0 {
                item.is_visible = false;
            }
        }
    }

    pub fn get_collection(&self, collection: AllCollects) -> &Collection {
        let floor = self.get_floor(collection.into());
        let index = collection.to_collection_index();
        &floor.collections[index]
    }

    pub fn get_exploration(&self, exploration: AllExplors) -> &Exploration {
        let floor = self.get_floor(exploration.into());
        let index = exploration.to_exploration_index();
        &floor.explorations[index]
    }

    pub fn get_crafting(&self, crafting: AllCrafts) -> &Crafting {
        let floor = self.get_floor(crafting.into());
        let index = crafting.to_crafting_index();
        &floor.craftings[index]
    }

    pub fn get_boost(&self, boost: BoostTypes) -> &Boost {
        &self.boosts[boost as usize]
    }

    pub fn get_item(&self, item_type: ItemTypes) -> &Item {
        &self.items[item_type as usize]
    }

    pub fn get_floor(&self, floor_type: FloorTypes) -> &Floor {
        &self.floors[floor_type as usize]
    }

    pub fn get_mut_collection(&mut self, collection: AllCollects) -> &mut Collection {
        let floor = self.get_mut_floor(collection.into());
        let index = collection.to_collection_index();
        &mut floor.collections[index]
    }

    pub fn get_mut_exploration(&mut self, exploration: AllExplors) -> &mut Exploration {
        let floor = self.get_mut_floor(exploration.into());
        let index = exploration.to_exploration_index();
        &mut floor.explorations[index]
    }

    pub fn get_mut_crafting(&mut self, crafting: AllCrafts) -> &mut Crafting {
        let floor = self.get_mut_floor(crafting.into());
        let index = crafting.to_crafting_index();
        &mut floor.craftings[index]
    }

    pub fn get_mut_boost(&mut self, boost: BoostTypes) -> &mut Boost {
        &mut self.boosts[boost as usize]
    }

    pub fn get_mut_item(&mut self, item_type: ItemTypes) -> &mut Item {
        &mut self.items[item_type as usize]
    }

    pub fn get_mut_floor(&mut self, floor_type: FloorTypes) -> &mut Floor {
        &mut self.floors[floor_type as usize]
    }
}

pub fn rebirth(state: &State) -> State {
    let mut new_state = State::default();

    new_state.status.reincarnation = state.status.reincarnation;
    new_state.status.calculate_starting_health(&state.status);

    for (i, skill) in state.skills.iter().enumerate() {
        new_state.skills[i].talent = skill.talent;
        new_state.skills[i].starting_talent = skill.talent;
        new_state.skills[i].talent_current_xp = skill.talent_current_xp;
    }
    for (i, floor) in state.floors.iter().enumerate() {
        let new_floor = &mut new_state.floors[i];
        for (j, collection) in floor.collections.iter().enumerate() {
            let new_collection = &mut new_floor.collections[j];
            new_collection.completion_count = collection.completion_count;
            new_collection.has_seen = collection.has_seen;
            new_collection.is_automatable = collection.is_automatable;
            new_collection.priority = collection.priority;
        }
        for (j, crafting) in floor.craftings.iter().enumerate() {
            let new_crafting = &mut new_floor.craftings[j];
            new_crafting.completion_count = crafting.completion_count;
            new_crafting.has_seen = crafting.has_seen;
            new_crafting.is_automatable = crafting.is_automatable;
            new_crafting.priority = crafting.priority;
        }
        for (j, exploration) in floor.explorations.iter().enumerate() {
            let new_explorations = &mut new_floor.explorations[j];
            new_explorations.completion_count = exploration.completion_count;
            new_explorations.has_seen = exploration.has_seen;
            new_explorations.is_automatable = exploration.is_automatable;
            new_explorations.priority = exploration.priority;
        }
    }
    new_state
}
