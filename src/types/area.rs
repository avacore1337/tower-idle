use serde::{Deserialize, Serialize};
use std::mem::variant_count;
use strum::{EnumIter, IntoEnumIterator};
use tsify::Tsify;

#[derive(
    Tsify, Serialize, Deserialize, EnumIter, Default, Clone, Copy, Debug, PartialEq, PartialOrd,
)]

pub enum F1Areas {
    #[default]
    Outside,
    TowerEntrance,
    Hallway,
    BrokenHandle,
    MassiveRoom,
    SideDoor,
    SideArea,
    BlockedDoor,
    Laboratory,
    Shrine,
}

#[derive(
    Tsify, Serialize, Deserialize, EnumIter, Default, Clone, Copy, Debug, PartialEq, PartialOrd,
)]
pub enum F2Areas {
    #[default]
    StairWell,
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
}

#[derive(
    Tsify, Serialize, Deserialize, EnumIter, Default, Clone, Copy, Debug, PartialEq, PartialOrd,
)]
pub enum F3Areas {
    #[default]
    Test,
}

#[derive(Tsify, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum AllAreas {
    First(F1Areas),
    Second(F2Areas),
    Third(F3Areas),
}

impl Default for AllAreas {
    fn default() -> Self {
        AllAreas::First(F1Areas::default())
    }
}

impl AllAreas {
    pub fn to_area_index(&self) -> usize {
        match self {
            AllAreas::First(x) => *x as usize,
            AllAreas::Second(x) => *x as usize,
            AllAreas::Third(x) => *x as usize,
        }
    }

    pub fn iter() -> impl Iterator<Item = AllAreas> {
        // AllAreasIterator { current: None }
        F1Areas::iter()
            .map(|x| AllAreas::First(x))
            .chain(F2Areas::iter().map(|x| AllAreas::Second(x)))
            .chain(F3Areas::iter().map(|x| AllAreas::Third(x)))
    }
}

pub const FIRST_AREAS_SIZE: usize = variant_count::<F1Areas>();
pub const SECOND_AREAS_SIZE: usize = variant_count::<F2Areas>();
pub const THIRD_AREAS_SIZE: usize = variant_count::<F3Areas>();
pub const ALL_AREAS_SIZE: usize = variant_count::<AllAreas>();

impl From<F1Areas> for AllAreas {
    fn from(e: F1Areas) -> Self {
        AllAreas::First(e)
    }
}

impl From<F2Areas> for AllAreas {
    fn from(e: F2Areas) -> Self {
        AllAreas::Second(e)
    }
}

impl From<F3Areas> for AllAreas {
    fn from(e: F3Areas) -> Self {
        AllAreas::Third(e)
    }
}
