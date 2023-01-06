use crate::action_queue::ActionEntry;
use crate::game::Game;
use crate::icon::{Icon, IconType};
use crate::types::*;
use serde::Serialize;
use strum::IntoEnumIterator;
use tsify::Tsify;

#[derive(Tsify, Serialize, Clone)]
pub struct WCollection {
    pub name: AllCollects,
    pub display_name: &'static str,
    pub description: &'static str,
    pub skill: SkillTypes,
    pub required_xp: f64,
    pub icon: Icon,
    pub collect: Collect,
    pub max_completions: Option<u32>,
    pub automate_limit: u32,
}

impl WCollection {
    pub fn on_completed(&self, _game: &mut Game) {
        match self.name {
            _ => (),
        }
    }

    pub fn to_action_entry(&self, amount: u32) -> ActionEntry {
        ActionEntry {
            action_key: self.name.to_record_key(),
            display_name: self.display_name.to_string(),
            amount,
            skill_icon: self.icon.clone().into(),
            category_icon: IconType::Collection,
        }
    }
}

pub fn get_collections(floor: FloorTypes) -> Vec<WCollection> {
    match floor {
        FloorTypes::Starting => get_first_floor_collections(),
        FloorTypes::Second => get_second_floor_collections(),
        FloorTypes::Third => get_third_floor_collections(),
    }
}

pub fn get_first_floor_collections() -> Vec<WCollection> {
    let mut collections = Vec::new();
    for collection_type in F1Collects::iter() {
        let wrapped_type = collection_type.into();
        let collection = match collection_type {
            F1Collects::MagicDust => WCollection {
                name: wrapped_type,
                display_name: "Gather dust",
                description: "A white powder that's good for you",
                skill: SkillTypes::Agility,
                required_xp: 5.0,
                icon: IconType::Agility.into(),
                collect: Collect::Mana(1.0),
                max_completions: Some(30),
                automate_limit: 90,
            },
            F1Collects::MetalScrap => WCollection {
                name: wrapped_type,
                display_name: "Scavenge nails",
                description: "You break apart furniture for nails. If you need metal, you need metal",
                skill: SkillTypes::Woodcutting,
                required_xp: 10.0,
                icon: IconType::Agility.into(),
                collect: Collect::Item(collection_type.into()),
                max_completions: Some(5),
                automate_limit: 20,
            },
            F1Collects::Rock => WCollection {
                name: wrapped_type,
                display_name: "Pick up rocks",
                description: "Can be mined for stones",
                skill: SkillTypes::Mining,
                required_xp: 15.0,
                icon: IconType::Mining.into(),
                collect: Collect::Item(collection_type.into()),
                max_completions: None,
                automate_limit: 50,
            },
            F1Collects::Wood => WCollection {
                name: wrapped_type,
                display_name: "Collect wood",
                description: "The furniture can be chopped down for wood",
                skill: SkillTypes::Woodcutting,
                required_xp: 15.0,
                icon: IconType::Woodcutting.into(),
                collect: Collect::Item(collection_type.into()),
                max_completions: None,
                automate_limit: 50,
            },
            F1Collects::CrystalFragments => WCollection {
                name: wrapped_type,
                display_name: "Gather slivers",
                description: "You see a bunch of small crystal fragments lying around",
                skill: SkillTypes::Agility,
                required_xp: 15.0,
                icon: IconType::Agility.into(),
                collect: Collect::Mana(3.0),
                max_completions: Some(5),
                automate_limit: 20,
            },
            F1Collects::Crystal => WCollection {
                name: wrapped_type,
                display_name: "Collect crystals",
                description: "There is a large chunk of crystal ore that you can collect crystals from",
                skill: SkillTypes::Mining,
                required_xp: 20.0,
                icon: IconType::Mining.into(),
                collect: Collect::Item(collection_type.into()),
                max_completions: Some(15),
                automate_limit: 60,
            },
        };
        collections.push(collection);
    }
    collections
}

pub fn get_second_floor_collections() -> Vec<WCollection> {
    let mut collections = Vec::new();
    for collection_type in F2Collects::iter() {
        let wrapped_type = collection_type.into();
        let collection = match collection_type {
            F2Collects::HuntRat => WCollection {
                name: wrapped_type,
                display_name: "Hunt rats",
                description: "Hunting down rats might be useful",
                skill: SkillTypes::Hunting,
                required_xp: 20.0,
                icon: IconType::Hunting.into(),
                collect: Collect::Mana(3.0),
                max_completions: Some(20),
                automate_limit: 80,
            },
            F2Collects::Stick => WCollection {
                name: wrapped_type,
                display_name: "Stick",
                description: "",
                skill: SkillTypes::Woodcutting,
                required_xp: 20.0,
                icon: IconType::Woodcutting.into(),
                collect: Collect::Item(collection_type.into()),
                max_completions: None,
                automate_limit: 80,
            },
            F2Collects::ExtractCrystal => WCollection {
                name: wrapped_type,
                display_name: "Extract shards",
                description: "",
                skill: SkillTypes::Crafting,
                required_xp: 30.0,
                icon: IconType::Hunting.into(),
                collect: Collect::Mana(4.0),
                max_completions: Some(10),
                automate_limit: 40,
            },
            F2Collects::HuntRabbits => WCollection {
                name: wrapped_type,
                display_name: "Hunt rabbits",
                description: "",
                skill: SkillTypes::Hunting,
                required_xp: 40.0,
                icon: IconType::Hunting.into(),
                collect: Collect::Mana(5.0),
                max_completions: Some(20),
                automate_limit: 80,
            },
            F2Collects::Flint => WCollection {
                name: wrapped_type,
                display_name: "Flint",
                description: "",
                skill: SkillTypes::Mining,
                required_xp: 50.0,
                icon: IconType::Mining.into(),
                collect: Collect::Item(collection_type.into()),
                max_completions: None,
                automate_limit: 80,
            },
            F2Collects::Poison => WCollection {
                name: wrapped_type,
                display_name: "Poison",
                description: "",
                skill: SkillTypes::Alchemy,
                required_xp: 20.0,
                icon: IconType::Alchemy.into(),
                collect: Collect::Item(collection_type.into()),
                max_completions: None,
                automate_limit: 40,
            },
            F2Collects::Log => WCollection {
                name: wrapped_type,
                display_name: "Log",
                description: "",
                skill: SkillTypes::Woodcutting,
                required_xp: 100.0,
                icon: IconType::Woodcutting.into(),
                collect: Collect::Item(collection_type.into()),
                max_completions: None,
                automate_limit: 40,
            },
            F2Collects::Fur => WCollection {
                name: wrapped_type,
                display_name: "Fur",
                description: "",
                skill: SkillTypes::Hunting,
                required_xp: 150.0,
                icon: IconType::Hunting.into(),
                collect: Collect::Item(collection_type.into()),
                max_completions: None,
                automate_limit: 40,
            },
        };
        collections.push(collection);
    }
    collections
}

pub fn get_third_floor_collections() -> Vec<WCollection> {
    let mut collections = Vec::new();
    for collection_type in F3Collects::iter() {
        let wrapped_type = collection_type.into();
        let collection = match collection_type {
            F3Collects::Test => WCollection {
                name: wrapped_type,
                display_name: "Test",
                description: "Hunting down rats might be useful",
                skill: SkillTypes::Hunting,
                required_xp: 20.0,
                icon: IconType::Hunting.into(),
                collect: Collect::Mana(3.0),
                max_completions: Some(20),
                automate_limit: 80,
            },
        };
        collections.push(collection);
    }
    collections
}

fn has_explored(exploration: AllExplors, game: &Game) -> bool {
    game.state.get_exploration(exploration).is_completed
}

pub fn should_be_visible_collection(collection_type: AllCollects, game: &Game) -> bool {
    // let wcrafting = game.world.get_wcrafting(crafting_type);
    match collection_type {
        AllCollects::First(First) => match First {
            F1Collects::MagicDust => {
                has_explored(F1Explors::Hallway.into(), game) && !has_explored(F1Explors::BlockedDoor.into(), game)
            }
            F1Collects::MetalScrap => has_explored(F1Explors::TowerEntrance.into(), game),
            F1Collects::Rock => {
                has_explored(F1Explors::SideArea.into(), game) && !has_explored(F1Explors::BlockedDoor.into(), game)
            }
            F1Collects::Wood => {
                has_explored(F1Explors::SideArea.into(), game) && !has_explored(F1Explors::BlockedDoor.into(), game)
            }

            F1Collects::CrystalFragments => has_explored(F1Explors::BlockedDoor.into(), game),
            F1Collects::Crystal => has_explored(F1Explors::Laboratory.into(), game),
        },
        AllCollects::Second(Second) => match Second {
            F2Collects::HuntRat => {
                has_explored(F2Explors::WideHallway.into(), game) && !has_explored(F2Explors::Intersection.into(), game)
            }
            F2Collects::Stick => {
                has_explored(F2Explors::ExploreHallway.into(), game)
                    && !has_explored(F2Explors::Intersection.into(), game)
            }
            F2Collects::ExtractCrystal => {
                has_explored(F2Explors::FightWolf.into(), game) && !has_explored(F2Explors::Intersection.into(), game)
            }
            F2Collects::Poison => {
                has_explored(F2Explors::Laboratory.into(), game) && !has_explored(F2Explors::Intersection.into(), game)
            }
            F2Collects::HuntRabbits => {
                has_explored(F2Explors::Jump.into(), game) && !has_explored(F2Explors::RabbitKing.into(), game)
            }
            F2Collects::Flint => {
                has_explored(F2Explors::RemoveDebris.into(), game)
                    && !has_explored(F2Explors::DownWithTrees.into(), game)
            }
            F2Collects::Log => has_explored(F2Explors::Clearing.into(), game),
            F2Collects::Fur => has_explored(F2Explors::Argue.into(), game),
        },
        AllCollects::Third(Third) => match Third {
            F3Collects::Test => false,
        },
    }
}

pub fn should_be_automatable_collection(collection_type: AllCollects, game: &Game) -> bool {
    if game.state.status.override_automatable {
        return true;
    }
    let wcollection = game.world.get_wcollection(collection_type);
    let collection = game.state.get_collection(collection_type);
    collection.completion_count >= wcollection.automate_limit
}

pub fn can_collect(collection_type: AllCollects, game: &Game) -> bool {
    let collection = game.state.get_collection(collection_type);
    let wcollection = game.world.get_wcollection(collection_type);
    if !collection.is_visible {
        return false;
    }
    if let Some(max_completions) = wcollection.max_completions {
        if collection.round_completions >= max_completions {
            return false;
        }
    }
    match wcollection.collect {
        Collect::Item(item) => !game.state.get_item(item).is_maxed(),
        Collect::Mana(_) => true,
    }
}
