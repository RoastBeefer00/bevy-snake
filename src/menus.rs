use crate::state::GameState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub fn ui_system(
    mut contexts: EguiContexts,
    mut state: ResMut<GameState>,
    input: Res<ButtonInput<KeyCode>>,
) -> Result {
    match *state {
        GameState::NewGame => {
            if input.just_pressed(KeyCode::Space) {
                *state = GameState::Playing;
            }
        }
        GameState::Paused => {
            if input.just_pressed(KeyCode::Escape) {
                state.toggle_pause();
            }
        }
        GameState::Playing => {
            if input.just_pressed(KeyCode::Escape) {
                state.toggle_pause();
            }
        }
    }
    match *state {
        GameState::NewGame => {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE.fill(egui::Color32::from_black_alpha(180)))
                .show(contexts.ctx_mut()?, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(20.0);

                        ui.heading(
                            egui::RichText::new("Snake")
                                .size(48.0)
                                .color(egui::Color32::BLACK),
                        );

                        ui.add_space(30.0);

                        if ui
                            .add_sized(
                                [200.0, 50.0],
                                egui::Button::new(egui::RichText::new("Start Game").size(20.0)),
                            )
                            .clicked()
                        {
                            *state = GameState::Playing;
                        }

                        ui.add_space(20.0);
                    });
                });
        }
        GameState::Paused => {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE.fill(egui::Color32::from_black_alpha(180)))
                .show(contexts.ctx_mut()?, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(20.0);

                        ui.heading(
                            egui::RichText::new("Paused")
                                .size(48.0)
                                .color(egui::Color32::BLACK),
                        );

                        ui.add_space(30.0);

                        if ui
                            .add_sized(
                                [200.0, 50.0],
                                egui::Button::new(egui::RichText::new("Resume").size(20.0)),
                            )
                            .clicked()
                        {
                            *state = GameState::Playing;
                        }

                        ui.add_space(20.0);
                    });
                });
        }
        GameState::Playing => {}
    }
    Ok(())
}
