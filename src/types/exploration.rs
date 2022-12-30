use super::{BasePriority, Prio, Recordable};
use crate::world::exploration::can_explore;
use crate::{action_queue::ActionEntry, game::Game};
use serde::{Deserialize, Serialize};
use std::mem::variant_count;
use strum::{EnumIter, IntoEnumIterator};
use tsify::Tsify;

#[derive(
    Tsify, Serialize, Deserialize, EnumIter, Default, Clone, Copy, Debug, PartialEq, PartialOrd,
)]

pub enum F1Explors {
    #[default]
    TowerEntrance,
    Hallway,
    BrokenHandle,
    MassiveRoom,
    SideDoor,
    SideArea,
    BackToMassive,
    BlockedDoor,
    Laboratory,
    Shrine,
    Stairs,
}

#[derive(
    Tsify, Serialize, Deserialize, EnumIter, Default, Clone, Copy, Debug, PartialEq, PartialOrd,
)]
pub enum F2Explors {
    #[default]
    WideHallway,
    ExploreHallway,
    FightWolf,
    Laboratory,
    Intersection,
    Crawl,
    Jump,
    RabbitKing,
    RabbitHorde,
    ChopForward,
    RemoveDebris,
    DownWithTrees,
    BalanceAlong,
    HitTheLever,
    Clearing,
    BridgeGap,
    MeetTheTrader,
    KillTrader,
    Negotiate,
    Argue,
    Haggle,
    UnlockStairs,
    Stairs2,
}

#[derive(
    Tsify, Serialize, Deserialize, EnumIter, Default, Clone, Copy, Debug, PartialEq, PartialOrd,
)]
pub enum F3Explors {
    #[default]
    Test,
}

#[derive(Tsify, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum AllExplors {
    First(F1Explors),
    Second(F2Explors),
    Third(F3Explors),
}

impl Default for AllExplors {
    fn default() -> Self {
        AllExplors::First(F1Explors::default())
    }
}

impl AllExplors {
    pub fn to_exploration_index(&self) -> usize {
        match self {
            AllExplors::First(x) => *x as usize,
            AllExplors::Second(x) => *x as usize,
            AllExplors::Third(x) => *x as usize,
        }
    }

    pub fn iter() -> impl Iterator<Item = AllExplors> {
        // AllExplorsIterator { current: None }
        F1Explors::iter()
            .map(|x| AllExplors::First(x))
            .chain(F2Explors::iter().map(|x| AllExplors::Second(x)))
            .chain(F3Explors::iter().map(|x| AllExplors::Third(x)))
    }
}

pub const FIRST_EXPLORATIONS_SIZE: usize = variant_count::<F1Explors>();
pub const SECOND_EXPLORATIONS_SIZE: usize = variant_count::<F2Explors>();
pub const THIRD_EXPLORATIONS_SIZE: usize = variant_count::<F3Explors>();
pub const ALL_EXPLORATIONS_SIZE: usize = variant_count::<AllExplors>();

impl Prio for AllExplors {
    fn get_user_priority(self, game: &Game) -> u32 {
        game.state.get_exploration(self).priority
    }
    fn get_automatable(self, game: &Game) -> bool {
        game.state.get_exploration(self).is_automatable
    }
    fn get_base_priority(self, _game: &Game) -> BasePriority {
        BasePriority::Exploration
    }
    fn get_doable(self, game: &Game) -> bool {
        can_explore(self, game)
    }
    fn get_action_entry(self, game: &Game) -> ActionEntry {
        game.world.get_wexploration(self).to_action_entry(1)
    }
}

impl Recordable for AllExplors {
    fn to_record_key(&self) -> String {
        match self {
            AllExplors::First(x) => x.to_record_key(),
            AllExplors::Second(x) => x.to_record_key(),
            AllExplors::Third(x) => x.to_record_key(),
        }
    }
}

impl Recordable for F1Explors {
    fn to_record_key(&self) -> String {
        format!("F1-E {:#?}", self)
    }
}

impl Recordable for F2Explors {
    fn to_record_key(&self) -> String {
        format!("F2-E {:#?}", self)
    }
}

impl Recordable for F3Explors {
    fn to_record_key(&self) -> String {
        format!("F3-E {:#?}", self)
    }
}

impl From<F1Explors> for AllExplors {
    fn from(e: F1Explors) -> Self {
        AllExplors::First(e)
    }
}

impl From<F2Explors> for AllExplors {
    fn from(e: F2Explors) -> Self {
        AllExplors::Second(e)
    }
}

impl From<F3Explors> for AllExplors {
    fn from(e: F3Explors) -> Self {
        AllExplors::Third(e)
    }
}
