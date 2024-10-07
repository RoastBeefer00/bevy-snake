use crate::snake::{SnakeGrow, SnakeHead};
use bevy::{ecs::schedule::MultiThreadedExecutor, prelude::*};
use rand::Rng;

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_food);
        app.add_systems(Update, handle_food_eaten);
        app.add_systems(Update, eat_food);
        app.add_event::<FoodEaten>();
    }
}

#[derive(Component)]
struct Food;

#[derive(Event)]
struct FoodEaten;

fn spawn_food(mut commands: Commands, transforms: Query<&Transform>) {
    let positions = transforms
        .iter()
        .map(|transform| transform.translation)
        .collect::<Vec<Vec3>>();

    let mut rng = rand::thread_rng();
    let x = rng.gen_range(-9..9);
    let y = rng.gen_range(-9..9);
    let position = Vec3::new(x as f32, y as f32, 0.0);
    if positions.contains(&position) {
        spawn_food(commands, transforms);
    } else {
        commands.spawn((
            Food,
            SpriteBundle {
                transform: Transform {
                    translation: position,
                    ..default()
                },
                sprite: Sprite {
                    color: Color::BLACK,
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    ..default()
                },
                ..default()
            },
        ));
    }
}

fn handle_food_eaten(
    mut commands: Commands,
    food: Query<(Entity, &Transform), With<Food>>,
    head: Query<&Transform, With<SnakeHead>>,
    mut food_writer: EventWriter<FoodEaten>,
) {
    if let Some(head_transform) = head.iter().next() {
        for (entity, food_transform) in food.iter() {
            if head_transform.translation == food_transform.translation {
                commands.entity(entity).despawn_recursive();
                food_writer.send(FoodEaten);
            }
        }
    }
}

fn eat_food(
    commands: Commands,
    transforms: Query<&Transform>,
    mut food_reader: EventReader<FoodEaten>,
    mut grow_writer: EventWriter<SnakeGrow>,
) {
    if food_reader.read().next().is_some() {
        spawn_food(commands, transforms);
        grow_writer.send(SnakeGrow);
    }
}
