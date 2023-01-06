#![allow(dead_code)]
#![cfg(debug_assertions)]
#![allow(non_upper_case_globals)]

use crate::engine::actionless_update;
use crate::engine::death_update;
use crate::engine::engine_run;
use crate::game::Game;
use crate::presets::get_presets;
use crate::types::*;
use crate::util::tune_floor_priority;
use crate::world::collection::should_be_automatable_collection;
use crate::world::crafting::should_be_automatable_crafting;
use crate::world::exploration::should_be_automatable_exploration;
use crate::GLOBAL_DATA;
use log::info;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn toggle_override_automatable() {
    let game: &mut Game = &mut *GLOBAL_DATA.lock().unwrap();
    game.state.status.override_automatable = !game.state.status.override_automatable;
    for collection in AllCollects::iter() {
        let automate = should_be_automatable_collection(collection, game);
        let mut collection = game.state.get_mut_collection(collection);
        collection.is_automatable = automate;
    }
    for crafting in AllCrafts::iter() {
        let automate = should_be_automatable_crafting(crafting, game);
        let mut crafting = game.state.get_mut_crafting(crafting);
        crafting.is_automatable = automate;
    }
    for exploration in AllExplors::iter() {
        let automate = should_be_automatable_exploration(exploration, game);
        let mut exploration = game.state.get_mut_exploration(exploration);
        exploration.is_automatable = automate;
    }
    for floor_index in 0..(FLOOR_SIZE) {
        tune_floor_priority(&mut game.state, floor_index);
    }
}

#[wasm_bindgen]
pub fn toggle_auto_rebirth() {
    let mut game = GLOBAL_DATA.lock().unwrap();
    game.state.status.auto_rebirth = !game.state.status.auto_rebirth;
}

#[wasm_bindgen]
pub fn debug_die() {
    let mut game = GLOBAL_DATA.lock().unwrap();
    game.state.status.current_health = 0.0;
    actionless_update(&mut *game);
}

#[wasm_bindgen]
pub fn print_debug_intermediate() {
    let game = GLOBAL_DATA.lock().unwrap();
    info!("intermediate: {:#?}", game.intermediate_state);
}

#[wasm_bindgen]
pub fn print_debug_state() {
    let game = GLOBAL_DATA.lock().unwrap();
    info!("state: {:#?}", game.state);
}

#[wasm_bindgen]
pub fn print_debug_history() {
    let game = GLOBAL_DATA.lock().unwrap();
    info!("history: {:#?}", game.history);
}

#[wasm_bindgen]
pub fn print_debug_meta() {
    let game = GLOBAL_DATA.lock().unwrap();
    info!("meta: {:#?}", game.meta_data);
}

#[wasm_bindgen]
pub fn print_debug_action_queue() {
    let game = GLOBAL_DATA.lock().unwrap();
    info!("action_queue: {:#?}", game.action_queue);
}

#[wasm_bindgen]
pub fn single_tick() {
    let mut game = GLOBAL_DATA.lock().unwrap();
    engine_run(&mut game);
}

#[wasm_bindgen]
pub fn set_gamespeed(speed: u32) {
    let mut game = GLOBAL_DATA.lock().unwrap();
    game.meta_data.game_speed = speed;
}

#[wasm_bindgen]
pub fn get_preset_saves() -> JsValue {
    log::info!("get preset");
    let list: Vec<&'static str> = get_presets().keys().into_iter().copied().collect();
    JsValue::from_serde(&list).unwrap()
}

#[wasm_bindgen]
pub fn set_preset_saves(preset_name: &str) {
    let game: &mut Game = &mut *GLOBAL_DATA.lock().unwrap();
    let mut presets = get_presets();
    if let Some(game_save) = presets.remove(preset_name) {
        game.load_game(game_save);
        death_update(game);
    }
}

#[wasm_bindgen]
pub fn test() {
    // let user_input_mapping = InputMapping::default();
    // info!("Mapping: {:#?}", user_input_mapping.user_function.keys());
    // let game = GLOBAL_DATA.lock().unwrap();
    // info!("input: {:#?}", game.inputs);
    // // let val = serde::ser
    // let res = serde_json::to_string(&game.inputs).unwrap();
    // info!("{}", res);
}
