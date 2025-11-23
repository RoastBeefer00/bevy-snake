mod camera;
mod food;
mod menus;
mod movement;
mod snake;
mod state;

use bevy::{prelude::*, window::WindowResolution};
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};
use camera::*;
use food::*;
// use menus::ui_system;
use movement::*;
use snake::*;
use state::*;

const WINDOW_WIDTH: u32 = 500;
const WINDOW_HEIGHT: u32 = 500;

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
        // .add_plugins(EguiPlugin::default())
        .add_plugins(CameraPlugin)
        // .add_plugins(GameStatePlugin)
        .add_plugins(SnakePlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(FoodPlugin)
        // .add_systems(EguiPrimaryContextPass, ui_system)
        .run();
}
