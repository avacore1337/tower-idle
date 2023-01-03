#![cfg(target_arch = "wasm32")]

use all_asserts::{assert_ge, assert_le};
use tower::engine::do_rebirth_internal;
use tower::game::Game;
use tower::presets::make_automated_start;
use tower::types::*;

use tower::util::{run_until_dead, set_exploration_to};
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
fn test_sanity_check() {
    let game = &mut Game::new();
    for skill in game.state.skills.iter() {
        assert_eq!(skill.talent, 0.0);
    }
}

fn exploration_completed(game: &Game, exploration: AllExplors) -> bool {
    game.state.get_exploration(exploration).is_completed
}

#[wasm_bindgen_test]
fn test_first_run() {
    let game = &mut Game::new();
    game.load_game(make_automated_start());
    set_exploration_to(&mut game.state, F1Explors::BackToMassive.into(), 2);
    run_until_dead(game);

    assert_eq!(game.state.current_area, F1Areas::MassiveRoom.into());
    assert!(exploration_completed(game, F1Explors::BackToMassive.into()));
    assert!(!exploration_completed(game, F1Explors::BlockedDoor.into()));
}

#[wasm_bindgen_test]
fn test_first_two() {
    let game = &mut Game::new();
    game.load_game(make_automated_start());
    set_exploration_to(&mut game.state, F1Explors::BackToMassive.into(), 2);
    run_until_dead(game);

    assert_eq!(game.state.current_area, F1Areas::MassiveRoom.into());
    assert!(exploration_completed(game, F1Explors::BackToMassive.into()));
    assert!(!exploration_completed(game, F1Explors::BlockedDoor.into()));
    do_rebirth_internal(game);
    run_until_dead(game);
    assert!(exploration_completed(game, F1Explors::BlockedDoor.into()));
}

#[wasm_bindgen_test]
fn test_completion_time() {
    let game = &mut Game::new();
    game.load_game(make_automated_start());
    while game.state.current_floor == FloorTypes::Starting {
        do_rebirth_internal(game);
        run_until_dead(game);
    }

    assert_le!(game.state.status.reincarnation, 8);
    assert_ge!(game.state.status.reincarnation, 5);
}
