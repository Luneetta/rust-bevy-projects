use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

pub const PLAYER_SIZE: f32 = 64.0;
pub const PLAYER_WIDTH: f32 = 10.0;
pub const STAR_SIZE: f32 = 30.0;
pub const PLAYER_SPEED: f32 = 600.0;
pub const STAR_SPEED: f32 = 500.0;
pub const STAR_ROTATE_SPEED: f32 = 5.0;
fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .init_resource::<PlayerOneScore>()
    .init_resource::<PlayerTwoScore>()
    .add_systems(Startup, spawn_camera)
    .add_systems(Startup, (spawn_player_one, spawn_player_two, spawn_star))
    .add_systems(Update, (confine_player_one, change_star_direction, player_star_collision, player_two_star_collision))
    .add_systems(Update, (player_one_movement, star_direction, player_two_movement))
    .run();
}

//component declarations

#[derive(Component)]
pub struct PlayerOne{}

#[derive(Component)]
pub struct PlayerTwo{}

#[derive(Component)]
pub struct Star{
    pub direction: Vec2,
}

//resource declarations
#[derive(Resource)]
pub struct PlayerOneScore{
    pub value: u32,
}

#[derive(Resource)]
pub struct PlayerTwoScore{
    pub value: u32,
}

impl Default for PlayerOneScore {
    fn default() -> PlayerOneScore {
        PlayerOneScore {
            value: 0
        }
    }
}

impl Default for PlayerTwoScore {
    fn default() -> PlayerTwoScore {
        PlayerTwoScore {value: 0}
    }
}


//spawning entities
pub fn spawn_camera(
    mut commands: Commands,
     window_query: Query<&Window, With<PrimaryWindow>>
    ){

    let window: &Window = window_query.get_single().unwrap();
    commands.spawn(
        Camera2dBundle{
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.),
            ..default()
        }
    );
}

pub fn spawn_player_one(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window: &Window = window_query.get_single().unwrap();
    commands.spawn (
        (
            SpriteBundle {
                transform: Transform::from_xyz(15.0, window.height() / 2.0, 0.0),
                texture: asset_server.load("sprites/player_sprite.png"),
                ..default()
            },
            PlayerOne{},
        )
    );
}

pub fn spawn_player_two(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window: &Window = window_query.get_single().unwrap();
    commands.spawn (
        (
            SpriteBundle {
                transform: Transform::from_xyz( window.width() - 15.0, window.height() / 2.0, 0.0),
                texture: asset_server.load("sprites/player_sprite.png"),
                ..default()
            },
            PlayerTwo{},
        )
    );
}
pub fn spawn_star(
    mut commands: Commands,
    window_query:Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window: &Window = window_query.get_single().unwrap();
    let rand_x = rand::thread_rng().gen_range(0..2);
    let rand_y = rand::thread_rng().gen_range(0..2);
    let x_value;
    let y_value: f32;
    if rand_x == 0 {
        x_value = 1.0;
    } else {
        x_value = -1.0;
    }
    if rand_y == 0 {
        y_value = 0.4;
    } else {
        y_value = -0.4;
    }


    commands.spawn(
        (
            SpriteBundle{
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star{
                direction: Vec2::new(x_value, y_value).normalize(),
            },
        )
    );
}




//movement and confinement systems

pub fn player_one_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<PlayerOne>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_one(
    mut player_query: Query<&mut Transform, With<PlayerOne>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();
        let half_player_size = PLAYER_SIZE / 2.0;

        let y_min: f32 = 0.0 + half_player_size;
        let y_max: f32 = window.height() - half_player_size;

        let mut translation: Vec3 = player_transform.translation;

        if translation.y < y_min {
            translation.y = y_min;
        }
        if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}

pub fn star_direction(
    mut star_query: Query<(&mut Transform, &Star)>,
    time: Res<Time>
) {
    for(mut transform, star) in star_query.iter_mut() {
        let direction = Vec3::new(star.direction.x, star.direction.y, 0.0);
        transform.translation += direction * STAR_SPEED * time.delta_seconds();
        transform.rotation *= Quat::from_rotation_z( STAR_ROTATE_SPEED * time.delta_seconds());
    }
}

pub fn change_star_direction(
    mut star_query: Query<(&Transform, &mut Star)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let max_y = window.height() - (STAR_SIZE / 2.0);
    let min_y: f32 = 0.0 + (STAR_SIZE / 2.0);

    for(transform, mut star) in star_query.iter_mut() {
        let translation: Vec3 = transform.translation;
        if translation.y < min_y { 
            star.direction.y *= -1.0;
        }
        if translation.y > max_y {
            star.direction.y *= -1.0;
        }

    }
}


pub fn player_star_collision(
    mut star_query: Query<(&Transform, &mut Star)>,
    player_query: Query<&Transform, With<PlayerOne>>,
) {
    for(star_transform, mut star) in star_query.iter_mut() {
        if let Ok(player_transform) = player_query.get_single() {
            let star_position = star_transform.translation;
            let player_position = player_transform.translation;
            let x_distance = star_position.x - player_position.x;

            if x_distance <= (PLAYER_WIDTH / 2.0) + (STAR_SIZE / 2.0) && (star_position.y - (STAR_SIZE / 2.0) <= player_position.y + (PLAYER_SIZE / 2.0) && star_position.y + (STAR_SIZE / 2.0) >= player_position.y - (PLAYER_SIZE / 2.0))  {
                star.direction.x *= -1.0;
            }
        }
    }
}

pub fn player_two_movement(
    star_query: Query<&Transform, With<Star>>,
    mut player_two_query: Query<&mut Transform, (With<PlayerTwo>, Without<Star>)>,
) {
    if let Ok(mut player_transform) = player_two_query.get_single_mut() {
        if let Ok(star_transform) = star_query.get_single() {
            player_transform.translation.y = star_transform.translation.y;
        }
    }
}


pub fn player_two_star_collision(
    mut star_query: Query<(&Transform, &mut Star)>,
    player_query: Query<&Transform, With<PlayerTwo>>,
) {
    for(star_transform, mut star) in star_query.iter_mut() {
        if let Ok(player_transform) = player_query.get_single() {
            let star_position = star_transform.translation;
            let player_position = player_transform.translation;
            let x_distance = player_position.x - star_position.x;
            if x_distance <= (PLAYER_WIDTH / 2.0) + (STAR_SIZE / 2.0) && (star_position.y - (STAR_SIZE / 2.0) <= player_position.y + (PLAYER_SIZE / 2.0) && star_position.y + (STAR_SIZE / 2.0) >= player_position.y - (PLAYER_SIZE / 2.0))  {
                star.direction.x *= -1.0;
            }
        }
    }
}