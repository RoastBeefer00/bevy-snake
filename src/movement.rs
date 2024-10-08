use crate::snake::{SnakeBody, SnakeHead, SnakeSegment};
use bevy::{input::common_conditions::input_just_pressed, prelude::*};

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
        app.add_systems(Update, move_snake);
    }
}

fn move_snake(
    mut head: Query<(&mut Direction, &mut Transform), With<SnakeHead>>,
    mut body: ResMut<SnakeBody>,
    mut segments: Query<(&mut Direction, &mut Transform), Without<SnakeHead>>,
    mut timer: ResMut<MovementTimer>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if let Some((mut head_direction, _)) = head.iter_mut().next() {
        if input.pressed(KeyCode::ArrowUp) && body.head_direction != Direction::Down {
            *head_direction = Direction::Up;
        }
        if input.pressed(KeyCode::ArrowDown) && body.head_direction != Direction::Up {
            *head_direction = Direction::Down;
        }
        if input.pressed(KeyCode::ArrowLeft) && body.head_direction != Direction::Right {
            *head_direction = Direction::Left;
        }
        if input.pressed(KeyCode::ArrowRight) && body.head_direction != Direction::Left {
            *head_direction = Direction::Right;
        }
    };
    timer.timer.tick(time.delta());
    if !timer.timer.just_finished() {
        return;
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
