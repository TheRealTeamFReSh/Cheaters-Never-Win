use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::seq::SliceRandom;
use serde::Deserialize;

use super::platform;
use crate::enemies;

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
pub struct Chunk {
    pub platforms: Vec<PlatformData>,
    pub enemies: Vec<EnemyData>,
    pub next_chunk_offset: f32,
    pub chunk_offset: f32,
    // ability dependency? optional?
}

pub fn spawn_chunk(
    chunk: &Chunk,
    commands: &mut Commands,
    rapier_config: &RapierConfiguration,
    asset_server: &AssetServer,
) {
    for platform_data in chunk.platforms.iter() {
        platform::spawn_platform(
            &platform_data.platform_kind,
            platform_data.position,
            commands,
            rapier_config,
            asset_server,
        )
    }

    for enemy_data in chunk.enemies.iter() {
        enemies::spawn_enemy(
            &enemy_data.enemy_kind,
            enemy_data.position,
            commands,
            rapier_config,
            asset_server,
        )
    }
}

pub fn chunk_test_system(
    chunks_resource: Res<ChunksResource>,
    mut commands: Commands,
    rapier_config: Res<RapierConfiguration>,
    asset_server: Res<AssetServer>,
) {
    let chunk_to_spawn = chunks_resource.prelude_chunks.get(0);

    if let Some(chunk) = chunk_to_spawn {
        spawn_chunk(chunk, &mut commands, &rapier_config, &asset_server);
    }
}

pub fn generate_prelude_chunk(
    mut commands: Commands,
    rapier_config: Res<RapierConfiguration>,
    asset_server: Res<AssetServer>,
    mut chunks_resource: ResMut<ChunksResource>,
) {
    if chunks_resource.furthest_x <= 0.0 {
        let chunk_to_spawn = chunks_resource
            .prelude_chunks
            .choose(&mut rand::thread_rng())
            .unwrap();

        spawn_chunk(chunk_to_spawn, &mut commands, &rapier_config, &asset_server);
        chunks_resource.furthest_x = chunk_to_spawn.next_chunk_offset;
    }
}
