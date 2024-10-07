use bevy::prelude::*;

use crate::food::Food;
use crate::movement::Direction;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SnakeBody {
            head_direction: Direction::Right,
            entities: vec![],
        });
        app.add_systems(Startup, spawn_snake);
        app.add_event::<SnakeGrow>();
        app.add_systems(Update, grow_snake);
        app.add_systems(PreUpdate, handle_snake_collision);
    }
}

#[derive(Component)]
pub struct SnakeHead;

#[derive(Component)]
pub struct SnakeTail;

#[derive(Bundle)]
pub struct SnakeSegment {
    direction: Direction,
    sprite: SpriteBundle,
}

impl SnakeSegment {
    fn new(direction: Direction, transform: Transform) -> Self {
        SnakeSegment {
            direction: direction,
            sprite: SpriteBundle {
                transform: transform,
                sprite: Sprite {
                    color: Color::BLACK,
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    ..default()
                },
                ..default()
            },
        }
    }
}

#[derive(Resource)]
pub struct SnakeBody {
    pub head_direction: Direction,
    pub entities: Vec<Entity>,
}

#[derive(Event)]
pub struct SnakeGrow;

fn spawn_snake(mut commands: Commands, mut body: ResMut<SnakeBody>) {
    commands.spawn((
        SnakeSegment::new(Direction::Right, Transform::from_xyz(0.0, 0.0, 0.0)),
        SnakeHead,
    ));
    body.head_direction = Direction::Right;
    body.entities = vec![commands
        .spawn((
            SnakeSegment::new(Direction::Right, Transform::from_xyz(-1.0, 0.0, 0.0)),
            SnakeTail,
        ))
        .id()];
}

fn grow_snake(
    mut commands: Commands,
    mut body: ResMut<SnakeBody>,
    mut segments: Query<(&Direction, &Transform), Without<SnakeHead>>,
    mut grow_reader: EventReader<SnakeGrow>,
) {
    for _event in grow_reader.read() {
        let last_entity = body.entities.last().unwrap();
        let last_segment = segments.get_mut(*last_entity).unwrap();
        let placement = match last_segment.0 {
            Direction::Up => Transform::from_xyz(
                last_segment.1.translation.x,
                last_segment.1.translation.y - 1.0,
                0.0,
            ),
            Direction::Down => Transform::from_xyz(
                last_segment.1.translation.x,
                last_segment.1.translation.y + 1.0,
                0.0,
            ),
            Direction::Left => Transform::from_xyz(
                last_segment.1.translation.x + 1.0,
                last_segment.1.translation.y,
                0.0,
            ),
            Direction::Right => Transform::from_xyz(
                last_segment.1.translation.x - 1.0,
                last_segment.1.translation.y,
                0.0,
            ),
        };
        body.entities.push(
            commands
                .spawn((SnakeSegment::new(*last_segment.0, placement), SnakeTail))
                .id(),
        )
    }
}

fn handle_snake_collision(
    mut commands: Commands,
    body: ResMut<SnakeBody>,
    mut head: Query<(Entity, &Transform), With<SnakeHead>>,
    segments: Query<&Transform, (Without<SnakeHead>, With<SnakeTail>)>,
) {
    if let Some((head_entity, head_transform)) = head.iter_mut().next() {
        for segment_transform in segments.iter() {
            if head_transform.translation == segment_transform.translation {
                error!(
                    "Head at pos {:?} collided with tail at pos {:?}",
                    head_transform.translation, segment_transform.translation
                );
                commands.entity(head_entity).despawn_recursive();
                body.entities
                    .iter()
                    .for_each(|e| commands.entity(*e).despawn_recursive());
                self::spawn_snake(commands, body);
                break;
            }
        }
    };
}
