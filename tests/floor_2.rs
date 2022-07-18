#![cfg(target_arch = "wasm32")]

use all_asserts::{assert_ge, assert_le};
use tower::do_rebirth_internal;
use tower::game::Game;
use tower::presets::make_f2;
use tower::types::*;
use tower::util::{run_until_dead, set_all_floors_to_automatable, set_exploration_to};

use wasm_bindgen_test::wasm_bindgen_test;

fn exploration_completed(game: &Game, exploration: AllExplors) -> bool {
    game.state.get_exploration(exploration).is_completed
}

#[wasm_bindgen_test]
fn preset_check() {
    let game = &mut Game::new();
    game.load_game(make_f2());
    set_all_floors_to_automatable(&mut game.state);
    run_until_dead(game);

    assert_eq!(game.state.current_floor, FloorTypes::Second);
}

// #[wasm_bindgen_test]
// fn test_first_two() {
//     let game = &mut Game::new();
//     game.load_game(make_automated_start());
//     set_exploration_to(&mut game.state, F1Explors::BackToMassive.into(), 2);
//     run_until_dead(game);

//     assert_eq!(game.state.current_area, F1Areas::MassiveRoom.into());
//     assert!(exploration_completed(game, F1Explors::BackToMassive.into()));
//     assert!(!exploration_completed(game, F1Explors::BlockedDoor.into()));
//     do_rebirth_internal(game);
//     run_until_dead(game);
//     assert!(exploration_completed(game, F1Explors::BlockedDoor.into()));
// }

// #[wasm_bindgen_test]
// fn test_completion_time() {
//     let game = &mut Game::new();
//     game.load_game(make_automated_start());
//     while game.state.current_floor == FloorTypes::Starting {
//         do_rebirth_internal(game);
//         run_until_dead(game);
//     }

//     assert_le!(game.state.status.reincarnation, 8);
//     assert_ge!(game.state.status.reincarnation, 5);
// }