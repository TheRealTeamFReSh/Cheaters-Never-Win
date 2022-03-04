use std::time::Duration;

use crate::enemies::Enemy;
use crate::{camera::TwoDCameraComponent, effects, physics, platforms, states::GameStates};
use bevy::math::Vec3Swizzles;
use bevy::{prelude::*, render::camera::Camera};
use bevy_kira_audio::{Audio, AudioChannel};
use bevy_rapier2d::prelude::*;
use rand::Rng;
use std::collections::HashMap;

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
    pub jump_count: u8,
    pub dash_input_timer: Timer,
    pub dash_cooldown_timer: Timer,
    pub dash_input_count: u8,
    pub is_dashing: bool,
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
        let mut collected_chars_list = CollectedChars {
            values: Vec::new(),
            values_map: HashMap::new(),
        };
        collected_chars_list.initialize_map();
        app.insert_resource(collected_chars_list)
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
                dash_attack: AnimationData {
                    length: 8,
                    offset: 32,
                },
                run_step_counter: 0,
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
            .add_system_set(SystemSet::on_exit(GameStates::Main).with_system(despawn_character))
            .add_system_set(
                SystemSet::on_update(GameStates::Main)
                    .with_system(follow_player_camera)
                    .with_system(animate_sprite)
                    .with_system(move_character)
                    .after("player_feet")
                    .with_system(detect_char_interactable)
                    .with_system(player_collide_enemy)
                    .with_system(player_fall_damage)
                    .with_system(detect_cheat_code_activation)
                    .with_system(show_terminal_toaster_notification),
            );
    }
}

fn despawn_character(
    mut commands: Commands,
    query: Query<Entity, Or<(With<Player>, With<RigidBodyPositionComponent>)>>,
) {
    for ent in query.iter() {
        warn!("Despawning player");
        commands.entity(ent).despawn_recursive();
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
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(71.0, 67.0), 8, 5);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let player = Player {
        speed: 8.0,
        lives: 6,
        acceleration: 0.12,
        deceleration: 0.1,
        feet_touching_platforms: FeetTouchingPlatforms { platforms: vec![] },
        jump_count: 0,
        dash_input_timer: Timer::from_seconds(0.25, false),
        dash_cooldown_timer: Timer::from_seconds(1.5, false),
        dash_input_count: 1,
        is_dashing: false,
    };

    let collider_size_hx = 30.0 / rapier_config.scale / 2.0;
    let collider_size_hy = 70.0 / rapier_config.scale / 2.0;

    commands
        .spawn_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Dynamic.into(),
            mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
            position: Vec2::new(0.0, 300.0 / rapier_config.scale).into(),
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
                restitution: 0.1,
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
    pub dash_attack: AnimationData,
    pub run_step_counter: u32,
}

pub struct AnimationData {
    pub length: usize,
    pub offset: usize,
}

pub fn animate_sprite(
    time: Res<Time>,
    mut player_animation_resource: ResMut<PlayerAnimationResource>,
    mut player_query: Query<(&mut Player, &RigidBodyVelocityComponent)>,
    mut query: Query<(&mut PlayerAnimationTimer, &mut TextureAtlasSprite)>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    rapier_config: Res<RapierConfiguration>,
) {
    for (mut player, rb_vel) in player_query.iter_mut() {
        for (mut timer, mut sprite) in query.iter_mut() {
            timer.0.tick(time.delta());
            if timer.0.just_finished() {
                if player.is_dashing {
                    if sprite.index
                        == player_animation_resource.dash_attack.offset
                            + player_animation_resource.dash_attack.length
                            - 1
                    {
                        player.is_dashing = false;
                    }
                    if sprite.index < player_animation_resource.dash_attack.offset
                        || sprite.index
                            >= (player_animation_resource.dash_attack.length
                                + player_animation_resource.dash_attack.offset)
                    {
                        sprite.index = player_animation_resource.dash_attack.offset;
                        let audio_channel = AudioChannel::new("ability-channel".to_owned());
                        audio.set_volume_in_channel(5.0, &audio_channel);
                        audio.play_in_channel(asset_server.load("dash.ogg"), &audio_channel);
                    } else if sprite.index
                        < (player_animation_resource.dash_attack.length
                            + player_animation_resource.dash_attack.offset)
                            - 1
                    {
                        sprite.index += 1;
                    }
                } else if player.feet_touching_platforms.platforms.is_empty() {
                    // player is jumping
                    if sprite.index < player_animation_resource.jump.offset
                        || sprite.index
                            >= (player_animation_resource.jump.length
                                + player_animation_resource.jump.offset)
                    {
                        sprite.index = player_animation_resource.jump.offset;
                        let audio_channel = AudioChannel::new("movement-channel".to_owned());
                        audio.set_volume_in_channel(10.0, &audio_channel);
                        audio.play_in_channel(asset_server.load("jump.ogg"), &audio_channel);
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
                            player_animation_resource.run_step_counter = 0;
                        } else {
                            sprite.index =
                                (sprite.index + 1) % player_animation_resource.run_right.length;
                            player_animation_resource.run_step_counter += 1;
                        }
                        let audio_channel = AudioChannel::new("movement-channel".to_owned());
                        if player_animation_resource.run_step_counter % 3 == 0 {
                            audio.set_volume_in_channel(
                                15.0 * (rb_vel.linvel.x.abs()
                                    / (player.speed * rapier_config.scale)),
                                &audio_channel,
                            );
                            audio.play_in_channel(
                                asset_server.load(
                                    format!(
                                        "footsteps/{}.ogg",
                                        rand::thread_rng().gen_range(0..10)
                                    )
                                    .as_str(),
                                ),
                                &audio_channel,
                            );
                        }
                    } else if rb_vel.linvel.x < 0.0 {
                        //player is running left
                        if sprite.index < player_animation_resource.run_left.offset
                            || sprite.index
                                >= (player_animation_resource.run_left.length
                                    + player_animation_resource.run_left.offset)
                        {
                            sprite.index = player_animation_resource.run_left.offset;
                            player_animation_resource.run_step_counter = 0;
                        } else {
                            sprite.index = ((sprite.index + 1)
                                % player_animation_resource.run_left.length)
                                + player_animation_resource.run_left.offset;
                            player_animation_resource.run_step_counter += 1;
                        }
                        let audio_channel = AudioChannel::new("movement-channel".to_owned());
                        if player_animation_resource.run_step_counter % 3 == 0 {
                            audio.set_volume_in_channel(
                                15.0 * (rb_vel.linvel.x.abs()
                                    / (player.speed * rapier_config.scale)),
                                &audio_channel,
                            );
                            audio.play_in_channel(
                                asset_server.load(
                                    format!(
                                        "footsteps/{}.ogg",
                                        rand::thread_rng().gen_range(0..10)
                                    )
                                    .as_str(),
                                ),
                                &audio_channel,
                            );
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
        &mut Player,
        &mut RigidBodyVelocityComponent,
        &RigidBodyMassPropsComponent,
    )>,
    mut animation_query: Query<&mut TextureAtlasSprite, With<PlayerAnimationTimer>>,
    player_animation_resource: Res<PlayerAnimationResource>,
    cheat_codes: ResMut<CheatCodeResource>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    time: Res<Time>,
) {
    for (mut player, mut rb_vel, rb_mprops) in query.iter_mut() {
        // update acceleration value
        if cheat_codes.is_code_activated(&CheatCodeKind::SpeedBoost3) {
            player.acceleration = 0.15;
            player.deceleration = 0.4;
            player.speed = 8.9;
        } else if cheat_codes.is_code_activated(&CheatCodeKind::SpeedBoost2) {
            player.acceleration = 0.14;
            player.deceleration = 0.3;
            player.speed = 8.6;
        } else if cheat_codes.is_code_activated(&CheatCodeKind::SpeedBoost1) {
            player.acceleration = 0.13;
            player.deceleration = 0.2;
            player.speed = 8.3;
        }

        let _up = keyboard_input.pressed(KeyCode::W);
        let _down = keyboard_input.pressed(KeyCode::S);
        let right = keyboard_input.pressed(KeyCode::D);
        let dash = keyboard_input.just_released(KeyCode::D);

        let jump = cheat_codes.is_code_activated(&CheatCodeKind::Jump)
            && keyboard_input.just_released(KeyCode::Space)
            && !player.feet_touching_platforms.platforms.is_empty()
            || (cheat_codes.is_code_activated(&CheatCodeKind::DoubleJump)
                && keyboard_input.just_released(KeyCode::Space));

        let left = cheat_codes.is_code_activated(&CheatCodeKind::MoveLeft)
            && keyboard_input.pressed(KeyCode::A);

        let x_axis = -(left as i8) + right as i8;

        if dash && cheat_codes.is_code_activated(&CheatCodeKind::Dash) {
            if player.dash_input_count == 0 {
                player.dash_input_count = 1;
                player.dash_input_timer.reset();
            } else if player.dash_input_count == 1 && player.dash_cooldown_timer.finished() {
                rb_vel.apply_impulse(rb_mprops, Vec2::new(1000.0, 0.0).into());
                player.is_dashing = true;
                player.dash_cooldown_timer.reset()
            }
        }

        if player.dash_input_count == 1 {
            player.dash_input_timer.tick(time.delta());
            if player.dash_input_timer.just_finished() {
                player.dash_input_count = 0;
            }
        }
        if !player.is_dashing {
            //decrease dash cooldown
            player.dash_cooldown_timer.tick(time.delta());

            if x_axis != 0 {
                rb_vel.linvel.x += player.acceleration * (x_axis as f32) * rapier_config.scale;
                if rb_vel.linvel.x.abs() > player.speed * rapier_config.scale {
                    rb_vel.linvel.x = (rb_vel.linvel.x / rb_vel.linvel.x.abs())
                        * player.speed
                        * rapier_config.scale;
                }
            } else if rb_vel.linvel.x.abs() > 0.4 {
                // decelerate
                rb_vel.linvel.x -= player.deceleration
                    * (rb_vel.linvel.x / rb_vel.linvel.x.abs())
                    * rapier_config.scale;
            } else {
                rb_vel.linvel.x = 0.0;
            }
        } else {
            rb_vel.linvel.y = 0.0;
            rb_vel.linvel.x += player.acceleration * 2.0;
        }

        if jump {
            if !player.feet_touching_platforms.platforms.is_empty() {
                // single jump
                physics::jump(1500.0, &mut rb_vel, rb_mprops);
                if cheat_codes.is_code_activated(&CheatCodeKind::DoubleJump) {
                    player.jump_count = 1;
                } else {
                    player.jump_count = 0;
                }
            } else if player.jump_count == 1 {
                // double jump
                rb_vel.linvel.y = 0.0;
                physics::jump(1500.0, &mut rb_vel, rb_mprops);
                for mut sprite in animation_query.iter_mut() {
                    sprite.index = player_animation_resource.jump.offset;
                }
                let audio_channel = AudioChannel::new("movement-channel".to_owned());
                audio.set_volume_in_channel(10.0, &audio_channel);
                audio.play_in_channel(asset_server.load("jump.ogg"), &audio_channel);
                player.jump_count = 0;
            }
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

                        let char_entry = collected_chars.values_map.get(&char_component.value);
                        if let Some(_count) = char_entry {
                            *collected_chars
                                .values_map
                                .get_mut(&char_component.value)
                                .unwrap() += 1;
                        }

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
    mut game_state: ResMut<State<GameStates>>,
) {
    for (mut player, transform) in player_query.iter_mut() {
        if transform.translation.y < -400.0 {
            player.lives = 0;
            game_over_event.send(GameOverEvent);
            info!("Fell down hole");
            game_state.push(GameStates::GameOver).unwrap();
        }
    }
}

pub fn player_collide_enemy(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Player)>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    mut contact_events: EventReader<ContactEvent>,
    mut game_over_event: EventWriter<GameOverEvent>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut game_state: ResMut<State<GameStates>>,
) {
    for contact_event in contact_events.iter() {
        if let ContactEvent::Started(h1, h2) = contact_event {
            for (player_entity, mut player) in player_query.iter_mut() {
                for (enemy_entity, enemy_transform) in enemy_query.iter() {
                    if h1.entity() == player_entity && h2.entity() == enemy_entity
                        || h2.entity() == player_entity && h1.entity() == enemy_entity
                    {
                        if !player.is_dashing {
                            player.lives -= 1;
                        }
                        commands.entity(enemy_entity).despawn();
                        // spawn explostion
                        effects::spawn_explosion(
                            enemy_transform.translation.xy(),
                            &mut commands,
                            &asset_server,
                            &mut texture_atlases,
                        );
                        let audio_channel = AudioChannel::new("explosion-channel".to_owned());
                        audio.set_volume_in_channel(0.6, &audio_channel);
                        audio.play_in_channel(asset_server.load("explosion.ogg"), &audio_channel);
                        if player.lives <= 0 {
                            game_over_event.send(GameOverEvent);
                            game_state.push(GameStates::GameOver).unwrap();
                        }
                    }
                }
            }
        }
    }
}

pub fn detect_cheat_code_activation(
    mut query: Query<&mut Player>,
    mut cheat_codes: ResMut<CheatCodeResource>,
) {
    for mut player in query.iter_mut() {
        if cheat_codes.is_code_activated(&CheatCodeKind::ExtraLife) {
            player.lives += 1;
            print!("Player has {} lives", player.lives);
            cheat_codes.deactivate_code(&CheatCodeKind::ExtraLife);
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
