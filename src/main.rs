use bevy::{
    core::FixedTimestep,
    math::{const_vec2, const_vec3},
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use bevy::core::CoreSystem::Time;
use crate::KeyCode::T;


const PLAYER_SPEED: f32 = 300.0;

const TIME_STEP: f32 = 1.0 / 144.0;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::hex("3636ff").unwrap()))
        .add_startup_system(start)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(update_player),
        )
        .run();
}

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct Player;

fn update_player(keyboard: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Player>>) {

    let mut movement = Vec2::new(0.0, 0.0);

    if keyboard.pressed(KeyCode::W ) {
        movement.y += 1.0;
    }
    if keyboard.pressed(KeyCode::S ) {
        movement.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::A ) {
        movement.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::D ) {
        movement.x += 1.0;
    }

    movement = movement.normalize_or_zero();

    movement *= PLAYER_SPEED * TIME_STEP;

    query.single_mut().translation += movement.extend(0.0);

}

fn start(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands
        .spawn()
        .insert(Player)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(5.0, 5.0, 1.0),
                ..default()
            },
            texture: assets.load("textures/FINAL_BOSS.png"),
            ..default()
        })
        .insert(Collider);
}



