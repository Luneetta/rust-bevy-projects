use bevy::{prelude::*, window::PrimaryWindow, app::AppExit};
use rand::prelude::*;



pub const PLAYER_SIZE: f32 = 64.0; // this is the players sprite size 
pub const PLAYER_SPEED: f32 = 500.0; // players movement speed
pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 250.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0;
pub const STAR_SPAWN_TIME: f32 = 2.0;
pub const ENEMY_SPAWN_TIME: f32 = 2.0;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .init_resource::<Score>()
    .init_resource::<StarSpawnTimer>()
    .init_resource::<EnemySpawnTimer>()
    .add_systems(Startup, (spawn_player, spawn_camera, spawn_enemy, spawn_stars))
    .add_systems(Update, (player_movement,
        update_enemy_direction,
        confine_enemy,
        player_collision,
        confine_player,
        enemy_movement,
        player_star_collision,
        tick_star_spawn_timer,
        tick_enemy_spawn_timer,
        spawn_enemy_over_time,
        spawn_stars_over_time,
        exit_game,
        update_score))
    .run();
}

#[derive(Component)]
pub struct Player{}

#[derive(Component)]
pub struct Enemy{
    pub direction: Vec2,
}
#[derive(Component)]
pub struct Star{}

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

impl Default for Score {
    fn default() -> Score {
        Score {
            value: 0
        }
    }
}

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> StarSpawnTimer {
        StarSpawnTimer {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating), 
        }
    }
}
#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}
impl Default for EnemySpawnTimer {
    fn default() -> EnemySpawnTimer {
        EnemySpawnTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}




pub fn spawn_player(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>, asset_server: Res<AssetServer>){
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn(
        (
            SpriteBundle{
                transform: Transform:: from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
                texture: asset_server.load("sprites/ball_blue_large.png"),
                ..default()
            },
            Player {},
        )
    );
}

pub fn spawn_enemy(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
){
    let window: &Window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x: f32 = random::<f32>() * window.width();
        let random_y: f32 = random::<f32>() * window.height();

        commands.spawn(
            (
                SpriteBundle{
                    transform: Transform::from_xyz(random_x, random_y, 0.0),
                    texture: asset_server.load("sprites/ball_red_large.png"),
                    ..default()
                },
                Enemy{
                    direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
                },
            )
        );


    }
}
pub 
fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
){
    let window: &Window = window_query.get_single().unwrap();
    for _ in 0..NUMBER_OF_STARS {
        let random_x: f32 = random::<f32>() * window.width();
        let random_y: f32 = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle{
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}




pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>){
    let window: &Window = window_query.get_single().unwrap();
    commands.spawn(
        Camera2dBundle{
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.),
            ..default()
        }
    );
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if direction.length() > 0.0{
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();

    }
}

pub fn confine_player(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
){
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();
        let half_player_size: f32 = PLAYER_SIZE / 2.0;

        let x_min: f32 = 0.0 + half_player_size;
        let x_max: f32 = window.width() - half_player_size;
        let y_min: f32 = 0.0 + half_player_size;
        let y_max: f32 = window.height() - half_player_size; 

        let mut translation: Vec3 = player_transform.translation;

        if translation.x < x_min{
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        if translation.y < y_min{
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }
        player_transform.translation = translation;

    }
}

pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>
){
    for (mut transform, enemy) in enemy_query.iter_mut(){
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    let window: &Window = window_query.get_single().unwrap();

    let half_enemy_size: f32 = ENEMY_SIZE / 2.0;
    let x_min: f32 = 0.0 + half_enemy_size;
    let x_max: f32 = window.width() - half_enemy_size;
    let y_min: f32 = 0.0 + half_enemy_size;
    let y_max: f32 = window.height() - half_enemy_size;

    for(transform, mut enemy) in enemy_query.iter_mut(){
        let mut direction_changed = false;

        let translation: Vec3 = transform.translation;
        if translation.x < x_min || translation.x > x_max{
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }
        if translation.y < y_min || translation.y > y_max{
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }
        if direction_changed{
            if random::<f32>() > 0.5 {
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/pluck_001.ogg"),
                    ..default()
                });
            } else {
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/pluck_002.ogg"),
                    ..default()
                });
            }
        }


    }
}

pub fn confine_enemy(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for mut transform in enemy_query.iter_mut() {
        let mut translation = transform.translation;

        // Bound the enemy x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        // Bound the enemy y position
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation;
    }
}

pub fn player_collision(
    enemy_query: Query<&Transform, With<Enemy>>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter(){
            let distance = player_transform.translation.distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;
            if distance < player_radius + enemy_radius {
                println!("Game over");
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/explosionCrunch_000.ogg"),
                    ..default()
                });
                commands.entity(player_entity).despawn();
            }
        }
    }
}

pub fn player_star_collision(
    star_query: Query<(Entity, &Transform), With<Star>>,
    player_query: Query<&Transform, With<Player>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
){
    if let Ok(player_transform) = player_query.get_single() {
        for(star_entity, star_transform) in star_query.iter() {
            let distance = player_transform.translation.distance(star_transform.translation);
            let star_radius = STAR_SIZE / 2.0;
            let player_radius = PLAYER_SIZE / 2.0;

            if distance < player_radius+ star_radius {
                println!("Collected star");
                score.value += 1;
                commands.entity(star_entity).despawn();
                commands.spawn(AudioBundle{
                    source: asset_server.load("audio/impactMetal_000.ogg"),
                    ..default()
                });
            }
        }
    }

}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string());
    }
}

pub fn tick_star_spawn_timer(
    mut star_spawn_timer: ResMut<StarSpawnTimer>,
    time: Res<Time>,
){
    star_spawn_timer.timer.tick(time.delta());
}

pub fn tick_enemy_spawn_timer(
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
) {
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let random_x: f32 = random::<f32>() * window.width();
        let random_y: f32 = random::<f32>() * window.height();
        
        commands.spawn( (
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}

pub fn spawn_enemy_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<StarSpawnTimer>,
) {
    if enemy_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let random_x: f32 = random::<f32>() * window.width();
        let random_y: f32 = random::<f32>() * window.height();
        commands.spawn((
            SpriteBundle{
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
            
        
    }
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_write: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_write.send(AppExit);
    }
}