use super::{BasePriority, Prio, Recordable};
use crate::world::collection::can_collect;
use crate::{action_queue::ActionEntry, game::Game};
use serde::{Deserialize, Serialize};
use std::mem::variant_count;
use strum::{EnumIter, IntoEnumIterator};
use tsify::Tsify;

#[derive(
    Tsify, Serialize, Deserialize, EnumIter, Default, Clone, Copy, Debug, PartialEq, PartialOrd,
)]

pub enum F1Collects {
    #[default]
    MetalScrap,
    Rock,
    Wood,
    MagicDust,
    CrystalFragments,
    Crystal,
}

#[derive(
    Tsify, Serialize, Deserialize, EnumIter, Default, Clone, Copy, Debug, PartialEq, PartialOrd,
)]
pub enum F2Collects {
    #[default]
    HuntRat,
    Stick,
    ExtractCrystal,
    HuntRabbits,
    Flint,
    Log,
    Poison,
    TraderCollect,
}

#[derive(
    Tsify, Serialize, Deserialize, EnumIter, Default, Clone, Copy, Debug, PartialEq, PartialOrd,
)]
pub enum F3Collects {
    #[default]
    Test,
}

#[derive(Tsify, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum AllCollects {
    First(F1Collects),
    Second(F2Collects),
    Third(F3Collects),
}

impl AllCollects {
    pub fn to_collection_index(&self) -> usize {
        match self {
            AllCollects::First(x) => *x as usize,
            AllCollects::Second(x) => *x as usize,
            AllCollects::Third(x) => *x as usize,
        }
    }

    pub fn iter() -> impl Iterator<Item = AllCollects> {
        F1Collects::iter()
            .map(|x| AllCollects::First(x))
            .chain(F2Collects::iter().map(|x| AllCollects::Second(x)))
            .chain(F3Collects::iter().map(|x| AllCollects::Third(x)))
    }
}

pub const FIRST_COLLECTIONS_SIZE: usize = variant_count::<F1Collects>();
pub const SECOND_COLLECTIONS_SIZE: usize = variant_count::<F2Collects>();
pub const THIRD_COLLECTIONS_SIZE: usize = variant_count::<F3Collects>();
pub const ALL_COLLECTIONS_SIZE: usize = variant_count::<AllCollects>();

impl Prio for AllCollects {
    fn get_user_priority(self, game: &Game) -> u32 {
        game.state.get_collection(self).priority
    }
    fn get_automatable(self, game: &Game) -> bool {
        game.state.get_collection(self).is_automatable
    }
    fn get_base_priority(self, game: &Game) -> BasePriority {
        game.world.get_wcollection(self).base_priority
    }
    fn get_doable(self, game: &Game) -> bool {
        can_collect(self, game)
    }
    fn get_action_entry(self, game: &Game) -> ActionEntry {
        game.world.get_wcollection(self).to_action_entry(1)
    }
}

impl Recordable for AllCollects {
    fn to_record_key(&self) -> String {
        match self {
            AllCollects::First(x) => x.to_record_key(),
            AllCollects::Second(x) => x.to_record_key(),
            AllCollects::Third(x) => x.to_record_key(),
        }
    }
}

impl Recordable for F1Collects {
    fn to_record_key(&self) -> String {
        format!("F1-C {:#?}", self)
    }
}

impl Recordable for F2Collects {
    fn to_record_key(&self) -> String {
        format!("F2-C {:#?}", self)
    }
}

impl Recordable for F3Collects {
    fn to_record_key(&self) -> String {
        format!("F3-C {:#?}", self)
    }
}

impl From<F1Collects> for AllCollects {
    fn from(e: F1Collects) -> Self {
        AllCollects::First(e)
    }
}

impl From<F2Collects> for AllCollects {
    fn from(e: F2Collects) -> Self {
        AllCollects::Second(e)
    }
}

impl From<F3Collects> for AllCollects {
    fn from(e: F3Collects) -> Self {
        AllCollects::Third(e)
    }
}
