use bevy::prelude::*;

#[derive(PartialEq, Eq, Resource)]
pub enum GameState {
    Playing,
    Paused,
    NewGame,
}

impl GameState {
    pub fn toggle_pause(&mut self) {
        *self = match *self {
            GameState::Playing => GameState::Paused,
            GameState::Paused => GameState::Playing,
            GameState::NewGame => GameState::NewGame,
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState::NewGame
    }
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameState::default());
    }
}
