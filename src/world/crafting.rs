use crate::action_queue::ActionEntry;
use crate::game::Game;
use crate::icon::{Icon, IconType};
use crate::types::*;
use serde::Serialize;
use strum::IntoEnumIterator;
use tsify::Tsify;

#[derive(Tsify, Serialize, Clone)]
pub struct WMaterial {
    pub item: ItemTypes,
    pub amount: u32,
}

#[derive(Tsify, Serialize, Clone)]
pub struct WCrafting {
    pub name: AllCrafts,
    pub display_name: &'static str,
    pub description: &'static str,
    pub materials: Vec<WMaterial>,
    pub segments_needed: u32,
    pub skill: SkillTypes,
    pub required_xp: f64,
    pub icon: Icon,
    pub repeatable: bool,
    pub automate_limit: u32,
}

impl WCrafting {
    pub fn on_completed(&self, game: &mut Game) {
        match self.name {
            AllCrafts::First(First) => match First {
                F1Crafts::CrushCrystal => {
                    game.state.status.add_health(2.5);
                }
                F1Crafts::Altar => game.state.get_mut_boost(BoostTypes::Altar).unlocked = true,
                F1Crafts::Axe => game.state.get_mut_boost(BoostTypes::Axe).unlocked = true,
                _ => {}
            },
            AllCrafts::Second(Second) => match Second {
                F2Crafts::BetterAxe => game.state.get_mut_boost(BoostTypes::BetterAxe).unlocked = true,
                F2Crafts::Spear => game.state.get_mut_boost(BoostTypes::Spear).unlocked = true,
                F2Crafts::PoisonTippedSpear => game.state.get_mut_boost(BoostTypes::PoisonTippedSpear).unlocked = true,
                F2Crafts::Bridge => {}
                F2Crafts::BuyKey => {}
            },
            AllCrafts::Third(Third) => match Third {
                // F3Crafts::Test => {}
                _ => {}
            },
        }
    }

    pub fn to_action_entry(&self, amount: u32) -> ActionEntry {
        ActionEntry {
            action_key: self.name.to_record_key(),
            display_name: self.display_name.to_string(),
            amount,
            skill_icon: self.icon.clone().into(),
            category_icon: IconType::Crafting,
        }
    }
}

pub fn should_be_automatable_crafting(crafting_type: AllCrafts, game: &Game) -> bool {
    let crafting = game.state.get_crafting(crafting_type);
    let wcrafting = game.world.get_wcrafting(crafting_type);
    crafting.completion_count >= wcrafting.automate_limit
}

pub fn get_craftings(floor: FloorTypes) -> Vec<WCrafting> {
    match floor {
        FloorTypes::Starting => get_first_floor_craftings(),
        FloorTypes::Second => get_second_floor_craftings(),
        FloorTypes::Third => get_third_floor_craftings(),
    }
}

pub fn get_first_floor_craftings() -> Vec<WCrafting> {
    let mut craftings = Vec::new();
    for crafting_type in F1Crafts::iter() {
        let wrapped_type = crafting_type.into();
        let crafting = match crafting_type {
            F1Crafts::DoorHandle => WCrafting {
                name: wrapped_type,
                display_name: "Crude lockpick",
                description: "A lockpick to open a very rudementary door",
                materials: vec![WMaterial {
                    item: ItemTypes::MetalScrap,
                    amount: 1,
                }],
                segments_needed: 5,
                skill: SkillTypes::Crafting,
                required_xp: 40.0,
                icon: IconType::Crafting.into(),
                repeatable: false,
                automate_limit: 4,
            },
            F1Crafts::Axe => WCrafting {
                name: wrapped_type,
                display_name: "Axe",
                description: "An axe that multiplies your woodcutting with 1.5",
                materials: vec![
                    WMaterial {
                        item: ItemTypes::Stone,
                        amount: 1,
                    },
                    WMaterial {
                        item: ItemTypes::Wood,
                        amount: 1,
                    },
                ],
                segments_needed: 5,
                skill: SkillTypes::Crafting,
                required_xp: 40.0,
                icon: IconType::Crafting.into(),
                repeatable: false,
                automate_limit: 4,
            },
            F1Crafts::RepairAlchemy => WCrafting {
                name: wrapped_type,
                display_name: "Repair alchemy table",
                description:
                    "A Table where you can performe alchemy, needed to convert the big crystals into something useful",
                materials: vec![
                    WMaterial {
                        item: ItemTypes::Stone,
                        amount: 1,
                    },
                    WMaterial {
                        item: ItemTypes::Wood,
                        amount: 1,
                    },
                ],
                segments_needed: 10,
                skill: SkillTypes::Crafting,
                required_xp: 100.0,
                icon: IconType::Crafting.into(),
                repeatable: false,
                automate_limit: 4,
            },
            F1Crafts::CrushCrystal => WCrafting {
                name: wrapped_type,
                display_name: "Crush crystal",
                description: "You can Crush the crystals into something more digestable",
                materials: vec![WMaterial {
                    item: ItemTypes::Crystal,
                    amount: 1,
                }],
                segments_needed: 1,
                skill: SkillTypes::Alchemy,
                required_xp: 20.0,
                icon: IconType::Crafting.into(),
                repeatable: true,
                automate_limit: 100,
            },
            F1Crafts::Altar => WCrafting {
                name: wrapped_type,
                display_name: "Altar",
                description: "The altar of a god",
                materials: vec![
                    WMaterial {
                        item: ItemTypes::Stone,
                        amount: 1,
                    },
                    WMaterial {
                        item: ItemTypes::Wood,
                        amount: 1,
                    },
                ],
                segments_needed: 10,
                skill: SkillTypes::Crafting,
                required_xp: 3000.0,
                icon: IconType::Crafting.into(),
                repeatable: false,
                automate_limit: 4,
            },
        };
        craftings.push(crafting);
    }
    craftings
}

pub fn get_second_floor_craftings() -> Vec<WCrafting> {
    let mut craftings = Vec::new();
    for crafting_type in F2Crafts::iter() {
        let wrapped_type = crafting_type.into();
        let crafting = match crafting_type {
            F2Crafts::Spear => WCrafting {
                name: wrapped_type,
                display_name: "Spear",
                description: "TODO",
                materials: vec![WMaterial {
                    item: ItemTypes::Stick,
                    amount: 1,
                }],
                segments_needed: 10,
                skill: SkillTypes::Crafting,
                required_xp: 200.0,
                icon: IconType::Crafting.into(),
                repeatable: false,
                automate_limit: 4,
            },
            F2Crafts::PoisonTippedSpear => WCrafting {
                name: wrapped_type,
                display_name: "PoisonTippedSpear",
                description: "TODO",
                materials: vec![WMaterial {
                    item: ItemTypes::Poison,
                    amount: 1,
                }],
                segments_needed: 10,
                skill: SkillTypes::Crafting,
                required_xp: 900.0,
                icon: IconType::Crafting.into(),
                repeatable: false,
                automate_limit: 4,
            },
            F2Crafts::BetterAxe => WCrafting {
                name: wrapped_type,
                display_name: "Flint axe",
                description: "TODO",
                materials: vec![WMaterial {
                    item: ItemTypes::Flint,
                    amount: 1,
                }],
                segments_needed: 10,
                skill: SkillTypes::Crafting,
                required_xp: 300.0,
                icon: IconType::Crafting.into(),
                repeatable: false,
                automate_limit: 4,
            },
            F2Crafts::Bridge => WCrafting {
                name: wrapped_type,
                display_name: "Bridge",
                description: "TODO",
                materials: vec![WMaterial {
                    item: ItemTypes::Log,
                    amount: 1,
                }],
                segments_needed: 10,
                skill: SkillTypes::Crafting,
                required_xp: 500.0,
                icon: IconType::Crafting.into(),
                repeatable: false,
                automate_limit: 4,
            },
            F2Crafts::BuyKey => WCrafting {
                name: wrapped_type,
                display_name: "BuyKey",
                description: "TODO",
                materials: vec![WMaterial {
                    item: ItemTypes::Fur,
                    amount: 10,
                }],
                segments_needed: 1,
                skill: SkillTypes::Crafting,
                required_xp: 10.0,
                icon: IconType::Crafting.into(),
                repeatable: false,
                automate_limit: 4,
            },
        };
        craftings.push(crafting);
    }
    craftings
}

pub fn get_third_floor_craftings() -> Vec<WCrafting> {
    let mut craftings = Vec::new();
    for crafting_type in F3Crafts::iter() {
        let wrapped_type = crafting_type.into();
        let crafting = match crafting_type {
            F3Crafts::Test => WCrafting {
                name: wrapped_type,
                display_name: "Spear",
                description: "TODO",
                materials: vec![WMaterial {
                    item: ItemTypes::Stick,
                    amount: 1,
                }],
                segments_needed: 10,
                skill: SkillTypes::Crafting,
                required_xp: 12.0,
                icon: IconType::Crafting.into(),
                repeatable: false,
                automate_limit: 4,
            },
        };
        craftings.push(crafting);
    }
    craftings
}

fn has_explored(exploration: AllExplors, game: &Game) -> bool {
    game.state.get_exploration(exploration).is_completed
}

pub fn should_be_visible_crafting(crafting_type: AllCrafts, game: &Game) -> bool {
    let wcrafting = game.world.get_wcrafting(crafting_type);
    let crafting = game.state.get_crafting(crafting_type);
    if crafting.is_completed && !wcrafting.repeatable {
        return false;
    }
    match crafting_type {
        AllCrafts::First(First) => match First {
            F1Crafts::DoorHandle => has_explored(F1Explors::Hallway.into(), game),
            F1Crafts::Axe => has_explored(F1Explors::SideArea.into(), game),
            F1Crafts::RepairAlchemy => has_explored(F1Explors::Laboratory.into(), game),
            F1Crafts::CrushCrystal => {
                game.state.get_item(ItemTypes::Crystal).is_visible
                    && game.state.get_crafting(F1Crafts::RepairAlchemy.into()).is_completed
            }
            F1Crafts::Altar => has_explored(F1Explors::Shrine.into(), game),
        },
        AllCrafts::Second(Second) => match Second {
            F2Crafts::Spear => has_explored(F2Explors::ExploreHallway.into(), game),
            F2Crafts::PoisonTippedSpear => has_explored(F2Explors::Laboratory.into(), game),
            F2Crafts::BetterAxe => has_explored(F2Explors::RemoveDebris.into(), game),
            F2Crafts::Bridge => has_explored(F2Explors::Clearing.into(), game),
            F2Crafts::BuyKey => has_explored(F2Explors::Haggle.into(), game),
        },
        AllCrafts::Third(Third) => match Third {
            F3Crafts::Test => false,
        },
    }
}

pub fn has_restriction(crafting_type: AllCrafts, _game: &Game) -> bool {
    match crafting_type {
        // AllCrafts::First(F1Crafts::CrushCrystal) => {
        //     game.state.get_item(ItemTypes::CrystalFragments).is_maxed()
        // }
        _ => false,
    }
}

pub fn crafting_allowed(crafting_type: AllCrafts, game: &Game) -> bool {
    let crafting = game.state.get_crafting(crafting_type);
    crafting.is_visible && !has_restriction(crafting_type, game)
}

pub fn can_craft(crafting_type: AllCrafts, game: &Game) -> bool {
    if !crafting_allowed(crafting_type, game) {
        return false;
    }
    if need_to_pay_crafting(crafting_type, game) {
        can_pay_crafting(crafting_type, game)
    } else {
        true
    }
}

pub fn can_pay_crafting(crafting_type: AllCrafts, game: &Game) -> bool {
    let wcrafting = game.world.get_wcrafting(crafting_type);
    for i in 0..wcrafting.materials.len() {
        let material = &wcrafting.materials[i];
        let item_amount = game.state.items[material.item as usize].amount;
        if item_amount < material.amount {
            return false;
        }
    }
    return true;
}

pub fn need_to_pay_crafting(crafting_type: AllCrafts, game: &Game) -> bool {
    let wcrafting = game.world.get_wcrafting(crafting_type);
    let crafting = game.state.get_crafting(crafting_type);
    if crafting.segments_paid == wcrafting.segments_needed {
        return false;
    }
    let segment_paid_percentage = (crafting.segments_paid as f64 / wcrafting.segments_needed as f64) * 100.0;
    crafting.completion_percentage >= segment_paid_percentage
}
