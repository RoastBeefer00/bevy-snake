use crate::{
    movement::Direction,
    snake::{SnakeBody, SnakeHead, SnakeSegment, SnakeTail},
    state::GameState,
};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub fn ui_system(
    mut contexts: EguiContexts,
    mut commands: Commands,
    mut head: Query<(Entity, &Transform), With<SnakeHead>>,
    mut body: ResMut<SnakeBody>,
    mut state: ResMut<GameState>,
    input: Res<ButtonInput<KeyCode>>,
) -> Result {
    match *state {
        GameState::NewGame => {
            if input.just_pressed(KeyCode::Space) {
                *state = GameState::Playing;
            }
        }
        GameState::Paused | GameState::Playing => {
            if input.just_pressed(KeyCode::Escape) {
                state.toggle_pause();
            }
        }
        GameState::GameOver => {
            if input.just_pressed(KeyCode::Space) {
                if let Some((head_entity, _)) = head.iter_mut().next() {
                    commands.entity(head_entity).despawn_children().despawn();
                    body.entities
                        .iter()
                        .for_each(|e| commands.entity(*e).despawn_children().despawn());
                    commands.spawn((
                        SnakeSegment::new(Direction::Right, Transform::from_xyz(0.0, 0.0, 0.0)),
                        SnakeHead,
                    ));
                    body.head_direction = Direction::Right;
                    body.entities = vec![commands
                        .spawn((
                            SnakeSegment::new(
                                Direction::Right,
                                Transform::from_xyz(-1.0, 0.0, 0.0),
                            ),
                            SnakeTail,
                        ))
                        .id()];
                }
                *state = GameState::Playing;
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
        GameState::GameOver => {
            egui::CentralPanel::default()
                .frame(
                    egui::Frame::NONE.fill(egui::Color32::from_rgba_unmultiplied(180, 0, 0, 200)),
                )
                .show(contexts.ctx_mut()?, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(20.0);

                        ui.heading(
                            egui::RichText::new("Game Over")
                                .size(48.0)
                                .color(egui::Color32::BLACK),
                        );

                        ui.add_space(30.0);

                        if ui
                            .add_sized(
                                [200.0, 50.0],
                                egui::Button::new(egui::RichText::new("Restart").size(20.0)),
                            )
                            .clicked()
                        {
                            if let Some((head_entity, _)) = head.iter_mut().next() {
                                commands.entity(head_entity).despawn_children().despawn();
                                body.entities
                                    .iter()
                                    .for_each(|e| commands.entity(*e).despawn_children().despawn());
                                crate::snake::spawn_snake(commands, body);
                            }
                            *state = GameState::Playing;
                        }

                        ui.add_space(20.0);
                    });
                });
        }
    }
    Ok(())
}
