use crate::engine::intermediate_state::IntermediateState;
use crate::meta::MetaData;
use crate::state::history::History;
use crate::state::State;
use crate::world::World;
use crate::WORLD;
use crate::{action_queue::ActionQueue, engine::actionless_update};
use serde::{Deserialize, Serialize};

pub struct Game {
    pub world: &'static World,
    pub state: State,
    pub history: History,
    pub intermediate_state: IntermediateState,
    pub meta_data: MetaData,
    pub action_queue: ActionQueue,
    pub just_loaded: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameSave {
    pub state: State,
    pub meta_data: MetaData,
    pub action_queue: ActionQueue,
    pub history: History,
}

impl Default for GameSave {
    fn default() -> GameSave {
        Game::new().into()
    }
}

impl From<&Game> for GameSave {
    fn from(game: &Game) -> Self {
        GameSave {
            state: game.state.clone(),
            meta_data: game.meta_data.clone(),
            action_queue: game.action_queue.clone(),
            history: game.history.clone(),
        }
    }
}

impl From<Game> for GameSave {
    fn from(game: Game) -> Self {
        let Game {
            world: _,
            state,
            history,
            intermediate_state: _,
            meta_data,
            action_queue,
            just_loaded: _,
        } = game;
        GameSave {
            state,
            meta_data,
            action_queue,
            history,
        }
    }
}

impl Default for Game {
    fn default() -> Game {
        Game::new()
    }
}

impl Game {
    pub fn new() -> Game {
        let world = &WORLD;
        let state = State::default();
        let history = History::new(&state);
        let intermediate_state = IntermediateState::new();
        let meta_data = MetaData::new();
        let action_queue = ActionQueue::default();
        let mut game = Game {
            world,
            state,
            history,
            intermediate_state,
            meta_data,
            action_queue,
            just_loaded: false,
        };
        actionless_update(&mut game);
        game
    }

    pub fn update_history(&mut self) {
        self.history.update_history(&self.state);
    }

    pub fn hard_reset(&mut self) {
        self.state = State::default();
        self.history = History::new(&self.state);
        self.intermediate_state = IntermediateState::new();
        self.meta_data = MetaData::new();
        self.action_queue = ActionQueue::default();
    }

    pub fn load_game(&mut self, save: GameSave) {
        let GameSave {
            state,
            meta_data,
            action_queue,
            history,
        } = save;
        self.state = state;
        self.meta_data = meta_data;
        self.action_queue = action_queue;
        self.history = history;
        self.just_loaded = true;
    }
}
