use bevy::prelude::*;

#[derive(Resource)]
pub enum GameState {
    Playing,
    Paused,
    NewGame,
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
