use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{seq::SliceRandom, Rng};
use serde::Deserialize;

use crate::interactables::{spawn_char, spawn_terminal};

use super::platform;
use crate::{enemies, interactables, runner};

#[derive(Deserialize)]
pub struct PlatformData {
    pub platform_kind: platform::PlatformKind,
    pub position: Vec2,
}

#[derive(Deserialize)]
pub struct EnemyData {
    pub enemy_kind: enemies::EnemyKind,
    pub position: Vec2,
}

#[derive(Deserialize)]
pub struct ChunksResource {
    pub prelude_chunks: Vec<Chunk>,
    pub basic_chunks: Vec<Chunk>,
    pub jump_chunks: Vec<Chunk>,
    // add chunk vec for each cheat
    pub furthest_x: f32,
}

#[derive(Deserialize)]
pub struct CharTextData {
    // TODO: this value should be randomed
    pub value: char,
    pub position: Vec2,
}

#[derive(Deserialize)]
pub struct Chunk {
    pub platforms: Vec<PlatformData>,
    pub enemies: Vec<EnemyData>,
    pub next_chunk_offset: f32,
    pub chunk_offset: f32,
    // ability dependency? optional?
    pub terminals: Vec<Vec2>,
    pub chars: Vec<CharTextData>,
}

pub fn spawn_chunk(
    chunk: &Chunk,
    x_offset: f32,
    commands: &mut Commands,
    rapier_config: &RapierConfiguration,
    asset_server: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlas>,
) {
    for platform_data in chunk.platforms.iter() {
        platform::spawn_platform(
            &platform_data.platform_kind,
            platform_data.position + Vec2::new(x_offset, 0.0),
            commands,
            rapier_config,
            asset_server,
        )
    }

    for enemy_data in chunk.enemies.iter() {
        enemies::spawn_enemy(
            &enemy_data.enemy_kind,
            enemy_data.position + Vec2::new(x_offset, 0.0),
            commands,
            rapier_config,
            asset_server,
        )
    }

    for terminal_position in chunk.terminals.iter() {
        spawn_terminal(commands, asset_server, texture_atlases, terminal_position)
    }

    for char_data in chunk.chars.iter() {
        spawn_char(
            commands,
            asset_server,
            texture_atlases,
            char_data.value,
            &char_data.position,
        )
    }
}

/// Test spawn platform
#[allow(dead_code)]
pub fn chunk_test_system(
    chunks_resource: Res<ChunksResource>,
    mut commands: Commands,
    rapier_config: Res<RapierConfiguration>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let chunk_to_spawn = chunks_resource.prelude_chunks.get(0);

    if let Some(chunk) = chunk_to_spawn {
        spawn_chunk(
            chunk,
            0.0,
            &mut commands,
            &rapier_config,
            &asset_server,
            &mut texture_atlases,
        );
    }
}

pub fn generate_prelude_chunk(
    mut commands: Commands,
    rapier_config: Res<RapierConfiguration>,
    asset_server: Res<AssetServer>,
    mut chunks_resource: ResMut<ChunksResource>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if chunks_resource.furthest_x <= 0.0 {
        let chunk_to_spawn = chunks_resource
            .prelude_chunks
            .choose(&mut rand::thread_rng())
            .unwrap();

        spawn_chunk(
            chunk_to_spawn,
            0.0,
            &mut commands,
            &rapier_config,
            &asset_server,
            &mut texture_atlases,
        );
        chunks_resource.furthest_x = chunk_to_spawn.next_chunk_offset;
    }
}

pub fn generate_chunks(
    mut commands: Commands,
    rapier_config: Res<RapierConfiguration>,
    asset_server: Res<AssetServer>,
    mut chunks_resource: ResMut<ChunksResource>,
    player_query: Query<(&runner::Player, &RigidBodyPositionComponent)>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    assert!(chunks_resource.furthest_x >= 0.0);

    for (_player, rb_pos) in player_query.iter() {
        if chunks_resource.furthest_x - (rb_pos.position.translation.x * rapier_config.scale)
            < 2000.0
        {
            info!("generating next chunks");
            for _ in 0..=4 {
                // roll for "cheat chunk" (chunk that requires an acquired cheat to get past)
                let mut cheat_chunk_roll = rand::thread_rng();
                let chunk_to_spawn = if cheat_chunk_roll.gen_range(0..=3) == 0 {
                    // TODO: check player's activated cheats
                    chunks_resource
                        .jump_chunks
                        .choose(&mut rand::thread_rng())
                        .unwrap()
                } else {
                    chunks_resource
                        .basic_chunks
                        .choose(&mut rand::thread_rng())
                        .unwrap()
                };

                spawn_chunk(
                    chunk_to_spawn,
                    chunks_resource.furthest_x + chunk_to_spawn.chunk_offset,
                    &mut commands,
                    &rapier_config,
                    &asset_server,
                    &mut texture_atlases,
                );

                chunks_resource.furthest_x += chunk_to_spawn.next_chunk_offset;
            }
        }
    }
}

pub fn despawn_platforms(
    mut commands: Commands,
    rapier_config: Res<RapierConfiguration>,
    player_query: Query<&RigidBodyPositionComponent, With<runner::Player>>,
    platform_query: Query<(Entity, &RigidBodyPositionComponent), With<platform::Platform>>,
) {
    for player_rb_pos in player_query.iter() {
        for (platform_entity, platform_rb_pos) in platform_query.iter() {
            if (player_rb_pos.position.translation.x * rapier_config.scale)
                - (platform_rb_pos.position.translation.x * rapier_config.scale)
                > 10000.0
            {
                info!("despawning platform");
                commands.entity(platform_entity).despawn_recursive();
            }
        }
    }
}

pub fn despawn_enemies(
    mut commands: Commands,
    rapier_config: Res<RapierConfiguration>,
    player_query: Query<&RigidBodyPositionComponent, With<runner::Player>>,
    enemy_query: Query<(Entity, &RigidBodyPositionComponent), With<enemies::Enemy>>,
) {
    for player_rb_pos in player_query.iter() {
        for (enemy_entity, enemy_rb_pos) in enemy_query.iter() {
            if (player_rb_pos.position.translation.x * rapier_config.scale)
                - (enemy_rb_pos.position.translation.x * rapier_config.scale)
                > 10000.0
            {
                info!("despawning enemy");
                commands.entity(enemy_entity).despawn_recursive();
            }
        }
    }
}

pub fn despawn_interactables(
    mut commands: Commands,
    rapier_config: Res<RapierConfiguration>,
    player_query: Query<&RigidBodyPositionComponent, With<runner::Player>>,
    interactable_query: Query<(Entity, &Transform), With<interactables::InteractableComponent>>,
) {
    for player_rb_pos in player_query.iter() {
        for (interactable_entity, interactable_transform) in interactable_query.iter() {
            if (player_rb_pos.position.translation.x * rapier_config.scale)
                - interactable_transform.translation.x
                > 10000.0
            {
                info!("despawning interactable");
                commands.entity(interactable_entity).despawn_recursive();
            }
        }
    }
}
