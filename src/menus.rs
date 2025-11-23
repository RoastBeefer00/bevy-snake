use crate::state::GameState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

// pub fn ui_system(mut contexts: EguiContexts, state: Local<GameState>) -> Result {
//     match state {
//         GameState::NewGame => {}
//         GameState::Paused => {}
//         GameState::Playing => {}
//     }
//     egui::Window::new("Hello").show(contexts.ctx_mut()?, |ui| {
//         ui.label("world");
//     });
// }
