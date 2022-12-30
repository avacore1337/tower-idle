use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use strum::EnumIter;
use strum::IntoEnumIterator;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Tsify, Serialize, Debug, Clone)]
pub struct Icon {
    pub name: &'static str,
}

impl Icon {
    pub fn new(name: &'static str) -> Icon {
        Icon { name }
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum IconType {
    Farming,
    Woodcutting,
    Mining,
    Crafting,
    Agility,
    Fishing,
    Cooking,
    Hunting,
    Fighting,
    Alchemy,
    Conversation,
    Play,
    Collection,
    Exploration,
    Schedule,
    Clock,
    Health,
    HealthDrain,
    Priority0,
    Priority1,
    Priority2,
    Priority3,
    Priority4,
    Plus,
    Minus,
    Stop,
    Trash,
    Check,
    Lock,
    Infinity,
    Settings,
    Automation,
    History,
    Save,
    // Load,
    Download,
    Export,
    Import,
    Tombstone,
}

impl From<IconType> for Icon {
    fn from(icon: IconType) -> Self {
        Icon {
            name: get_icon_name(icon),
        }
    }
}

impl From<Icon> for IconType {
    fn from(icon: Icon) -> Self {
        REV_ICON_MAP[icon.name]
    }
}

lazy_static! {
    static ref REV_ICON_MAP: BTreeMap<&'static str, IconType> = make_rev_icon_map();
}

fn make_rev_icon_map() -> BTreeMap<&'static str, IconType> {
    let mut icons = BTreeMap::new();

    for icon in IconType::iter() {
        icons.insert(get_icon_name(icon), icon.into());
    }
    icons
}

pub fn get_icon_name(icon: IconType) -> &'static str {
    match icon {
        IconType::Farming => "wheat-awn",
        IconType::Woodcutting => "axe",
        IconType::Mining => "pickaxe",
        IconType::Crafting => "screwdriver-wrench",
        IconType::Agility => "running",
        IconType::Fishing => "fish",
        IconType::Cooking => "utensils",
        IconType::Hunting => "bow-arrow",
        IconType::Fighting => "swords",
        IconType::Alchemy => "flask-round-potion",
        IconType::Conversation => "comments",
        IconType::Play => "play",
        IconType::Collection => "toolbox",
        IconType::Exploration => "map-location-dot",
        IconType::Schedule => "calendar-plus",
        IconType::Clock => "clock",
        IconType::Health => "heart",
        IconType::HealthDrain => "heart-crack",
        IconType::Priority0 => "gauge-min",
        IconType::Priority1 => "gauge-low",
        IconType::Priority2 => "gauge",
        IconType::Priority3 => "gauge-high",
        IconType::Priority4 => "gauge-max",
        IconType::Plus => "plus",
        IconType::Minus => "minus",
        IconType::Stop => "circle-xmark",
        IconType::Check => "check",
        IconType::Lock => "lock",
        IconType::Trash => "trash",
        IconType::Infinity => "infinity",
        IconType::Settings => "gear",
        IconType::Automation => "gauge-circle-plus",
        IconType::History => "calendar-days",
        IconType::Save => "floppy-disk",
        // IconType::Load => "gear",
        IconType::Download => "download",
        IconType::Export => "file-export",
        IconType::Import => "file-import",
        // IconType::Int => "brain",
        // IconType::Str => "dumbbell",
        // IconType::Cha => "glass-cheers",
        // IconType::Faith => "church",
        // IconType::Mindfull => "hand-holding",
        // IconType::Tactics => "map",
        // IconType::Coin => "coins",
        // IconType::Money => "money-bill",
        // IconType::DivineFavor => "bible",
        // IconType::Happiness => "laugh-beam",
        // IconType::Health => "medkit",
        // IconType::Question => "question",
        // IconType::Pause => "pause",
        // IconType::Death => "skull-crossbones",
        IconType::Tombstone => "tombstone", // should be a tombstone
    }
}

pub fn get_icons() -> BTreeMap<String, Icon> {
    let mut icons = BTreeMap::new();
    for icon in IconType::iter() {
        icons.insert(format!("{:?}", icon), icon.into());
    }
    icons
}
