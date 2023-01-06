#![allow(dead_code)]
#![cfg(not(debug_assertions))]
#![allow(non_upper_case_globals)]

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn toggle_override_automatable() {}

#[wasm_bindgen]
pub fn toggle_auto_rebirth() {}

#[wasm_bindgen]
pub fn debug_die() {}

#[wasm_bindgen]
pub fn print_debug_intermediate() {}

#[wasm_bindgen]
pub fn print_debug_state() {}

#[wasm_bindgen]
pub fn print_debug_history() {}

#[wasm_bindgen]
pub fn print_debug_meta() {}

#[wasm_bindgen]
pub fn print_debug_action_queue() {}

#[wasm_bindgen]
pub fn single_tick() {}

#[wasm_bindgen]
pub fn set_gamespeed(_speed: u32) {}

#[wasm_bindgen]
pub fn get_preset_saves() -> JsValue {
    JsValue::NULL
}

#[wasm_bindgen]
pub fn set_preset_saves(_preset_name: &str) {}

#[wasm_bindgen]
pub fn test() {}
