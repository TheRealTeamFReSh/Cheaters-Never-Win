use crate::states::GameStates;
use bevy::{prelude::*, render::camera::Camera};
use std::f32::consts::PI;

#[derive(Debug, Component)]
pub struct Player {
    pub speed: f32,
}

#[derive(Component)]
struct AnimationTimer(Timer);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_character)
            .add_system(move_character)
            .add_system(follow_player_camera)
            .add_system(animate_sprite);
    }
}

/// Spawns our character and loads it's resources
fn spawn_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("gabe-idle-run.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let player = Player { speed: 5.0 };

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                scale: Vec3::new(2.0, 2.0, 1.0),
                translation: Vec3::new(0.0, -200.0, 100.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .insert(player);
}

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

fn move_character(
    app_state: Res<State<GameStates>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    match app_state.current() {
        GameStates::Main => {
            for (player, mut transform) in query.iter_mut() {
                if keyboard_input.pressed(KeyCode::A) {
                    transform.translation.x += -1.0 * player.speed;
                    transform.rotation = Quat::from_rotation_y(PI).into();
                } else if keyboard_input.pressed(KeyCode::D) {
                    transform.translation.x += 1.0 * player.speed;
                    transform.rotation = Quat::from_rotation_y(0.0).into();
                }
            }
        }
        _ => {}
    }
}

fn follow_player_camera(
    player: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if let Some(player) = player.iter().next() {
        for mut transform in camera.iter_mut() {
            transform.translation.x = player.translation.x;
        }
    }
}
