// pub mod auto_functions;
pub mod actions;
pub mod intermediate_state;
pub mod value_keys;

use crate::action_mapping::ActionResult;
use crate::game::Game;
use crate::state::rebirth;
use crate::state::skill::Skill;
use crate::state::status::base_values;
use crate::types::*;
use crate::world::collection::{should_be_automatable_collection, should_be_visible_collection};
use crate::world::crafting::{should_be_automatable_crafting, should_be_visible_crafting};
use crate::world::exploration::{should_be_automatable_exploration, should_be_visible_exploration};
use crate::world::item::should_be_visible_item;
use crate::world::skill::{should_be_visible_skill, WSkill};
use crate::TICK_RATE;
use crate::WORLD;
use intermediate_state::IntermediateState;
use strum::IntoEnumIterator;
use value_keys::KeyValues;

pub fn actionless_update(game: &mut Game) {
    // init all the stats for the game
    pre_action(game);
    update_frontend_values(game);
    for skill_type in SkillTypes::iter() {
        let skill: &mut Skill = &mut game.state.skills[skill_type as usize];
        let wskill = &game.world.skills[skill_type as usize];
        skill.level_xp_required = calculate_skill_next_level_xp_neeeded(skill, wskill);
        skill.talent_xp_required = calculate_skill_next_talent_xp_neeeded(skill, wskill);
    }
}

pub fn do_rebirth_internal(game: &mut Game) {
    game.history.rebirth_update(&game.state);
    game.state.status.reincarnation += 1;
    game.state = rebirth(&game.state);
    actionless_update(game);
}

pub fn engine_run(game: &mut Game) {
    if game.state.status.is_dead {
        if game.state.status.auto_rebirth {
            do_rebirth_internal(game)
        }
        return;
    }
    if game.state.status.waiting && game.action_queue.is_empty() {
        // log::info!("no action and waiting");
        return;
    }
    let run_count = game.meta_data.handle_run_count();
    for _ in 0..run_count {
        pre_action(game);
        do_action(game);
        // frontend read values
        update_frontend_values(game);
        if game.state.status.is_dead || game.state.status.waiting {
            return;
        }
    }
}

pub fn do_action(game: &mut Game) {
    match execute_action(game) {
        ActionResult::Worked => (),
        ActionResult::NewAction(action) => {
            game.action_queue.preppend_action(action);
            do_action(game)
        }
        ActionResult::Completed(amount) => {
            let action = game.action_queue.get_first_mut().unwrap();
            action.amount -= amount;
            if action.amount <= 0 {
                game.action_queue.delete_first();
            }
        }
        ActionResult::Invalid => {
            game.action_queue.delete_first();
        }
        ActionResult::Waiting => set_waiting(game),
    }
}

pub fn auto_schedule_action(game: &mut Game) -> bool {
    let wfloor = &game.world.floors[game.state.current_floor as usize];
    let coll_prios = wfloor.collections.iter().map(|x| Priority::new(x.name, game));
    let craft_prios = wfloor.craftings.iter().map(|x| Priority::new(x.name, game));
    let explo_prios = wfloor.explorations.iter().map(|x| Priority::new(x.name, game));
    let res = coll_prios
        .chain(craft_prios)
        .chain(explo_prios)
        .reduce(|accum, item| if accum >= item { accum } else { item });
    if let Some(higest_priority) = res {
        if higest_priority.is_doable && higest_priority.is_automatable && higest_priority.user_priority > 0 {
            game.action_queue.preppend_action(higest_priority.action_entry);
            return true;
        }
    }
    false
}

pub fn set_waiting(game: &mut Game) {
    // signal back to UI that we are forced to wait
    // log::info!("no actions");
    game.state.status.waiting = true;
}

pub fn execute_action(game: &mut Game) -> ActionResult {
    let maybe_action = game.action_queue.get_first();
    match maybe_action {
        Some(action) => {
            game.state.status.waiting = false;
            let mapping = game.world.action_mapping.lock().unwrap();
            let function = mapping.action_functions.get(&action.action_key).unwrap();
            // log::info!("executing action");
            function(game)
        }
        None => {
            if auto_schedule_action(game) {
                // we managed to schedule an action, try to run queue now
                game.state.status.waiting = false;
                execute_action(game)
            } else {
                // No action was available
                // log::info!("Could not schedule action");
                ActionResult::Waiting
            }
        }
    }
}

pub fn update_frontend_values(game: &mut Game) {
    update_unlocks(game);
    calculate_completion_times(game);
    update_status(game);
}

pub fn pre_action(game: &mut Game) {
    game.intermediate_state = IntermediateState::new();
    apply_boosts(game);
    apply_skill_multipliers(game);
    calculate_skill_rates(game);

    // Base values for status values
    base_values(game);
    status_multipliers(game);
}

fn status_multipliers(game: &mut Game) {
    game.intermediate_state.add_base(
        KeyValues::HealthDrain,
        0.20 * 2.0_f64.powf(game.state.current_floor as u64 as f64),
        "level drain",
    );
}

pub fn update_unlocks(game: &mut Game) {
    for collection in AllCollects::iter() {
        game.state.get_mut_collection(collection).is_visible = should_be_visible_collection(collection, game);
        let mut collection = game.state.get_mut_collection(collection);
        collection.has_seen |= collection.is_visible;
    }
    for crafting in AllCrafts::iter() {
        game.state.get_mut_crafting(crafting).is_visible = should_be_visible_crafting(crafting, game);
        let mut crafting = game.state.get_mut_crafting(crafting);
        crafting.has_seen |= crafting.is_visible;
    }
    for exploration in AllExplors::iter() {
        game.state.get_mut_exploration(exploration).is_visible = should_be_visible_exploration(exploration, game);
        let mut exploration = game.state.get_mut_exploration(exploration);
        exploration.has_seen |= exploration.is_visible;
    }
    for item_type in ItemTypes::iter() {
        if !game.state.items[item_type as usize].is_visible {
            game.state.items[item_type as usize].is_visible = should_be_visible_item(item_type, game);
        }
    }
    for stat in SkillTypes::iter() {
        game.state.skills[stat as usize].is_visible = should_be_visible_skill(stat, game);
    }
}

fn apply_boosts(game: &mut Game) {
    for boost in WORLD.boosts.iter() {
        boost.get_boost_gains(game);
    }
}

pub fn calculate_completion_times(game: &mut Game) {
    for exploration_type in AllExplors::iter() {
        // log::info!("exploration: {:#?}", exploration_type);
        let wexploration = game.world.get_wexploration(exploration_type);
        let xp_rate = game.state.skills[wexploration.skill as usize].xp_rate;
        let exploration = game.state.get_mut_exploration(exploration_type);
        exploration.ticks_to_complete = f64::min(f64::ceil(wexploration.required_xp / xp_rate), u32::MAX as f64) as u32;
    }
    for collection_type in AllCollects::iter() {
        // log::info!("collection: {:#?}", collection_type);
        let wcollection = game.world.get_wcollection(collection_type);
        let xp_rate = game.state.skills[wcollection.skill as usize].xp_rate;
        let collection = game.state.get_mut_collection(collection_type);
        collection.ticks_to_complete = f64::min(f64::ceil(wcollection.required_xp / xp_rate), u32::MAX as f64) as u32;
    }
    for crafting_type in AllCrafts::iter() {
        // log::info!("collection: {:#?}", crafting_type);
        let wcraft = game.world.get_wcrafting(crafting_type);
        let xp_rate = game.state.skills[wcraft.skill as usize].xp_rate;
        let craft = game.state.get_mut_crafting(crafting_type);
        craft.ticks_to_complete = f64::min(f64::ceil(wcraft.required_xp / xp_rate), u32::MAX as f64) as u32;
    }
}

fn apply_skill_multipliers(game: &mut Game) {
    for skill in WORLD.skills.iter() {
        skill.get_skills_gains(game);
    }
}

fn calculate_skill_rates(game: &mut Game) {
    for skill_type in SkillTypes::iter() {
        let skill: &mut Skill = &mut game.state.skills[skill_type as usize];
        // let wskill = &game.world.skills[skill_type as usize];
        let multiplier = game.intermediate_state.get_multiplier(skill_type.into());
        skill.xp_rate = multiplier / TICK_RATE;
        skill.total_multiplier = multiplier;
    }
}

fn general_action(game: &mut Game) {
    gain_talent_xp(game);
    gain_level_xp(game);
    update_status_values(game);
}

fn gain_level_xp(game: &mut Game) {
    let skill_type = game.state.current_skill;
    let skill: &mut Skill = &mut game.state.skills[skill_type as usize];
    let wskill = &game.world.skills[skill_type as usize];
    skill.level_current_xp += skill.xp_rate;
    let mut next_level_xp_needed = calculate_skill_next_level_xp_neeeded(skill, wskill);
    while skill.level_current_xp > next_level_xp_needed {
        skill.level += 1.0;
        skill.level_current_xp -= next_level_xp_needed;
        next_level_xp_needed = calculate_skill_next_level_xp_neeeded(skill, wskill);
    }
    skill.level_xp_required = next_level_xp_needed;
    skill.level_percent = (skill.level_current_xp * 100.0) / next_level_xp_needed;
}

fn gain_talent_xp(game: &mut Game) {
    let skill_type = game.state.current_skill;
    let skill: &mut Skill = &mut game.state.skills[skill_type as usize];
    let wskill = &game.world.skills[skill_type as usize];
    skill.talent_current_xp += skill.xp_rate;
    let mut next_talent_xp_needed = calculate_skill_next_talent_xp_neeeded(skill, wskill);
    while skill.talent_current_xp > next_talent_xp_needed {
        skill.talent += 1.0;
        skill.talent_current_xp -= next_talent_xp_needed;
        next_talent_xp_needed = calculate_skill_next_talent_xp_neeeded(skill, wskill);
    }
    skill.talent_xp_required = next_talent_xp_needed;
    skill.talent_percent = (skill.talent_current_xp * 100.0) / next_talent_xp_needed;
}

fn update_status_values(game: &mut Game) {
    let status = &mut game.state.status;
    let intermediate = &game.intermediate_state;
    status.tick += 1;
    status.health_drain = intermediate.get_value(KeyValues::HealthDrain);
    status.current_health -= status.health_drain / TICK_RATE;
}

fn update_status(game: &mut Game) {
    let status = &mut game.state.status;
    if status.current_health <= 0.0 {
        status.current_health_percentage = 0.0;
        status.is_dead = true;
        death_update(game);
    } else {
        status.current_health_percentage = status.current_health / status.max_health * 100.0;
    }
}

fn death_update(game: &mut Game) {
    log::info!("Doing death update");
    game.history.pre_death_clear();
    for collection in AllCollects::iter() {
        let automate = should_be_automatable_collection(collection, game);
        let mut collection = game.state.get_mut_collection(collection);
        if !collection.is_automatable && automate {
            collection.is_automatable = true;
            collection.is_newly_automatable = true;
        }
    }
    for crafting in AllCrafts::iter() {
        let automate = should_be_automatable_crafting(crafting, game);
        let mut crafting = game.state.get_mut_crafting(crafting);
        if !crafting.is_automatable && automate {
            crafting.is_automatable = true;
            crafting.is_newly_automatable = true;
        }
    }
    for exploration in AllExplors::iter() {
        let automate = should_be_automatable_exploration(exploration, game);
        let mut exploration = game.state.get_mut_exploration(exploration);
        if !exploration.is_automatable && automate {
            exploration.is_automatable = true;
            exploration.is_newly_automatable = true;
        }
    }
}

fn calculate_skill_next_level_xp_neeeded(skill: &mut Skill, wskill: &WSkill) -> f64 {
    20.0 * 1.10_f64.powf(skill.level) * wskill.xp_req_modifier
}

fn calculate_skill_next_talent_xp_neeeded(skill: &mut Skill, wskill: &WSkill) -> f64 {
    80.0 * 1.10_f64.powf(skill.talent) * wskill.xp_req_modifier
}
