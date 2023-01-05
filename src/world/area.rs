use crate::types::*;
use crate::types::{AllExplors, F1Explors as E1, F2Explors as E2};
use serde::Serialize;
use strum::IntoEnumIterator;
use tsify::Tsify;

#[derive(Tsify, Serialize, Clone)]
pub struct WArea {
    pub name: AllAreas,
    pub new_explorations: Vec<AllExplors>,
    // pub display_name: &'static str,
    // pub description: &'static str,
    // pub story_line: &'static str,
    // Row column information used to generate the map graph
    // pub row: u32,
    // pub column: u32,
}

pub fn get_areas(floor: FloorTypes) -> Vec<WArea> {
    match floor {
        FloorTypes::Starting => get_first_floor_areas(),
        FloorTypes::Second => get_second_floor_areas(),
        FloorTypes::Third => get_third_floor_areas(),
    }
}

pub fn get_first_floor_areas() -> Vec<WArea> {
    let mut areas = Vec::new();
    for area_type in F1Areas::iter() {
        let wrapped_type = area_type.into();
        let area = match area_type {
            F1Areas::Outside => WArea {
                name: wrapped_type,
                new_explorations: vec![E1::TowerEntrance.into()],
            },
            F1Areas::TowerEntrance => WArea {
                name: wrapped_type,
                new_explorations: vec![E1::Hallway.into(), E1::BrokenHandle.into()],
            },
            F1Areas::BrokenHandle => WArea {
                name: wrapped_type,
                new_explorations: vec![E1::MassiveRoom.into()],
            },
            F1Areas::MassiveRoom => WArea {
                name: wrapped_type,
                new_explorations: vec![E1::SideDoor.into(), E1::BlockedDoor.into(), E1::Hallway.into()],
            },
            F1Areas::SideDoor => WArea {
                name: wrapped_type,
                new_explorations: vec![E1::SideArea.into()],
            },
            F1Areas::SideArea => WArea {
                name: wrapped_type,
                new_explorations: vec![E1::BackToMassive.into()],
            },
            F1Areas::BlockedDoor => WArea {
                name: wrapped_type,
                new_explorations: vec![E1::Laboratory.into()],
            },
            F1Areas::Laboratory => WArea {
                name: wrapped_type,
                new_explorations: vec![E1::Shrine.into()],
            },
            F1Areas::Shrine => WArea {
                name: wrapped_type,
                new_explorations: vec![E1::Stairs.into()],
            },
        };
        areas.push(area);
    }
    areas
}

pub fn get_second_floor_areas() -> Vec<WArea> {
    let mut areas = Vec::new();
    for area_type in F2Areas::iter() {
        let wrapped_type = area_type.into();
        let area = match area_type {
            F2Areas::StairWell => WArea {
                name: wrapped_type,
                new_explorations: vec![E2::WideHallway.into()],
            },
            F2Areas::WideHallway => WArea {
                name: wrapped_type,
                new_explorations: vec![E2::ExploreHallway.into()],
            },
            F2Areas::ExploreHallway => WArea {
                name: wrapped_type,
                new_explorations: vec![E2::FightWolf.into()],
            },
            F2Areas::FightWolf => WArea {
                name: wrapped_type,
                new_explorations: vec![E2::Laboratory.into()],
            },
            F2Areas::Laboratory => WArea {
                name: wrapped_type,
                new_explorations: vec![E2::Intersection.into()],
            },
            F2Areas::Intersection => WArea {
                name: wrapped_type,
                new_explorations: vec![E2::Crawl.into(), E2::ChopForward.into()],
            },
            F2Areas::Crawl => WArea {
                name: wrapped_type,
                new_explorations: vec![E2::Jump.into()],
            },
            F2Areas::Jump => WArea {
                name: wrapped_type,
                new_explorations: vec![E2::RabbitKing.into()],
            },
            F2Areas::RabbitKing => WArea {
                name: wrapped_type,
                new_explorations: vec![E2::RabbitHorde.into()],
            },
            F2Areas::RabbitHorde => WArea {
                name: wrapped_type,
                new_explorations: vec![E2::BalanceAlong.into()],
            },
            F2Areas::ChopForward => WArea {
                name: wrapped_type,
                new_explorations: vec![E2::RemoveDebris.into()],
            },
            F2Areas::RemoveDebris => WArea {
                name: wrapped_type,
                new_explorations: vec![E2::DownWithTrees.into()],
            },
            F2Areas::DownWithTrees => WArea {
                name: wrapped_type,
                new_explorations: vec![E2::BalanceAlong.into()],
            },
            F2Areas::BalanceAlong => WArea {
                name: wrapped_type,
                new_explorations: vec![E2::HitTheLever.into()],
            },
            F2Areas::HitTheLever => WArea {
                name: wrapped_type,
                new_explorations: vec![E2::Clearing.into()],
            },
            F2Areas::Clearing => WArea {
                name: wrapped_type,
                new_explorations: vec![E2::BridgeGap.into()],
            },
            F2Areas::BridgeGap => WArea {
                name: wrapped_type,
                new_explorations: vec![E2::MeetTheTrader.into()],
            },
            F2Areas::MeetTheTrader => WArea {
                name: wrapped_type,
                new_explorations: vec![E2::KillTrader.into(), E2::Negotiate.into()],
            },
            F2Areas::KillTrader => WArea {
                name: wrapped_type,
                new_explorations: vec![E2::UnlockStairs.into()],
            },
            F2Areas::Negotiate => WArea {
                name: wrapped_type,
                new_explorations: vec![E2::Argue.into()],
            },
            F2Areas::Argue => WArea {
                name: wrapped_type,
                new_explorations: vec![E2::Haggle.into()],
            },
            F2Areas::Haggle => WArea {
                name: wrapped_type,
                new_explorations: vec![E2::UnlockStairs.into()],
            },
            F2Areas::UnlockStairs => WArea {
                name: wrapped_type,
                new_explorations: vec![E2::Stairs2.into()],
            },
        };
        areas.push(area);
    }
    areas
}

pub fn get_third_floor_areas() -> Vec<WArea> {
    let mut areas = Vec::new();
    for area_type in F3Areas::iter() {
        let wrapped_type = area_type.into();
        let area = match area_type {
            F3Areas::Test => WArea {
                name: wrapped_type,
                new_explorations: vec![E2::WideHallway.into()],
            },
        };
        areas.push(area);
    }
    areas
}
