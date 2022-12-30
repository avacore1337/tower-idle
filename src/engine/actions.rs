use super::general_action;
use super::value_keys::KeyValues;
use crate::action_mapping::ActionResult;
use crate::game::Game;
use crate::types::*;
use crate::world::collection::{can_collect, Collect};
use crate::world::crafting::{can_pay_crafting, crafting_allowed, need_to_pay_crafting};
use crate::world::exploration::can_explore;

pub fn explore_action(game: &mut Game, exploration: AllExplors) -> ActionResult {
    if !can_explore(exploration, game) {
        return ActionResult::Invalid;
    }
    let wexploration = game.world.get_wexploration(exploration);
    let skill = wexploration.skill;
    game.state.current_skill = skill;
    if wexploration.dps > 0.0 {
        game.intermediate_state
            .add_base(KeyValues::HealthDrain, wexploration.dps, "Exploration damage");
    }
    general_action(game); // Health Update
    let floor_index = FloorTypes::from(exploration) as usize;
    let exploration_index = exploration.to_exploration_index();
    let floor = &mut game.state.floors[floor_index];
    let exploration = &mut floor.explorations[exploration_index];
    let skill = &game.state.skills[skill as usize];
    exploration.current_xp += skill.xp_rate;
    exploration.completion_percentage = (exploration.current_xp * 100.0) / wexploration.required_xp;
    exploration.completion_percentage = f64::min(exploration.completion_percentage, 100.0);
    exploration.ticks_spent += 1;

    if exploration.current_xp > wexploration.required_xp {
        exploration.is_completed = true;
        exploration.ticks_spent = 0;
        exploration.add_completion(1);
        wexploration.on_completed(game);
        ActionResult::Completed(1)
    } else {
        ActionResult::Worked
    }
}

pub fn queue_up_collection(game: &mut Game, crafting: AllCrafts) -> ActionResult {
    // for all collects, if we can collect item for crafting, do

    let wcrafting = game.world.get_wcrafting(crafting);
    let missing_items: Vec<ItemTypes> = wcrafting
        .materials
        .iter()
        .filter(|material| game.state.get_item(material.item).amount < material.amount)
        .map(|material| material.item)
        .collect();
    let wfloor = &game.world.floors[game.state.current_floor as usize];
    for wcollection in wfloor.collections.iter() {
        if let Collect::Item(item) = &wcollection.collect {
            let collection = game.state.get_collection(wcollection.name);
            if missing_items.contains(item) && can_collect(wcollection.name, game) && collection.is_automatable {
                return ActionResult::NewAction(wcollection.to_action_entry(1));
            }
        }
    }
    ActionResult::Invalid
}

pub fn crafting_action(game: &mut Game, crafting: AllCrafts) -> ActionResult {
    let wcrafting = game.world.get_wcrafting(crafting);
    if need_to_pay_crafting(crafting, game) {
        if crafting_allowed(crafting, game) {
            if can_pay_crafting(crafting, game) {
                pay_crafting(game, crafting);
            } else {
                return queue_up_collection(game, crafting);
            }
        } else {
            return ActionResult::Invalid;
        }
    }
    let skill = wcrafting.skill;
    game.state.current_skill = skill;
    general_action(game); // Health Update
    let floor_index = FloorTypes::from(crafting) as usize;
    let crafting_index = crafting.to_crafting_index();
    let floor = &mut game.state.floors[floor_index];
    let crafting = &mut floor.craftings[crafting_index];
    let skill = &game.state.skills[skill as usize];
    crafting.current_xp += skill.xp_rate;
    crafting.completion_percentage = (crafting.current_xp * 100.0) / wcrafting.required_xp;
    crafting.completion_percentage = f64::min(crafting.completion_percentage, 100.0);
    let items_paid = crafting.segments_paid == wcrafting.segments_needed;
    crafting.ticks_spent += 1;
    if crafting.current_xp > wcrafting.required_xp && items_paid {
        crafting.is_completed = true;
        crafting.ticks_spent = 0;
        crafting.completion_percentage = 0.0;
        crafting.current_xp = 0.0;
        crafting.segments_paid = 0;
        crafting.add_completion(1);
        wcrafting.on_completed(game);
        ActionResult::Completed(1)
    } else {
        ActionResult::Worked
    }
}

pub fn collect_action(game: &mut Game, collection: AllCollects) -> ActionResult {
    if !can_collect(collection, game) {
        return ActionResult::Invalid;
    }
    let wcollection = game.world.get_wcollection(collection);
    let skill = wcollection.skill;
    game.state.current_skill = skill;
    general_action(game); // Health Update
    let floor_index = FloorTypes::from(collection) as usize;
    let collection_index = collection.to_collection_index();
    let floor = &mut game.state.floors[floor_index];
    let collection = &mut floor.collections[collection_index];
    let skill = &game.state.skills[skill as usize];
    collection.current_xp += skill.xp_rate;
    collection.completion_percentage = (collection.current_xp * 100.0) / wcollection.required_xp;
    collection.completion_percentage = f64::min(collection.completion_percentage, 100.0);
    collection.ticks_spent += 1;
    if collection.current_xp > wcollection.required_xp {
        // max one completion per tick atm
        collection.ticks_spent = 0;
        collection.current_xp = 0.0;
        match wcollection.collect {
            Collect::Item(item) => game.state.items[item as usize].amount += 1,
            Collect::Mana(amount) => game.state.status.add_health(amount),
        }
        collection.completion_percentage = 0.0;
        collection.add_completion(1);
        wcollection.on_completed(game);
        ActionResult::Completed(1)
    } else {
        ActionResult::Worked
    }
}

pub fn pay_crafting(game: &mut Game, crafting_type: AllCrafts) {
    let wcrafting = game.world.get_wcrafting(crafting_type);
    for i in 0..wcrafting.materials.len() {
        let material = &wcrafting.materials[i];
        game.state.items[material.item as usize].amount -= material.amount;
    }
    let crafting = game.state.get_mut_crafting(crafting_type);
    crafting.segments_paid += 1;
}
