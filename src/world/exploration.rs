use crate::action_queue::ActionEntry;
use crate::game::Game;
use crate::icon::{Icon, IconType};
use crate::types::*;
use serde::Serialize;
use strum::IntoEnumIterator;
use tsify::Tsify;

#[derive(Tsify, Serialize, Clone)]
pub struct WExploration {
    pub name: AllExplors,
    pub display_name: &'static str,
    pub description: &'static str,
    pub story_line: &'static str,
    pub skill: SkillTypes,
    pub required_xp: f64,
    pub dps: f64,
    pub is_reenterable: bool,
    pub target_area: AllAreas,
    pub icon: Icon,
    pub automate_limit: u32,
}

impl WExploration {
    pub fn on_completed(&self, game: &mut Game) {
        game.state.current_area = self.target_area;
        if self.story_line != "" {
            game.state.messages.push(self.story_line.to_string())
        }
        match self.name {
            AllExplors::First(F1Explors::Stairs) => {
                game.state.set_current_floor(FloorTypes::Second);
            }
            AllExplors::Second(F2Explors::Stairs2) => {
                game.state.set_current_floor(FloorTypes::Third);
            }
            _ => (),
        }
    }

    pub fn to_action_entry(&self, amount: u32) -> ActionEntry {
        ActionEntry {
            action_key: self.name.to_record_key(),
            display_name: self.display_name.to_string(),
            amount,
            skill_icon: self.icon.clone().into(),
            category_icon: IconType::Exploration,
        }
    }
}

pub fn get_explorations(floor: FloorTypes) -> Vec<WExploration> {
    match floor {
        FloorTypes::Starting => get_first_floor_explorations(),
        FloorTypes::Second => get_second_floor_explorations(),
        FloorTypes::Third => get_third_floor_explorations(),
    }
}

pub fn get_first_floor_explorations() -> Vec<WExploration> {
    let mut explorations = Vec::new();
    for exploration_type in F1Explors::iter() {
        let wrapped_type = exploration_type.into();
        let exploration = match exploration_type {
            F1Explors::TowerEntrance => WExploration {
                name: wrapped_type,
                display_name: "Tower entrance",
                description: "The door to the tower looms in front of you",
                story_line: "As you open the doors you see a blinding light.\n\
                    You take a step back but then brace yourself and then step into the light.\n\
                    As the doors close behind you are thrown into darkness you have one singular thought in your mind.\n\
                    Reach the top of the tower.",
                skill: SkillTypes::Agility,
                required_xp: 10.0,
                dps: 0.0,
                is_reenterable: false,
                icon: IconType::Agility.into(),
                target_area: F1Areas::TowerEntrance.into(),
                automate_limit: 4,
            },
            F1Explors::Hallway => WExploration {
                name: wrapped_type,
                display_name: "Hallway",
                description: "A long and dark hallway",
                story_line: "You slowly explore the hallway, expecting something to jump out at you at any moment",
                skill: SkillTypes::Agility,
                required_xp: 20.0,
                dps: 0.0,
                is_reenterable: false,
                icon: IconType::Agility.into(),
                target_area: F1Areas::Hallway.into(),
                automate_limit: 4,
            },
            F1Explors::BrokenHandle => WExploration {
                name: wrapped_type,
                display_name: "Open door",
                description: "An old rusty door, the handle is broken, but maybe you can fix it?",
                story_line: "You fiddle with the handle and after a little bit you manage to find a small piece of metal that you open the door with.",
                skill: SkillTypes::Crafting,
                required_xp: 20.0,
                dps: 0.0,
                is_reenterable: false,
                icon: IconType::Crafting.into(),
                target_area: F1Areas::BrokenHandle.into(),
                automate_limit: 4,
            },
            F1Explors::MassiveRoom => WExploration {
                name: wrapped_type,
                display_name: "Massive room",
                description: "A massive room with candles along the wall, light it up a bit.",
                story_line: "You light up the candles and see two doors. One looks like a side door and the other looks to go deeper into the tower.",
                skill: SkillTypes::Agility,
                required_xp: 30.0,
                dps: 0.0,
                is_reenterable: true,
                icon: IconType::Agility.into(),
                target_area: F1Areas::MassiveRoom.into(),
                automate_limit: 4,
            },
            F1Explors::SideDoor => WExploration {
                name: wrapped_type,
                display_name: "Side door",
                description: "A small door to the side, maybe worth exploring",
                story_line: "The door was booby trapped, but the trap had rusted and didn't go off. You carefully squize through the door.",
                skill: SkillTypes::Agility,
                required_xp: 40.0,
                dps: 0.0,
                is_reenterable: false,
                icon: IconType::Agility.into(),
                target_area: F1Areas::SideDoor.into(),
                automate_limit: 4,
            },
            F1Explors::SideArea => WExploration {
                name: wrapped_type,
                display_name: "Side area",
                description: "TODO",
                story_line: "",
                skill: SkillTypes::Agility,
                required_xp: 40.0,
                dps: 0.0,
                is_reenterable: false,
                icon: IconType::Agility.into(),
                target_area: F1Areas::SideArea.into(),
                automate_limit: 4,
            },
            F1Explors::BackToMassive => WExploration {
                name: wrapped_type,
                display_name: "Back to massive room",
                description: "A massive room with candles along the wall, light it up a bit.",
                story_line: "You light up the candles and see two doors. One looks like a side door and the other looks to go deeper into the tower.",
                skill: SkillTypes::Agility,
                required_xp: 30.0,
                dps: 0.0,
                is_reenterable: true,
                icon: IconType::Agility.into(),
                target_area: F1Areas::MassiveRoom.into(),
                automate_limit: 4,
            },
            F1Explors::BlockedDoor => WExploration {
                name: wrapped_type,
                display_name: "Blocked door",
                description:
                    "The door is blocked, but given enough time you can probably break it down",
                story_line: "",
                skill: SkillTypes::Woodcutting,
                required_xp: 700.0,
                dps: 0.0,
                is_reenterable: false,
                icon: IconType::Agility.into(),
                target_area: F1Areas::BlockedDoor.into(),
                automate_limit: 4,
            },
            F1Explors::Laboratory => WExploration {
                name: wrapped_type,
                display_name: "Clear laboratory",
                description: "It looks like an old laboratory, but it's filled with large rocks. You need to clear a path here.",
                story_line: "While clearing out the laboratory you see that some of the rocks seems to be ordinary things that has been turned into stone. How strange.",
                skill: SkillTypes::Mining,
                required_xp: 100.0,
                dps: 0.0,
                is_reenterable: false,
                icon: IconType::Agility.into(),
                target_area: F1Areas::Laboratory.into(),
                automate_limit: 4,
            },
            F1Explors::Shrine => WExploration {
                name: wrapped_type,
                display_name: "Shrine",
                description: "There seems to be a shrine up front, but the path is filled with a mystery liquid. Maybe you can turn it into stone?",
                story_line: "After looking at some old notes you found and experimenting a bit you manage to make something. Unfortunatly you only have enough for one dose.",
                skill: SkillTypes::Alchemy,
                required_xp: 400.0,
                dps: 0.0,
                is_reenterable: false,
                icon: IconType::Alchemy.into(),
                target_area: F1Areas::Shrine.into(),
                automate_limit: 4,
            },
            F1Explors::Stairs => WExploration {
                name: wrapped_type,
                display_name: "Stairs",
                description: "A flight of stairs to the next level. There is no turning back.",
                story_line: "As you walk up the stairs you encounter a shimmering wall with a sign next to it that says \
                    \"Only equipment and Stones allowed past this point. All forbidden items will be removed by the barrier\".\
                    Well, you have to continue, so you push on and take a look to see if you lost anything.",
                skill: SkillTypes::Agility,
                required_xp: 50.0,
                dps: 0.0,
                is_reenterable: false,
                icon: IconType::Agility.into(),
                target_area: F2Areas::StairWell.into(),
                automate_limit: 4,
            },
        };
        explorations.push(exploration);
    }
    explorations
}

pub fn get_second_floor_explorations() -> Vec<WExploration> {
    let mut explorations = Vec::new();
    for exploration_type in F2Explors::iter() {
        let wrapped_type = exploration_type.into();
        let exploration = match exploration_type {
            F2Explors::WideHallway => WExploration {
                name: wrapped_type,
                display_name: "Explore Enterance",
                description: "A wide hallway, you hear squeaks further ahead",
                story_line: "",
                skill: SkillTypes::Agility,
                required_xp: 50.0,
                dps: 0.0,
                is_reenterable: false,
                icon: IconType::Agility.into(),
                target_area: F2Areas::WideHallway.into(),
                automate_limit: 4,
            },
            F2Explors::ExploreHallway => WExploration {
                name: wrapped_type,
                display_name: "Sneak onwards",
                description:
                    "After encountering the rats you feel that you need to be more carefull",
                story_line: "",
                skill: SkillTypes::Agility,
                required_xp: 50.0,
                dps: 0.0,
                is_reenterable: false,
                icon: IconType::Agility.into(),
                target_area: F2Areas::ExploreHallway.into(),
                automate_limit: 4,
            },
            F2Explors::FightWolf => WExploration {
                name: wrapped_type,
                display_name: "Fight Wolf",
                description: "It looks scary and intimdating. Is this the end?",
                story_line: "",
                skill: SkillTypes::Fighting,
                required_xp: 20.0,
                dps: 0.5,
                is_reenterable: false,
                icon: IconType::Fighting.into(),
                target_area: F2Areas::FightWolf.into(),
                automate_limit: 4,
            },
            F2Explors::Laboratory => WExploration {
                name: wrapped_type,
                display_name: "Laboratory",
                description: "Another laboratory, what is this place really?",
                story_line: "",
                skill: SkillTypes::Alchemy,
                required_xp: 100.0,
                dps: 0.0,
                is_reenterable: false,
                icon: IconType::Alchemy.into(),
                target_area: F2Areas::Laboratory.into(),
                automate_limit: 4,
            },
            F2Explors::Intersection => WExploration {
                name: wrapped_type,
                display_name: "Intersection",
                description: "There is a sign saying \"No return after this point\"",
                story_line: "",
                skill: SkillTypes::Agility,
                required_xp: 50.0,
                dps: 0.0,
                is_reenterable: false,
                icon: IconType::Agility.into(),
                target_area: F2Areas::Intersection.into(),
                automate_limit: 4,
            },
            F2Explors::Crawl => WExploration {
                name: wrapped_type,
                display_name: "Crawl",
                description: "TODO",
                story_line: "",
                skill: SkillTypes::Agility,
                required_xp: 200.0,
                dps: 0.0,
                is_reenterable: false,
                icon: IconType::Agility.into(),
                target_area: F2Areas::Crawl.into(),
                automate_limit: 4,
            },
            F2Explors::Jump => WExploration {
                name: wrapped_type,
                display_name: "Jump",
                description: "You will have to jump, it will hurt.\nYou will take 30 damage",
                story_line: "",
                skill: SkillTypes::Agility,
                required_xp: 400.0,
                dps: 0.0,
                is_reenterable: false,
                icon: IconType::Agility.into(),
                target_area: F2Areas::Jump.into(),
                automate_limit: 4,
            },
            F2Explors::RabbitKing => WExploration {
                name: wrapped_type,
                display_name: "Fight rabbit king",
                description: "This rabbit is surrounded by the corpses of holy knights. A small golden ball with a cross on it lies at it's feet",
                story_line: "",
                skill: SkillTypes::Fighting,
                required_xp: 40.0,
                dps: 1.0,
                is_reenterable: false,
                icon: IconType::Fighting.into(),
                target_area: F2Areas::RabbitKing.into(),
                automate_limit: 4,
            },
            F2Explors::RabbitHorde => WExploration {
                name: wrapped_type,
                display_name: "Flee the rabbit horde",
                description: "There is a swarm of rabbits approaching. Run!",
                story_line: "",
                skill: SkillTypes::Agility,
                required_xp: 600.0,
                dps: 0.0,
                is_reenterable: false,
                icon: IconType::Agility.into(),
                target_area: F2Areas::RabbitHorde.into(),
                automate_limit: 4,
            },
            F2Explors::ChopForward => WExploration {
                name: wrapped_type,
                display_name: "Chop forward",
                description: "TODO",
                story_line: "",
                skill: SkillTypes::Woodcutting,
                required_xp: 600.0,
                dps: 0.0,
                is_reenterable: false,
                icon: IconType::Woodcutting.into(),
                target_area: F2Areas::ChopForward.into(),
                automate_limit: 4,
            },
            F2Explors::RemoveDebris => WExploration {
                name: wrapped_type,
                display_name: "Remove debris",
                description: "You need to swing your trusty pickaxe more than once to get past here",
                story_line: "",
                skill: SkillTypes::Mining,
                required_xp: 400.0,
                dps: 0.0,
                is_reenterable: false,
                icon: IconType::Mining.into(),
                target_area: F2Areas::RemoveDebris.into(),
                automate_limit: 4,
            },
            F2Explors::DownWithTrees => WExploration {
                name: wrapped_type,
                display_name: "Keep chopping trees",
                description: "Now you encounter even more trees. Will this never end?",
                story_line: "",
                skill: SkillTypes::Woodcutting,
                required_xp: 800.0,
                dps: 0.0,
                is_reenterable: false,
                icon: IconType::Woodcutting.into(),
                target_area: F2Areas::DownWithTrees.into(),
                automate_limit: 4,
            },
            F2Explors::BalanceAlong => WExploration {
                name: wrapped_type,
                display_name: "Balance along edge",
                description: "You better not mess up or you will die",
                story_line: "",
                skill: SkillTypes::Agility,
                required_xp: 800.0,
                dps: 0.0,
                is_reenterable: false,
                icon: IconType::Agility.into(),
                target_area: F2Areas::BalanceAlong.into(),
                automate_limit: 4,
            },
            F2Explors::HitTheLever => WExploration {
                name: wrapped_type,
                display_name: "Hit the lever",
                description: "",
                story_line: "",
                skill: SkillTypes::Agility,
                required_xp: 80.0,
                dps: 0.0,
                is_reenterable: false,
                icon: IconType::Agility.into(),
                target_area: F2Areas::HitTheLever.into(),
                automate_limit: 4,
            },
            F2Explors::Clearing => WExploration {
                name: wrapped_type,
                display_name: "Clearing",
                description: "",
                story_line: "",
                skill: SkillTypes::Agility,
                required_xp: 80.0,
                dps: 0.0,
                is_reenterable: false,
                icon: IconType::Agility.into(),
                target_area: F2Areas::Clearing.into(),
                automate_limit: 4,
            },
            F2Explors::BridgeGap => WExploration {
                name: wrapped_type,
                display_name: "Bridge the gap",
                description: "",
                story_line: "",
                skill: SkillTypes::Agility,
                required_xp: 80.0,
                dps: 0.0,
                is_reenterable: false,
                icon: IconType::Agility.into(),
                target_area: F2Areas::BridgeGap.into(),
                automate_limit: 4,
            },
            F2Explors::MeetTheTrader => WExploration {
                name: wrapped_type,
                display_name: "Meet the trader",
                description: "",
                story_line: "",
                skill: SkillTypes::Conversation,
                required_xp: 80.0,
                dps: 0.0,
                is_reenterable: false,
                icon: IconType::Agility.into(),
                target_area: F2Areas::MeetTheTrader.into(),
                automate_limit: 4,
            },
            F2Explors::KillTrader => WExploration {
                name: wrapped_type,
                display_name: "Kill the trader",
                description: "",
                story_line: "",
                skill: SkillTypes::Agility,
                required_xp: 80.0,
                dps: 0.0,
                is_reenterable: false,
                icon: IconType::Agility.into(),
                target_area: F2Areas::KillTrader.into(),
                automate_limit: 4,
            },
            F2Explors::Negotiate => WExploration {
                name: wrapped_type,
                display_name: "Negotiate",
                description: "",
                story_line: "",
                skill: SkillTypes::Conversation,
                required_xp: 80.0,
                dps: 0.0,
                is_reenterable: false,
                icon: IconType::Agility.into(),
                target_area: F2Areas::Negotiate.into(),
                automate_limit: 4,
            },
            F2Explors::Argue => WExploration {
                name: wrapped_type,
                display_name: "Argue",
                description: "",
                story_line: "",
                skill: SkillTypes::Conversation,
                required_xp: 80.0,
                dps: 0.0,
                is_reenterable: false,
                icon: IconType::Agility.into(),
                target_area: F2Areas::Argue.into(),
                automate_limit: 4,
            },
            F2Explors::Haggle => WExploration {
                name: wrapped_type,
                display_name: "Haggle",
                description: "",
                story_line: "",
                skill: SkillTypes::Conversation,
                required_xp: 80.0,
                dps: 0.0,
                is_reenterable: false,
                icon: IconType::Agility.into(),
                target_area: F2Areas::Haggle.into(),
                automate_limit: 4,
            },
            F2Explors::UnlockStairs => WExploration {
                name: wrapped_type,
                display_name: "Unlock stairs",
                description: "",
                story_line: "",
                skill: SkillTypes::Agility,
                required_xp: 80.0,
                dps: 0.0,
                is_reenterable: false,
                icon: IconType::Agility.into(),
                target_area: F2Areas::UnlockStairs.into(),
                automate_limit: 4,
            },
            F2Explors::Stairs2 => WExploration {
                name: wrapped_type,
                display_name: "Floor3",
                description: "",
                story_line: "",
                skill: SkillTypes::Agility,
                required_xp: 80.0,
                dps: 0.0,
                is_reenterable: false,
                icon: IconType::Agility.into(),
                target_area: F3Areas::Test.into(),
                automate_limit: 4,
            },
        };
        explorations.push(exploration);
    }
    explorations
}

pub fn get_third_floor_explorations() -> Vec<WExploration> {
    let mut explorations = Vec::new();
    for exploration_type in F3Explors::iter() {
        let wrapped_type = exploration_type.into();
        let exploration = match exploration_type {
            F3Explors::Test => WExploration {
                name: wrapped_type,
                display_name: "Explore Enterance",
                description: "A wide hallway, you hear squeaks further ahead",
                story_line: "",
                skill: SkillTypes::Agility,
                required_xp: 50.0,
                dps: 0.0,
                is_reenterable: false,
                icon: IconType::Agility.into(),
                target_area: F2Areas::WideHallway.into(),
                automate_limit: 4,
            },
        };
        explorations.push(exploration);
    }
    explorations
}

pub fn should_be_automatable_exploration(exploration_type: AllExplors, game: &Game) -> bool {
    let exploration = game.state.get_exploration(exploration_type);
    let wexploration = game.world.get_wexploration(exploration_type);
    exploration.completion_count >= wexploration.automate_limit
}

pub fn should_be_visible_exploration(exploration_type: AllExplors, game: &Game) -> bool {
    let wexploration = game.world.get_wexploration(exploration_type);
    let exploration = game.state.get_exploration(exploration_type);
    if exploration.is_completed && !wexploration.is_reenterable {
        return false;
    }
    if !game
        .world
        .get_wareas(game.state.current_area)
        .new_explorations
        .contains(&exploration_type)
    {
        return false;
    }
    // Add extra requirements here
    match exploration_type {
        AllExplors::First(first) => match first {
            F1Explors::BrokenHandle => {
                game.state
                    .get_crafting(F1Crafts::DoorHandle.into())
                    .is_completed
            }
            _ => true,
        },
        AllExplors::Second(second) => match second {
            F2Explors::BridgeGap => {
                game.state
                    .get_crafting(F2Crafts::Bridge.into())
                    .is_completed
            }
            _ => true,
        },
        AllExplors::Third(third) => match third {
            _ => true,
        },
    }
}

pub fn can_explore(exploration_type: AllExplors, game: &Game) -> bool {
    let exploration = game.state.get_exploration(exploration_type);
    exploration.is_visible
}
