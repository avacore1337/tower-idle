#![feature(variant_count)]
#![feature(drain_filter)]
#![allow(non_upper_case_globals)]
// #![feature(generic_const_exprs)]

use icon::{Icon, IconType};
use log::{info, Level};
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

// #[macro_use]
// extern crate num_derive;

#[macro_use]
extern crate lazy_static;

// // #[macro_use]
// // extern crate serde_big_array;

pub mod action_mapping;
pub mod action_queue;
pub mod engine;
pub mod game;
pub mod icon;
pub mod map;
pub mod meta;
pub mod presets;
pub mod state;
pub mod types;
pub mod util;
pub mod wasm_api;
pub mod world;

use crate::engine::{actionless_update, do_rebirth_internal};
use crate::types::*;
use crate::world::item::consume_item;
use engine::engine_run;
use game::Game;
use map::generate_map_data;
use meta::UserSettings;
use wasm_api::meta::do_save;
use world::World;

const TICK_RATE: f64 = 30.0;
const TICK_MS: f64 = 1000.0 / TICK_RATE;

lazy_static! {
    static ref WORLD: World = World::default();
    static ref GLOBAL_DATA: Mutex<Game> = Mutex::new(Game::new());
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_log::init_with_level(Level::Info).expect("error initializing log");
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    info!("Hello One Life!");

    Ok(())
}

#[wasm_bindgen]
pub fn get_new_texts() -> JsValue {
    // info!("Getting new texts");
    let game: &mut Game = &mut *GLOBAL_DATA.lock().unwrap();
    let ret = JsValue::from_serde(&game.state.messages).unwrap();
    game.state.messages.clear();
    ret
}

#[wasm_bindgen]
pub fn get_map_for_floor(val: &JsValue) -> JsValue {
    info!("Getting map data for: {:?} ", val);
    let game: &mut Game = &mut *GLOBAL_DATA.lock().unwrap();
    let f: FloorTypes = val.into_serde().unwrap();
    JsValue::from_serde(&generate_map_data(game, f)).unwrap()
}

#[wasm_bindgen]
pub fn toggle_priority_exploration(val: &JsValue) {
    info!("Rust toggle priority exploration");
    let game: &mut Game = &mut *GLOBAL_DATA.lock().unwrap();
    let t: AllExplors = val.into_serde().unwrap();
    let x = game.state.get_mut_exploration(t);
    x.priority = (x.priority + 1) % 5;
}

#[wasm_bindgen]
pub fn toggle_priority_collection(val: &JsValue) {
    info!("Rust toggle priority collection");
    let game: &mut Game = &mut *GLOBAL_DATA.lock().unwrap();
    let t: AllCollects = val.into_serde().unwrap();
    let x = game.state.get_mut_collection(t);
    x.priority = (x.priority + 1) % 5;
}

#[wasm_bindgen]
pub fn toggle_priority_crafting(val: &JsValue) {
    info!("Rust toggle priority crafting");
    let game: &mut Game = &mut *GLOBAL_DATA.lock().unwrap();
    let t: AllCrafts = val.into_serde().unwrap();
    let x = game.state.get_mut_crafting(t);
    x.priority = (x.priority + 1) % 5;
}

#[wasm_bindgen]
pub fn use_item(val: &JsValue) {
    info!("Rust enqueue crafting");
    let game: &mut Game = &mut *GLOBAL_DATA.lock().unwrap();
    let t: ItemTypes = val.into_serde().unwrap();
    consume_item(t, game);
    actionless_update(game);
}

#[wasm_bindgen]
pub fn lower_action_count(index: u32) {
    info!("Rust lower action count");
    let game: &mut Game = &mut *GLOBAL_DATA.lock().unwrap();
    game.action_queue.lower_action_count(index);
}

#[wasm_bindgen]
pub fn clear_all_actions() {
    let game: &mut Game = &mut *GLOBAL_DATA.lock().unwrap();
    game.action_queue.clear();
}

#[wasm_bindgen]
pub fn clear_action_count() {
    info!("Rust clear action count");
    let game: &mut Game = &mut *GLOBAL_DATA.lock().unwrap();
    game.action_queue.clear();
}

#[wasm_bindgen]
pub fn prepend_exploration(val: &JsValue, amount: u32) {
    info!("Rust enqueue exploration");
    let game: &mut Game = &mut *GLOBAL_DATA.lock().unwrap();
    let t: AllExplors = val.into_serde().unwrap();
    game.action_queue
        .preppend_action(game.world.get_wexploration(t).to_action_entry(amount));
}

#[wasm_bindgen]
pub fn prepend_crafting(val: &JsValue, amount: u32) {
    info!("Rust enqueue crafting");
    let game: &mut Game = &mut *GLOBAL_DATA.lock().unwrap();
    let t: AllCrafts = val.into_serde().unwrap();
    game.action_queue
        .preppend_action(game.world.get_wcrafting(t).to_action_entry(amount));
}

#[wasm_bindgen]
pub fn prepend_collection(val: &JsValue, amount: u32) {
    info!("Rust enqueue collection");
    let game: &mut Game = &mut *GLOBAL_DATA.lock().unwrap();
    let t: AllCollects = val.into_serde().unwrap();
    game.action_queue
        .preppend_action(game.world.get_wcollection(t).to_action_entry(amount));
}

#[wasm_bindgen]
pub fn append_exploration(val: &JsValue, amount: u32) {
    info!("Rust enqueue exploration");
    let game: &mut Game = &mut *GLOBAL_DATA.lock().unwrap();
    let t: AllExplors = val.into_serde().unwrap();
    game.action_queue
        .append_action(game.world.get_wexploration(t).to_action_entry(amount));
}

#[wasm_bindgen]
pub fn append_crafting(val: &JsValue, amount: u32) {
    info!("Rust enqueue crafting");
    let game: &mut Game = &mut *GLOBAL_DATA.lock().unwrap();
    let t: AllCrafts = val.into_serde().unwrap();
    game.action_queue
        .append_action(game.world.get_wcrafting(t).to_action_entry(amount));
}

#[wasm_bindgen]
pub fn append_collection(val: &JsValue, amount: u32) {
    info!("Rust enqueue exploration");
    let game: &mut Game = &mut *GLOBAL_DATA.lock().unwrap();
    let t: AllCollects = val.into_serde().unwrap();
    game.action_queue
        .append_action(game.world.get_wcollection(t).to_action_entry(amount));
}

#[wasm_bindgen]
pub fn get_icon_by_enum(val: JsValue) -> JsValue {
    let icon_type: IconType = val.into_serde().unwrap();
    let icon: Icon = icon_type.into();
    JsValue::from_serde(&icon).unwrap()
}

#[wasm_bindgen]
pub fn get_world() -> JsValue {
    let game = GLOBAL_DATA.lock().unwrap();
    JsValue::from_serde(&game.world).unwrap()
}

#[wasm_bindgen]
pub fn get_state() -> JsValue {
    let game = GLOBAL_DATA.lock().unwrap();
    // This method is the standard one and might be faster or slower?
    // serde_wasm_bindgen::to_value(&game.state).unwrap()
    serde_wasm_bindgen::to_value(&game.state).unwrap()
}

#[wasm_bindgen]
pub fn get_history() -> JsValue {
    // log::info!("got history");
    let mut game = GLOBAL_DATA.lock().unwrap();
    game.update_history();
    serde_wasm_bindgen::to_value(&game.history).unwrap()
}

#[wasm_bindgen]
pub fn get_user_settings() -> UserSettings {
    let game = GLOBAL_DATA.lock().unwrap();
    game.meta_data.user_settings.clone()
}

#[wasm_bindgen]
pub fn set_user_settings(settings: &UserSettings) {
    let mut game = GLOBAL_DATA.lock().unwrap();
    game.meta_data.user_settings = settings.clone();
}

#[wasm_bindgen]
pub fn get_action_queue() -> JsValue {
    let game = GLOBAL_DATA.lock().unwrap();
    JsValue::from_serde(&game.action_queue.queue).unwrap()
}

#[wasm_bindgen]
pub fn do_rebirth() {
    let game: &mut Game = &mut *GLOBAL_DATA.lock().unwrap();
    if !game.state.status.is_dead {
        return;
    }
    do_rebirth_internal(game);
    info!("Rust did rebirth");
}

#[wasm_bindgen]
pub fn toggle_paused() {
    let mut game = GLOBAL_DATA.lock().unwrap();
    game.meta_data.user_settings.paused = !game.meta_data.user_settings.paused;
}

#[wasm_bindgen]
pub fn paused() {
    let game: &mut Game = &mut *GLOBAL_DATA.lock().unwrap();
    game.meta_data.update_tick_time();
    game.meta_data.paused();
}

#[wasm_bindgen]
pub fn tick() {
    let game: &mut Game = &mut *GLOBAL_DATA.lock().unwrap();
    tick_internal(game);
}

pub fn tick_internal(game: &mut Game) {
    if game.meta_data.should_autosave() {
        do_save(game);
    }
    game.meta_data.update_tick_time();
    if game.just_loaded {
        game.just_loaded = false;
        if game.state.status.is_dead {
            return;
        }
        one_tick(game);
    } else {
        if game.state.status.is_dead {
            return;
        }
        if game.meta_data.missed_time_ticks() < -1.0 {
            game.meta_data.missed_tick();
            return;
        }
        one_tick(game);
    }
}

pub fn one_tick(game: &mut Game) {
    for _ in 0..game.meta_data.game_speed {
        engine_run(game);
    }
}
