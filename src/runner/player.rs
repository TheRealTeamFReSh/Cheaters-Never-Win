use crate::{camera::TwoDCameraComponent, physics, states::GameStates};
use bevy::{prelude::*, render::camera::Camera};
use bevy_rapier2d::prelude::*;

use super::CollectedChars;
use crate::cheat_codes::{CheatCodeKind, CheatCodeResource};
use crate::interactables::{CharTextComponent, InteractableComponent, InteractableType};

#[derive(Debug, Component)]
pub struct Player {
    pub speed: f32,
    pub acceleration: f32,
    pub deceleration: f32,
    pub feet_touching_platforms: FeetTouchingPlatforms,
}

#[derive(Debug)]
pub struct FeetTouchingPlatforms {
    pub platforms: Vec<Entity>,
}

#[derive(Component)]
pub struct PlayerAnimationTimer(Timer);

#[derive(Component)]
pub struct PlayerFeet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CollectedChars { values: Vec::new() })
            .insert_resource(PlayerAnimationResource {
                run: AnimationData {
                    length: 8,
                    offset: 0,
                },
                jump: AnimationData {
                    length: 4,
                    offset: 8,
                },
            })
            .add_system_set(
                SystemSet::on_enter(GameStates::Main)
                    .with_system(spawn_character.after("setup_physics")),
            )
            .add_system_set(
                SystemSet::on_update(GameStates::Main)
                    .with_system(follow_player_camera)
                    .with_system(animate_sprite)
                    .with_system(move_character)
                    .with_system(detect_char_interactable)
                    .with_system(player_feet),
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
    let texture_handle = asset_server.load("player.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(71.0, 67.0), 8, 2);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let player = Player {
        speed: 8.0,
        acceleration: 0.12,
        deceleration: 0.1,
        feet_touching_platforms: FeetTouchingPlatforms { platforms: vec![] },
    };

    let collider_size_hx = 30.0 / rapier_config.scale / 2.0;
    let collider_size_hy = 70.0 / rapier_config.scale / 2.0;

    commands
        .spawn_bundle(RigidBodyBundle {
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
        .insert(Name::new("Player"))
        .insert(player)
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    transform: Transform {
                        scale: Vec3::new(1.5, 1.5, 1.0),
                        translation: Vec3::new(0.0, 12.0, 100.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(PlayerAnimationTimer(Timer::from_seconds(0.1, true)));

            parent
                .spawn_bundle(ColliderBundle {
                    shape: ColliderShape::cuboid(
                        collider_size_hx,
                        35.0 / rapier_config.scale / 2.0,
                    )
                    .into(),
                    position: [0.0, -35.0 / rapier_config.scale / 2.0].into(),
                    collider_type: ColliderType::Sensor.into(),
                    flags: ColliderFlags {
                        active_events: ActiveEvents::INTERSECTION_EVENTS,
                        ..Default::default()
                    }
                    .into(),
                    ..Default::default()
                })
                .insert(PlayerFeet);
        });
}

pub fn player_feet(
    mut intersection_events: EventReader<IntersectionEvent>,
    player_feet_query: Query<Entity, With<PlayerFeet>>,
    mut player_query: Query<&mut Player>,
) {
    for event in intersection_events.iter() {
        let collider1_entity = event.collider1.entity();
        let collider2_entity = event.collider2.entity();
        for feet_entity in player_feet_query.iter() {
            for mut player in player_query.iter_mut() {
                if event.intersecting {
                    if collider1_entity == feet_entity {
                        player
                            .feet_touching_platforms
                            .platforms
                            .push(collider2_entity);
                    } else if collider2_entity == feet_entity {
                        player
                            .feet_touching_platforms
                            .platforms
                            .push(collider1_entity);
                    }
                } else if collider1_entity == feet_entity {
                    let index = player
                        .feet_touching_platforms
                        .platforms
                        .iter()
                        .position(|x| *x == collider2_entity)
                        .unwrap();
                    player.feet_touching_platforms.platforms.remove(index);
                } else if collider2_entity == feet_entity {
                    let index = player
                        .feet_touching_platforms
                        .platforms
                        .iter()
                        .position(|x| *x == collider1_entity)
                        .unwrap();
                    player.feet_touching_platforms.platforms.remove(index);
                }
            }
        }
    }
}

pub struct PlayerAnimationResource {
    pub run: AnimationData,
    pub jump: AnimationData,
}

pub struct AnimationData {
    pub length: usize,
    pub offset: usize,
}

pub fn animate_sprite(
    time: Res<Time>,
    player_animation_resource: Res<PlayerAnimationResource>,
    player_query: Query<&Player>,
    mut query: Query<(&mut PlayerAnimationTimer, &mut TextureAtlasSprite)>,
) {
    for player in player_query.iter() {
        for (mut timer, mut sprite) in query.iter_mut() {
            timer.0.tick(time.delta());
            if timer.0.just_finished() {
                // is the player jumping
                if player.feet_touching_platforms.platforms.is_empty() {
                    // player is jumping
                    if sprite.index < player_animation_resource.jump.offset
                        || sprite.index
                            >= (player_animation_resource.jump.length
                                + player_animation_resource.jump.offset)
                    {
                        sprite.index = player_animation_resource.jump.offset;
                    } else if sprite.index
                        < (player_animation_resource.jump.length
                            + player_animation_resource.jump.offset)
                            - 1
                    {
                        sprite.index += 1;
                    }
                } else {
                    // player is running
                    if sprite.index >= player_animation_resource.run.length {
                        sprite.index = 0;
                    } else {
                        sprite.index = (sprite.index + 1) % player_animation_resource.run.length;
                    }
                }
            }
        }
    }
}

fn move_character(
    keyboard_input: Res<Input<KeyCode>>,
    rapier_config: Res<RapierConfiguration>,
    mut query: Query<(
        &Player,
        &mut RigidBodyVelocityComponent,
        &RigidBodyMassPropsComponent,
    )>,
    cheat_codes: ResMut<CheatCodeResource>,
) {
    for (player, mut rb_vel, rb_mprops) in query.iter_mut() {
        let _up = keyboard_input.pressed(KeyCode::W);
        let _down = keyboard_input.pressed(KeyCode::S);
        let left = keyboard_input.pressed(KeyCode::A);
        let right = keyboard_input.pressed(KeyCode::D);

        // TODO: check if player is on the ground
        let jump = cheat_codes.is_code_activated(&CheatCodeKind::Jump)
            && keyboard_input.just_released(KeyCode::Space)
            && !player.feet_touching_platforms.platforms.is_empty();

        let x_axis = -(left as i8) + right as i8;

        if x_axis != 0 {
            rb_vel.linvel.x += player.acceleration * (x_axis as f32) * rapier_config.scale;
            if rb_vel.linvel.x.abs() > player.speed * rapier_config.scale {
                rb_vel.linvel.x =
                    (rb_vel.linvel.x / rb_vel.linvel.x.abs()) * player.speed * rapier_config.scale;
            }
        } else if rb_vel.linvel.x.abs() > 0.4 {
            // decelerate
            rb_vel.linvel.x -= player.deceleration
                * (rb_vel.linvel.x / rb_vel.linvel.x.abs())
                * rapier_config.scale;
        } else {
            rb_vel.linvel.x = 0.0;
        }

        if jump {
            physics::jump(1500.0, &mut rb_vel, rb_mprops)
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
                        collected_chars.values.push(char_component.value);
                        commands.entity(entity).despawn();
                    }
                }
                _ => {}
            }
        }
    }
}
