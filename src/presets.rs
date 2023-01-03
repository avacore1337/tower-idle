use crate::game::GameSave;
use crate::types::*;
use crate::util::*;
use crate::WORLD;
use std::collections::BTreeMap;

// use strum::IntoEnumIterator;

pub fn get_presets() -> BTreeMap<&'static str, GameSave> {
    let mut presets = BTreeMap::new();
    presets.insert("00: Auto start", make_automated_start());
    presets.insert("01: almost auto start", make_almost_automated_start());
    presets.insert("02: F2 expected", make_f2());
    presets.insert("03: F3 expected", make_f3());
    presets.insert("04: F4 expected", make_f4());
    // presets.insert("06: T5 Faith", make_t5_faith());
    // presets.insert("T1 Test_2: Re 3", rebirth_3());

    presets
}

fn make_almost_automated_start() -> GameSave {
    let mut game_save = GameSave::default();
    let state = &mut game_save.state;
    let floor_index = 0;
    for collection in &mut state.floors[floor_index].collections {
        let wcollection = WORLD.get_wcollection(collection.name);
        collection.completion_count = wcollection.automate_limit - 1;
        collection.has_seen = true;
    }
    for crafting in &mut state.floors[floor_index].craftings {
        let wcrafting = WORLD.get_wcrafting(crafting.name);
        crafting.completion_count = wcrafting.automate_limit - 1;
        crafting.has_seen = true;
    }
    for exploration in &mut state.floors[floor_index].explorations {
        let wexploration = WORLD.get_wexploration(exploration.name);
        exploration.completion_count = wexploration.automate_limit - 1;
        exploration.has_seen = true;
    }
    game_save
}

pub fn make_automated_start() -> GameSave {
    let mut game_save = GameSave::default();
    let state = &mut game_save.state;
    override_automatable(state);
    game_save
}

pub fn make_f2() -> GameSave {
    let mut game_save = GameSave::default();
    let state = &mut game_save.state;
    set_floor_to_automatable(state, 0);
    set_talent(state, SkillTypes::Agility, 12.0);
    set_talent(state, SkillTypes::Woodcutting, 16.0);
    set_talent(state, SkillTypes::Crafting, 8.0);
    set_talent(state, SkillTypes::Mining, 12.0);
    set_talent(state, SkillTypes::Alchemy, 12.0);
    game_save
}

fn make_f3() -> GameSave {
    let mut game_save = GameSave::default();
    let state = &mut game_save.state;
    set_up_to_floor_to_automatable(state, 1);
    set_talent(state, SkillTypes::Agility, 25.0);
    set_talent(state, SkillTypes::Woodcutting, 30.0);
    set_talent(state, SkillTypes::Mining, 25.0);
    set_talent(state, SkillTypes::Crafting, 20.0);
    set_talent(state, SkillTypes::Alchemy, 20.0);
    set_talent(state, SkillTypes::Hunting, 22.0);
    set_talent(state, SkillTypes::Fighting, 4.0);
    game_save
}

fn make_f4() -> GameSave {
    let mut game_save = GameSave::default();
    let state = &mut game_save.state;
    set_up_to_floor_to_automatable(state, 2);
    set_talent(state, SkillTypes::Agility, 40.0);
    set_talent(state, SkillTypes::Woodcutting, 40.0);
    set_talent(state, SkillTypes::Mining, 40.0);
    set_talent(state, SkillTypes::Crafting, 40.0);
    set_talent(state, SkillTypes::Alchemy, 40.0);
    set_talent(state, SkillTypes::Hunting, 40.0);
    set_talent(state, SkillTypes::Fighting, 15.0);
    game_save
}

// fn make_current() -> GameSave {
//     let mut game_save = GameSave::default();
//     game_save

// pub fn rebirth_28() -> GameSave {
//     let mut game_save = GameSave::default();
//     let r = &mut game_save.state.rebirth_stats;
//     r.rebirth_count = 11;
//     r.tier = 3;
//     get_upgrades_up_to_tier_max_cost(r, 3, 300.0);
//     set_lower_tier_jobs_to(r, 30);
//     r.max_job_levels[WorkTypes::Hypaspists as usize] = 10;
//     r.max_job_levels[WorkTypes::Farmer as usize] = 25;
//     game_save.state = rebirth(r.clone());

//     let state = &mut game_save.state;

//     set_full_auto(&mut game_save.meta_data.options);
//     state.life_stats.replaying = true;
//     let pi = &mut game_save.previous_inputs;
//     balance_activities(pi, 4000, 50000, &get_run_study_array());
//     pi.register_input_on_tick(40000, AutoSettingTypes::AutoWorkFalse);
//     pi.register_input_on_tick(10000, WorkTypes::Mines);
//     pi.register_input_on_tick(45000, ActivityTypes::Flirt);
//     pi.register_input_on_tick(40000, HousingTypes::LargeCloset);
//     pi.register_input_on_tick(40000, AutoSettingTypes::AutoLivingFalse);
//     pi.register_input_on_tick(45000, AutoSettingTypes::AutoBuyItemFalse);
//     // pi.register_input_on_tick(72000, BoostItemTypes::Burial3);

//     game_save.input = Input::new(&game_save.state);
//     game_save
// }
