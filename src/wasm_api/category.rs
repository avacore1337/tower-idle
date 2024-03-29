use crate::game::Game;
use crate::types::*;
use crate::GLOBAL_DATA;
use log::info;
use wasm_bindgen::prelude::*;

pub fn got_user_input(game: &mut Game) {
    game.state.status.waiting = false;
}

#[wasm_bindgen]
pub fn prepend_action(category: String, val: &JsValue, amount: u32) {
    info!("prepend action: {}, {:?}", category, val);
    let game: &mut Game = &mut *GLOBAL_DATA.lock().unwrap();
    match category.as_str() {
        "Exploration" => {
            let t: AllExplors = val.into_serde().unwrap();
            game.action_queue
                .preppend_action(game.world.get_wexploration(t).to_action_entry(amount));
        }
        "Collection" => {
            let t: AllCollects = val.into_serde().unwrap();
            game.action_queue
                .preppend_action(game.world.get_wcollection(t).to_action_entry(amount));
        }
        "Crafting" => {
            let t: AllCrafts = val.into_serde().unwrap();
            game.action_queue
                .preppend_action(game.world.get_wcrafting(t).to_action_entry(amount));
        }
        _ => panic!("Unknown category: {}", category),
    }
}

#[wasm_bindgen]
pub fn append_action(category: String, val: &JsValue, amount: u32) {
    info!("append action: {}, {:?}", category, val);
    let game: &mut Game = &mut *GLOBAL_DATA.lock().unwrap();
    match category.as_str() {
        "Exploration" => {
            let t: AllExplors = val.into_serde().unwrap();
            game.action_queue
                .append_action(game.world.get_wexploration(t).to_action_entry(amount));
        }
        "Collection" => {
            let t: AllCollects = val.into_serde().unwrap();
            game.action_queue
                .append_action(game.world.get_wcollection(t).to_action_entry(amount));
        }
        "Crafting" => {
            let t: AllCrafts = val.into_serde().unwrap();
            game.action_queue
                .append_action(game.world.get_wcrafting(t).to_action_entry(amount));
        }
        _ => panic!("Unknown category: {}", category),
    }
}

#[wasm_bindgen]
pub fn toggle_favourite(category: String, val: &JsValue) {
    info!("toggle favourite: {}, {:?}", category, val);
    let game: &mut Game = &mut *GLOBAL_DATA.lock().unwrap();
    got_user_input(game);
    match category.as_str() {
        "Exploration" => {
            let t: AllExplors = val.into_serde().unwrap();
            let x = game.state.get_mut_exploration(t);
            x.favourite = !x.favourite;
        }
        "Collection" => {
            let t: AllCollects = val.into_serde().unwrap();
            let x = game.state.get_mut_collection(t);
            x.favourite = !x.favourite;
        }
        "Crafting" => {
            let t: AllCrafts = val.into_serde().unwrap();
            let x = game.state.get_mut_crafting(t);
            x.favourite = !x.favourite;
        }
        _ => panic!("Unknown category: {}", category),
    }
}

#[wasm_bindgen]
pub fn toggle_priority(category: String, val: &JsValue) {
    info!("toggle priority: {}, {:?}", category, val);
    let game: &mut Game = &mut *GLOBAL_DATA.lock().unwrap();
    got_user_input(game);
    match category.as_str() {
        "Exploration" => {
            let t: AllExplors = val.into_serde().unwrap();
            let x = game.state.get_mut_exploration(t);
            x.priority = (x.priority + 1) % 5;
        }
        "Collection" => {
            let t: AllCollects = val.into_serde().unwrap();
            let x = game.state.get_mut_collection(t);
            x.priority = (x.priority + 1) % 5;
        }
        "Crafting" => {
            let t: AllCrafts = val.into_serde().unwrap();
            let x = game.state.get_mut_crafting(t);
            x.priority = (x.priority + 1) % 5;
        }
        _ => panic!("Unknown category: {}", category),
    }
}
