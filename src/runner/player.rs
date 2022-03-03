use std::time::Duration;

use crate::enemies::Enemy;
use crate::{camera::TwoDCameraComponent, physics, platforms, states::GameStates};
use bevy::{prelude::*, render::camera::Camera};
use bevy_kira_audio::{Audio, AudioChannel};
use bevy_rapier2d::prelude::*;

use super::CollectedChars;
use crate::cheat_codes::{CheatCodeKind, CheatCodeResource};
use crate::interactables::{CharTextComponent, InteractableComponent, InteractableType};
use crate::toast::ShowToast;

#[derive(Debug, Component)]
pub struct Player {
    pub speed: f32,
    pub acceleration: f32,
    pub deceleration: f32,
    pub lives: i32,
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
                run_right: AnimationData {
                    length: 8,
                    offset: 0,
                },
                jump: AnimationData {
                    length: 4,
                    offset: 8,
                },
                idle: AnimationData {
                    length: 4,
                    offset: 16,
                },
                run_left: AnimationData {
                    length: 8,
                    offset: 24,
                },
            })
            .add_system_set(
                SystemSet::on_enter(GameStates::Main)
                    .with_system(spawn_character.after("setup_physics")),
            )
            .add_event::<GameOverEvent>()
            .add_system_set(
                SystemSet::on_update(GameStates::Main)
                    .with_system(player_feet)
                    .label("player_feet"),
            )
            .add_system_set(
                SystemSet::on_update(GameStates::Main)
                    .with_system(follow_player_camera)
                    .with_system(animate_sprite)
                    .with_system(move_character)
                    .after("player_feet")
                    .with_system(detect_char_interactable)
                    .with_system(player_collide_enemy)
                    .with_system(player_fall_damage)
                    .with_system(show_terminal_toaster_notification),
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
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(71.0, 67.0), 8, 4);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let player = Player {
        speed: 8.0,
        lives: 3,
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
            flags: ColliderFlags {
                active_events: ActiveEvents::CONTACT_EVENTS,
                ..Default::default()
            }
            .into(),
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
    platform_query: Query<Entity, With<platforms::platform::Platform>>,
    mut player_query: Query<&mut Player>,
) {
    for event in intersection_events.iter() {
        let collider1_entity = event.collider1.entity();
        let collider2_entity = event.collider2.entity();

        for feet_entity in player_feet_query.iter() {
            for mut player in player_query.iter_mut() {
                for platform_entity in platform_query.iter() {
                    // remove index 0 if there are 3 elements
                    if player.feet_touching_platforms.platforms.len() > 2 {
                        player.feet_touching_platforms.platforms.remove(0);
                    }

                    if event.intersecting {
                        if collider1_entity == feet_entity
                            && !player
                                .feet_touching_platforms
                                .platforms
                                .contains(&collider2_entity)
                            && collider2_entity == platform_entity
                        {
                            player
                                .feet_touching_platforms
                                .platforms
                                .push(collider2_entity);
                        } else if collider2_entity == feet_entity
                            && !player
                                .feet_touching_platforms
                                .platforms
                                .contains(&collider1_entity)
                            && collider1_entity == platform_entity
                        {
                            player
                                .feet_touching_platforms
                                .platforms
                                .push(collider1_entity);
                        }
                    } else if collider1_entity == feet_entity {
                        while player
                            .feet_touching_platforms
                            .platforms
                            .contains(&collider2_entity)
                        {
                            let index = player
                                .feet_touching_platforms
                                .platforms
                                .iter()
                                .position(|x| *x == collider2_entity)
                                .unwrap();
                            player.feet_touching_platforms.platforms.remove(index);
                        }
                    } else if collider2_entity == feet_entity {
                        while player
                            .feet_touching_platforms
                            .platforms
                            .contains(&collider1_entity)
                        {
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
    }
}

pub struct PlayerAnimationResource {
    pub run_right: AnimationData,
    pub run_left: AnimationData,
    pub jump: AnimationData,
    pub idle: AnimationData,
}

pub struct AnimationData {
    pub length: usize,
    pub offset: usize,
}

pub fn animate_sprite(
    time: Res<Time>,
    player_animation_resource: Res<PlayerAnimationResource>,
    player_query: Query<(&Player, &RigidBodyVelocityComponent)>,
    mut query: Query<(&mut PlayerAnimationTimer, &mut TextureAtlasSprite)>,
) {
    for (player, rb_vel) in player_query.iter() {
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
                    if rb_vel.linvel.x > 0.0 {
                        // player is running right
                        if sprite.index >= player_animation_resource.run_right.length {
                            sprite.index = 0;
                        } else {
                            sprite.index =
                                (sprite.index + 1) % player_animation_resource.run_right.length;
                        }
                    } else if rb_vel.linvel.x < 0.0 {
                        //player is running left
                        if sprite.index < player_animation_resource.run_left.offset
                            || sprite.index
                                >= (player_animation_resource.run_left.length
                                    + player_animation_resource.run_left.offset)
                        {
                            sprite.index = player_animation_resource.run_left.offset;
                        } else {
                            sprite.index = ((sprite.index + 1)
                                % player_animation_resource.run_left.length)
                                + player_animation_resource.run_left.offset;
                        }
                    } else {
                        //player is idling
                        if sprite.index < player_animation_resource.idle.offset
                            || sprite.index
                                >= (player_animation_resource.idle.length
                                    + player_animation_resource.idle.offset)
                        {
                            sprite.index = player_animation_resource.idle.offset;
                        } else {
                            sprite.index = ((sprite.index + 1)
                                % player_animation_resource.idle.length)
                                + player_animation_resource.idle.offset;
                        }
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
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
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
                        let audio_channel = AudioChannel::new("sfx-channel".to_owned());
                        audio.set_volume_in_channel(0.3, &audio_channel);
                        audio.play_in_channel(asset_server.load("pickup.ogg"), &audio_channel);
                        collected_chars.values.push(char_component.value);
                        commands.entity(entity).despawn();
                    }
                }
                _ => {}
            }
        }
    }
}

pub struct GameOverEvent;

pub fn player_fall_damage(
    mut player_query: Query<(&mut Player, &Transform)>,
    mut game_over_event: EventWriter<GameOverEvent>,
) {
    for (mut player, transform) in player_query.iter_mut() {
        if transform.translation.y < -400.0 {
            player.lives = 0;
            game_over_event.send(GameOverEvent);
            info!("Fell down hole")
        }
    }
}

pub fn player_collide_enemy(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Player)>,
    enemy_query: Query<Entity, With<Enemy>>,
    mut contact_events: EventReader<ContactEvent>,
    mut game_over_event: EventWriter<GameOverEvent>,
) {
    for contact_event in contact_events.iter() {
        if let ContactEvent::Started(h1, h2) = contact_event {
            for (player_entity, mut player) in player_query.iter_mut() {
                for enemy_entity in enemy_query.iter() {
                    if h1.entity() == player_entity && h2.entity() == enemy_entity
                        || h2.entity() == player_entity && h1.entity() == enemy_entity
                    {
                        player.lives -= 1;
                        commands.entity(enemy_entity).despawn();
                        if player.lives <= 0 {
                            game_over_event.send(GameOverEvent);
                        }
                    }
                }
            }
        }
    }
}

fn show_terminal_toaster_notification(
    player_query: Query<&Transform, With<Player>>,
    mut toast_writer: EventWriter<ShowToast>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let right = keyboard_input.just_released(KeyCode::D);
    let left = keyboard_input.just_released(KeyCode::A);

    if let Some(player_transform) = player_query.iter().next() {
        if (right || left)
            && player_transform.translation.x > 1150.
            && player_transform.translation.x <= 1300.
        {
            let value = String::from("Press E to access console");
            toast_writer.send(ShowToast {
                value,
                duration: Duration::from_secs(3),
            });
        }
    }
}
