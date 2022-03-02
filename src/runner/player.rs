use crate::{camera::TwoDCameraComponent, states::GameStates};
use bevy::{prelude::*, render::camera::Camera};
use bevy_rapier2d::prelude::*;

use super::CollectedChars;
use crate::interactables::{CharTextComponent, InteractableComponent, InteractableType};

#[derive(Debug, Component)]
pub struct Player {
    pub speed: f32,
    pub acceleration: f32,
    pub deceleration: f32,
}

#[derive(Component)]
pub struct PlayerAnimationTimer(Timer);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CollectedChars { values: Vec::new() })
            .add_startup_system(spawn_character.after("setup_physics"))
            .add_system_set(
                SystemSet::on_update(GameStates::Main)
                    .with_system(follow_player_camera)
                    .with_system(animate_sprite)
                    .with_system(move_character)
                    .with_system(detect_char_interactable),
            );
    }
}

/// Spawns our character and loads it's resources
fn spawn_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    rapier_config: Res<RapierConfiguration>,
) {
    let texture_handle = asset_server.load("gabe-idle-run.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let player = Player {
        speed: 5.0,
        acceleration: 0.2,
        deceleration: 0.2,
    };

    let collider_size_hx = 24.0 * 2.0 / rapier_config.scale / 2.0;
    let collider_size_hy = 24.0 * 2.0 / rapier_config.scale / 2.0;

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                scale: Vec3::new(2.0, 2.0, 1.0),
                translation: Vec3::new(0.0, 0.0, 100.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Dynamic.into(),
            mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
            position: Vec2::new(0.0, -200.0 / rapier_config.scale).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(collider_size_hx, collider_size_hy).into(),
            material: ColliderMaterial {
                friction: 0.5,
                restitution: 0.0,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(PlayerAnimationTimer(Timer::from_seconds(0.1, true)))
        .insert(Name::new("Player"))
        .insert(player);
}

pub fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut PlayerAnimationTimer,
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
    keyboard_input: Res<Input<KeyCode>>,
    rapier_config: Res<RapierConfiguration>,
    mut query: Query<(&Player, &mut RigidBodyVelocityComponent)>,
) {
    for (player, mut rb_vel) in query.iter_mut() {
        let _up = keyboard_input.pressed(KeyCode::W);
        let _down = keyboard_input.pressed(KeyCode::S);
        let left = keyboard_input.pressed(KeyCode::A);
        let right = keyboard_input.pressed(KeyCode::D);

        let x_axis = -(left as i8) + right as i8;

        if x_axis != 0 {
            rb_vel.linvel.x += player.acceleration * (x_axis as f32) * rapier_config.scale;
            if rb_vel.linvel.x.abs() > player.speed * rapier_config.scale {
                rb_vel.linvel.x =
                    (rb_vel.linvel.x / rb_vel.linvel.x.abs()) * player.speed * rapier_config.scale;
            }
        } else if rb_vel.linvel.x.abs() > 0.01 {
            // decelerate
            rb_vel.linvel.x -= player.deceleration
                * (rb_vel.linvel.x / rb_vel.linvel.x.abs())
                * rapier_config.scale;
        } else {
            rb_vel.linvel.x = 0.0;
        }
    }
}

fn follow_player_camera(
    player: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut camera: Query<&mut Transform, (With<TwoDCameraComponent>, Without<Player>)>,
) {
    if let Some(player) = player.iter().next() {
        for mut transform in camera.iter_mut() {
            transform.translation.x = player.translation.x;
        }
    }
}

fn detect_char_interactable(
    mut commands: Commands,
    mut collected_chars: ResMut<CollectedChars>,
    player_query: Query<&Transform, With<Player>>,
    interactable_query: Query<(
        Entity,
        &InteractableComponent,
        &Transform,
        &CharTextComponent,
    )>,
) {
    if let Some(player_transform) = player_query.iter().next() {
        for (entity, interactable, transform, char_component) in interactable_query.iter() {
            match interactable.interactable_type {
                InteractableType::CharText => {
                    let distance_x = player_transform.translation.x - transform.translation.x;
                    let distance_y = player_transform.translation.y - transform.translation.y;
                    let range = interactable.range;

                    if distance_x <= range
                        && distance_x >= -range
                        && distance_y <= range
                        && distance_y >= -range
                    {
                        println!("Picked up: {}", char_component.value);
                        collected_chars.values.push(char_component.value);
                        println!("Length of chars: {}", collected_chars.values.len());
                        commands.entity(entity).despawn();
                    }
                }
                _ => {}
            }
        }
    }
}
