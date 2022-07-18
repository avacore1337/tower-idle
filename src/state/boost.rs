use crate::types::*;
use serde::{Deserialize, Serialize};
use std::mem::{self, MaybeUninit};
use strum::IntoEnumIterator;
use tsify::Tsify;

#[derive(Tsify, Serialize, Deserialize, Clone, Debug)]
pub struct Boost {
    pub name: BoostTypes,
    pub unlocked: bool,
}

impl Boost {
    pub fn new(boost: BoostTypes) -> Boost {
        Boost {
            name: boost,
            unlocked: false,
        }
    }
}

pub fn get_boosts() -> [Boost; BOOST_SIZE] {
    let mut boosts: [MaybeUninit<Boost>; BOOST_SIZE] =
        unsafe { MaybeUninit::uninit().assume_init() };
    for name in BoostTypes::iter() {
        boosts[name as usize].write(Boost::new(name));
    }
    unsafe { mem::transmute(boosts) }
}
