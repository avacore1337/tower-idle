#![allow(dead_code)]

use crate::action_queue::ActionEntry;
use crate::engine::actions::*;
use crate::game::Game;
use crate::types::*;
use std::collections::HashMap;
// use strum::IntoEnumIterator;

pub enum ActionResult {
    Worked,
    Completed(u32),
    Invalid,
    Waiting,
    NewAction(ActionEntry),
}

type Callback = Box<dyn Fn(&mut Game) -> ActionResult + Send>;

pub struct ActionMapping {
    pub action_functions: HashMap<String, Callback>,
}

impl ActionMapping {
    fn add<T: Recordable>(&mut self, key: T, callback: Callback) {
        self.action_functions.insert(key.to_record_key(), callback);
    }
}

impl Default for ActionMapping {
    fn default() -> ActionMapping {
        let mut mapping = ActionMapping {
            action_functions: HashMap::new(),
        };

        for exploration in AllExplors::iter() {
            mapping.add(
                exploration,
                Box::new(move |game: &mut Game| explore_action(game, exploration)),
            );
        }
        for collection in AllCollects::iter() {
            mapping.add(
                collection,
                Box::new(move |game: &mut Game| collect_action(game, collection)),
            );
        }
        for crafting in AllCrafts::iter() {
            mapping.add(
                crafting,
                Box::new(move |game: &mut Game| crafting_action(game, crafting)),
            );
        }
        mapping
    }
}
