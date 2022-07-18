// use crate::info::Info;
use crate::TICK_MS;
use js_sys::Date;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[wasm_bindgen]
pub struct UserSettings {
    pub autosave: bool,
    pub use_saved_ticks: bool,
    pub paused: bool,
}

#[wasm_bindgen]
impl UserSettings {
    #[wasm_bindgen(constructor)]
    pub fn new() -> UserSettings {
        UserSettings {
            autosave: get_autosave_default(),
            use_saved_ticks: true,
            paused: false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MetaData {
    pub user_settings: UserSettings,
    pub game_speed: u32,
    pub last_save_time: f64,
    pub last_tick_time: f64,
    pub missed_time: f64,
    pub saved_ticks: u32,
    // pub info: Info,
}

impl Default for MetaData {
    fn default() -> Self {
        Self::new()
    }
}

impl MetaData {
    pub fn new() -> MetaData {
        MetaData {
            user_settings: UserSettings::new(),
            game_speed: get_game_speed_default(),
            last_save_time: Date::new_0().get_time(),
            last_tick_time: Date::new_0().get_time(),
            missed_time: 0.0,
            saved_ticks: 0,
            // info: Info::new(),
        }
    }

    pub fn should_autosave(&self) -> bool {
        let now = Date::new_0().get_time();
        self.user_settings.autosave && now >= self.last_save_time + (60.0 * 1000.0)
    }

    pub fn handle_run_count(&mut self) -> u32 {
        if self.user_settings.use_saved_ticks {
            let count = self.saved_ticks;
            self.saved_ticks = 0;
            count + 1
        } else {
            self.saved_ticks = 0;
            1
        }
    }

    pub fn set_save_time(&mut self) {
        self.last_save_time = Date::new_0().get_time();
    }

    pub fn update_tick_time(&mut self) {
        let now = Date::new_0().get_time();
        self.missed_time += (now - self.last_tick_time) - TICK_MS;
        // log::info!("delta: {}", (now - self.last_tick_time));
        // log::info!(
        //     "missed_time delta: {}",
        //     (now - self.last_tick_time) - TICK_MS
        // );

        self.last_tick_time = now;
        self.convert_missed_time_to_saved_ticks();
    }
    pub fn convert_missed_time_to_saved_ticks(&mut self) {
        // log::info!("missed_time: {}", self.missed_time);
        let missed_ticks = self.missed_time_ticks();
        if missed_ticks >= 1.0 {
            let missed_ticks = missed_ticks as u32;
            // log::info!("convert: {}", missed_ticks);
            self.saved_ticks += missed_ticks;
            self.missed_time -= missed_ticks as f64 * TICK_MS;
        }
    }

    pub fn missed_time_ticks(&mut self) -> f64 {
        self.missed_time / TICK_MS
    }

    pub fn missed_tick(&mut self) {
        self.missed_time += TICK_MS
    }

    pub fn paused(&mut self) {
        self.saved_ticks = 0;
    }
}

fn get_game_speed_default() -> u32 {
    if cfg!(debug_assertions) {
        100
    } else {
        1
    }
}

fn get_autosave_default() -> bool {
    !cfg!(debug_assertions)
}
