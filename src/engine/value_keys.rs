use crate::types::*;
use serde::{Deserialize, Serialize};
use strum::EnumIter;
// use std::mem::variant_count;

#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KeyValues {
    // StatusValues
    InventorySize,
    // StartingHealthBoost,
    // CurrentHealth,
    // MaxHealth,
    HealthDrain,
    //SkillTypes
    Farming,
    Woodcutting,
    Mining,
    Alchemy,
    Crafting,
    Agility,
    Fishing,
    Cooking,
    Hunting,
    Fighting,
    Conversation,
}

impl From<SkillTypes> for KeyValues {
    fn from(stat: SkillTypes) -> Self {
        match stat {
            SkillTypes::Farming => KeyValues::Farming,
            SkillTypes::Woodcutting => KeyValues::Woodcutting,
            SkillTypes::Mining => KeyValues::Mining,
            SkillTypes::Alchemy => KeyValues::Alchemy,
            SkillTypes::Crafting => KeyValues::Crafting,
            SkillTypes::Agility => KeyValues::Agility,
            SkillTypes::Fishing => KeyValues::Fishing,
            SkillTypes::Cooking => KeyValues::Cooking,
            SkillTypes::Hunting => KeyValues::Hunting,
            SkillTypes::Fighting => KeyValues::Fighting,
            SkillTypes::Conversation => KeyValues::Conversation,
        }
    }
}
