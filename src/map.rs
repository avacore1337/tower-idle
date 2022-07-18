use crate::game::Game;
use crate::types::*;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use serde::Serialize;
use tsify::Tsify;

#[derive(Serialize, Tsify)]
pub struct Node {
    pub name: String,
    pub x: usize,
    pub y: usize,
}

#[derive(Serialize, Tsify)]
pub struct Edge {
    pub source: String,
    pub target: String,
}

#[derive(Serialize, Tsify)]
pub struct MapData {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

pub fn generate_map_data(game: &mut Game, floor: FloorTypes) -> MapData {
    //
    let mut nodes: Vec<Node> = vec![];
    let mut edges: Vec<Edge> = vec![];
    let wfloor = game.world.get_wfloor(floor);
    let mut rng = ChaCha8Rng::seed_from_u64(2);
    for exploration in wfloor.explorations.iter() {
        let exploration_index = exploration.name.to_exploration_index();
        nodes.push(Node {
            name: exploration.display_name.to_string(),
            x: exploration_index * 100,
            y: exploration_index * rng.gen_range(0..100),
        });
        if exploration_index != 0 {
            edges.push(Edge {
                source: wfloor.explorations[(exploration_index - 1)]
                    .display_name
                    .to_string(),
                target: exploration.display_name.to_string(),
            });
        }
    }
    MapData { nodes, edges }
}
