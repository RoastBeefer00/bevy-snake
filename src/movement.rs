use crate::{
    snake::{SnakeBody, SnakeHead},
    state::GameState,
};
use bevy::prelude::*;

#[derive(Resource)]
pub struct MovementTimer {
    pub timer: Timer,
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MovementTimer {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        });
        app.insert_resource(InputQueue::default());
        app.add_systems(Update, (queue_input_system, move_snake));
    }
}

#[derive(Resource, Default)]
pub struct InputQueue {
    queue: Vec<KeyCode>,
}

fn queue_input_system(input: Res<ButtonInput<KeyCode>>, mut queue: ResMut<InputQueue>) {
    // Add new inputs to queue
    for key in [
        KeyCode::ArrowUp,
        KeyCode::ArrowDown,
        KeyCode::ArrowLeft,
        KeyCode::ArrowRight,
    ] {
        if input.just_pressed(key) {
            if queue.queue.last() != Some(&key) {
                queue.queue.push(key);
            }
        }
    }

    // Keep queue reasonable size
    if queue.queue.len() > 3 {
        queue.queue.remove(0);
    }
}

fn move_snake(
    mut head: Query<(&mut Direction, &mut Transform), With<SnakeHead>>,
    mut body: ResMut<SnakeBody>,
    mut queue: ResMut<InputQueue>,
    mut segments: Query<(&mut Direction, &mut Transform), Without<SnakeHead>>,
    mut timer: ResMut<MovementTimer>,
    state: Res<GameState>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if *state != GameState::Playing {
        return;
    }
    timer.timer.tick(time.delta());
    if !timer.timer.just_finished() {
        return;
    }

    if let Some(key) = queue.queue.first() {
        if let Some((mut head_direction, _)) = head.iter_mut().next() {
            if key == &KeyCode::ArrowUp && body.head_direction != Direction::Down {
                *head_direction = Direction::Up;
            }
            if key == &KeyCode::ArrowDown && body.head_direction != Direction::Up {
                *head_direction = Direction::Down;
            }
            if key == &KeyCode::ArrowLeft && body.head_direction != Direction::Right {
                *head_direction = Direction::Left;
            }
            if key == &KeyCode::ArrowRight && body.head_direction != Direction::Left {
                *head_direction = Direction::Right;
            }
        };
        queue.queue.remove(0);
    }
    if let Some((head_direction, mut head_transform)) = head.iter_mut().next() {
        // Get all the old directions of every segment and place them in order
        let mut segment_directions = body
            .entities
            .iter()
            .map(|entity| *segments.get_mut(*entity).unwrap().0)
            .collect::<Vec<Direction>>();

        segment_directions.insert(0, *head_direction);

        // Move the head
        info!("Head position: {:?}", head_transform.translation);
        match *head_direction {
            Direction::Up => {
                if head_transform.translation.y >= 9.0 {
                    head_transform.translation.y *= -1.0
                } else {
                    head_transform.translation.y += 1.0
                }
                body.head_direction = *head_direction
            }
            Direction::Down => {
                if head_transform.translation.y <= -9.0 {
                    head_transform.translation.y *= -1.0
                } else {
                    head_transform.translation.y += -1.0
                }
                body.head_direction = *head_direction
            }
            Direction::Left => {
                if head_transform.translation.x <= -9.0 {
                    head_transform.translation.x *= -1.0
                } else {
                    head_transform.translation.x += -1.0
                }
                body.head_direction = *head_direction
            }
            Direction::Right => {
                if head_transform.translation.x >= 9.0 {
                    head_transform.translation.x *= -1.0
                } else {
                    head_transform.translation.x += 1.0
                }
                body.head_direction = *head_direction
            }
        }
        // Move the body
        for (direction, mut transform) in segments.iter_mut() {
            info!("Segment position: {:?}", transform.translation);
            match *direction {
                Direction::Up => {
                    if transform.translation.y >= 9.0 {
                        transform.translation.y *= -1.0
                    } else {
                        transform.translation.y += 1.0
                    }
                }
                Direction::Down => {
                    if transform.translation.y <= -9.0 {
                        transform.translation.y *= -1.0
                    } else {
                        transform.translation.y += -1.0
                    }
                }
                Direction::Left => {
                    if transform.translation.x <= -9.0 {
                        transform.translation.x *= -1.0
                    } else {
                        transform.translation.x += -1.0
                    }
                }
                Direction::Right => {
                    if transform.translation.x >= 9.0 {
                        transform.translation.x *= -1.0
                    } else {
                        transform.translation.x += 1.0
                    }
                }
            }
        }

        // Update segment directions
        body.entities.iter().enumerate().for_each(|(i, entity)| {
            *segments.get_mut(*entity).unwrap().0 = segment_directions[i];
        });
    }
}
