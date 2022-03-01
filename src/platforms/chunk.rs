use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

use super::platform;

#[derive(Deserialize)]
pub struct PlatformData {
    pub platform_kind: platform::PlatformKind,
    pub position: Vec2,
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub enum ChunkName {
    JumpPrelude,
}

#[derive(Deserialize)]
pub struct ChunksResource {
    pub chunks: HashMap<ChunkName, Chunk>,
}

#[derive(Deserialize)]
pub struct Chunk {
    pub platforms: Vec<PlatformData>,
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
}

pub fn chunk_test_system(
    chunks_resource: Res<ChunksResource>,
    mut commands: Commands,
    rapier_config: Res<RapierConfiguration>,
    asset_server: Res<AssetServer>,
) {
    let chunk_to_spawn = chunks_resource.chunks.get(&ChunkName::JumpPrelude);

    if let Some(chunk) = chunk_to_spawn {
        spawn_chunk(chunk, &mut commands, &rapier_config, &asset_server);
    }
}