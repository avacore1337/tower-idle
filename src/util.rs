#![allow(dead_code)]

use crate::engine::engine_run;
use crate::game::Game;
use crate::state::State;
use crate::types::*;
use crate::WORLD;

// use strum::IntoEnumIterator;

// pub fn get_upgrades_up_to_current_tier(rebirth_stats: &mut RebirthStats) {
//     get_upgrades_up_to_tier(rebirth_stats, rebirth_stats.tier)
// }

pub fn run_until_dead(game: &mut Game) {
    while !game.state.status.is_dead {
        engine_run(game);
    }
}

// pub fn do_test_rebirth(game: &mut Game) {
//     game.state.rebirth_stats.rebirth_count += 1;
//     game.state = rebirth(game.state.rebirth_stats.clone());
//     game.input = Input::new(&game.state);
// }
//

pub fn set_talents(state: &mut State, skill_types: Vec<SkillTypes>, amount: f64) {
    for skill_type in skill_types.into_iter() {
        state.skills[skill_type as usize].starting_talent = amount;
        state.skills[skill_type as usize].talent = amount;
    }
}

pub fn set_talent(state: &mut State, skill_type: SkillTypes, amount: f64) {
    state.skills[skill_type as usize].starting_talent = amount;
    state.skills[skill_type as usize].talent = amount;
}

pub fn set_floor_to_automatable(state: &mut State, floor_index: usize) {
    state.floors[floor_index].has_seen = true;
    for collection in &mut state.floors[floor_index].collections {
        // let wcollection = WORLD.get_wcollection(collection.name);
        // collection.completion_count = wcollection.automate_limit;
        collection.is_automatable = true;
        collection.has_seen = true;
    }
    for crafting in &mut state.floors[floor_index].craftings {
        // let wcrafting = WORLD.get_wcrafting(crafting.name);
        // crafting.completion_count = wcrafting.automate_limit;
        crafting.is_automatable = true;
        crafting.has_seen = true;
    }
    for exploration in &mut state.floors[floor_index].explorations {
        // let wexploration = WORLD.get_wexploration(exploration.name);
        // exploration.completion_count = wexploration.automate_limit;
        exploration.is_automatable = true;
        exploration.has_seen = true;
    }
    tune_floor_priority(state, floor_index);
}

pub fn override_automatable(state: &mut State) {
    state.status.override_automatable = true;
    set_all_floors_to_automatable(state);
}

pub fn set_all_floors_to_automatable(state: &mut State) {
    set_up_to_floor_to_automatable(state, FLOOR_SIZE - 1);
}

pub fn set_up_to_floor_to_automatable(state: &mut State, floor_index: usize) {
    for index in 0..(floor_index + 1) {
        set_floor_to_automatable(state, index);
    }
}

pub fn set_exploration_to(state: &mut State, exploration: AllExplors, priority: u32) {
    state.get_mut_exploration(exploration.into()).priority = priority;
}

pub fn set_collection_to(state: &mut State, collection: AllCollects, priority: u32) {
    state.get_mut_collection(collection.into()).priority = priority;
}

pub fn set_crafting_to(state: &mut State, crafting: AllCrafts, priority: u32) {
    state.get_mut_crafting(crafting.into()).priority = priority;
}

pub fn tune_floor_priority(state: &mut State, floor_index: usize) {
    // let floor = state.floors[floor_index];
    match floor_index {
        0..=FLOOR_SIZE => match floor_index {
            0 => {
                set_exploration_to(state, F1Explors::BackToMassive.into(), 1);
                set_exploration_to(state, F1Explors::Shrine.into(), 1);
                set_crafting_to(state, F1Crafts::RepairAlchemy.into(), 4);
                set_crafting_to(state, F1Crafts::CrushCrystal.into(), 4);
            }
            1 => {}
            2 => {}
            _ => {}
        },
        _ => panic!("Trying to access floor outside of floor range"),
    }
}

pub fn get_mana_for_floor(state: &mut State, floor_index: usize) {
    for collection in &mut state.floors[floor_index].collections {
        let wcollection = WORLD.get_wcollection(collection.name);
        match wcollection.collect {
            Collect::Mana(amount) => {
                state.status.add_health(amount * 25.0);
                collection.completion_count = 50;
            }
            _ => {}
        }
    }
    for crafting in &mut state.floors[floor_index].craftings {
        let wcrafting = WORLD.get_wcrafting(crafting.name);
        match wcrafting.craft {
            Craft::Mana(amount) => {
                state.status.add_health(amount * 25.0);
                crafting.completion_count = 50;
            }
            _ => {}
        }
    }
}

// Gain mana with a certain amount of clear of the entire floor
// pub fn get_mana_for_floor_clears(state: &mut State, floor_index: usize, clear_count: u32) {
//     for collection in &mut state.floors[floor_index].collections {
//         let wcollection = WORLD.get_wcollection(collection.name);
//         // le
//         let max_completions = match wcollection.max_completions {
//             Some(max_completions) => max_completions,
//             None => 0,
//         };
//         let clear_multiplier = 20.0 * f64::log10(clear_count * max_completions + 20);
//         match wcollection.collect {
//             Collect::Mana(amount) => state.status.add_health(amount * clear_multiplier),
//             _ => {}
//         }
//     }
//     for crafting in &mut state.floors[floor_index].craftings {
//         let wcrafting = WORLD.get_wcrafting(crafting.name);
//         match wcrafting.craft {
//             Craft::Mana(amount) => state.status.add_health(amount * 25.0),
//             _ => {}
//         }
//     }
// }
