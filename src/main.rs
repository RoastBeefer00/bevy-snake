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
use menus::ui_system;
use movement::*;
use snake::*;
use state::*;

const WINDOW_WIDTH: u32 = 500;
const WINDOW_HEIGHT: u32 = 500;

fn main() {
    let mut app = App::new();
    let window_plugin = if cfg!(target_arch = "wasm32") {
        WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#snake-canvas".into()),
                resolution: WindowResolution::new(800, 800),
                ..default()
            }),
            ..default()
        }
    } else {
        WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                resizable: false,
                ..default()
            }),
            ..default()
        }
    };
    app.add_plugins(DefaultPlugins.set(window_plugin))
        .add_plugins(EguiPlugin::default())
        .add_plugins(CameraPlugin)
        .add_plugins(GameStatePlugin)
        .add_plugins(SnakePlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(FoodPlugin)
        .add_systems(EguiPrimaryContextPass, ui_system)
        .run();
}
