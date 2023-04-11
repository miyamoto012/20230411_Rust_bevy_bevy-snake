use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::random;
use bevy::time::common_conditions::on_timer;
use bevy::utils::Duration;

pub const ARENA_WIDTH: u32 = 10;
pub const ARENA_HEIGHT: u32 = 10;

pub const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
pub const BACKGROUND_COLOR: Color = Color::rgb(0.04, 0.04, 0.04);
pub const FOOD_COLOR: Color = Color::rgb(1.0, 0.0, 1.0);

pub const PLAYER_SPEED: f32 = 2.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake!".into(),
                resolution: (500.0, 500.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_snake)
        .add_system(snake_movement_input.before(snake_movement))
        .add_system(snake_movement.run_if(on_timer(Duration::from_secs_f32(0.15))))
        .add_system(position_translation.in_base_set(CoreSet::PostUpdate))
        .add_system(size_scaling.in_base_set(CoreSet::PostUpdate))
        .add_system(food_spawner.run_if(on_timer(Duration::from_secs_f32(0.5))))
        .run();
}

#[derive(Component)]
struct SnakeHead{
    direction: Direction
}

#[derive(Component)]
struct Food;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

pub fn spawn_camera(mut commands: Commands){
    commands.spawn(Camera2dBundle::default());
}

pub fn spawn_snake(mut commands: Commands){
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        },
        SnakeHead { direction: Direction::Up, },
        Position { x: 3, y: 3 },
        Size::square(0.8),
    ));
}

fn snake_movement_input(
    mut snakehead_query: Query<&mut SnakeHead>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut snakehead = snakehead_query.get_single_mut().unwrap();

    let direction: Direction = if keyboard_input.pressed(KeyCode::Left) {
        Direction::Left
    } else if keyboard_input.pressed(KeyCode::Down) {
        Direction::Down
    } else if keyboard_input.pressed(KeyCode::Up) {
        Direction::Up
    } else if keyboard_input.pressed(KeyCode::Right) {
        Direction::Right
    } else {
        snakehead.direction
    };

    // 方向の反転を禁止
    if direction != snakehead.direction.opposite() {
        snakehead.direction = direction;
    }
}

fn snake_movement(
    mut snakehead_query: Query<(&mut Position, &SnakeHead)>
) {

    let (mut snakehead_position, snakehead) = snakehead_query.get_single_mut().unwrap();

    match snakehead.direction {
        Direction::Left => {
            snakehead_position.x += -1;
        }
        Direction::Right => {
            snakehead_position.x += 1;
        }
        Direction::Up => {
            snakehead_position.y += 1;
        }
        Direction::Down => {
            snakehead_position.y += -1;
        }
    };
}

fn size_scaling(
    mut size_query: Query<(&Size, &mut Transform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    for (sprite_size, mut transform) in size_query.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
            1.0,
        );
    }
}

fn position_translation(
    mut position_query: Query<(&Position, &mut Transform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    fn convert(position: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        position / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }

    let window = window_query.get_single().unwrap();

    for (position, mut transform) in position_query.iter_mut() {
        transform.translation = Vec3::new(
            convert(position.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(position.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0,
        );
    }
}

fn food_spawner(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Food)
        .insert(Position {
            x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
        })
        .insert(Size::square(0.8));
}
