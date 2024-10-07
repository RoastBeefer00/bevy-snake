mod camera;
mod food;
mod movement;
mod snake;

use bevy::{prelude::*, window::WindowResolution};
use camera::*;
use food::*;
use movement::*;
use snake::*;

const WINDOW_WIDTH: f32 = 500.0;
const WINDOW_HEIGHT: f32 = 500.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(CameraPlugin)
        .add_plugins(SnakePlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(FoodPlugin)
        .run();
}
