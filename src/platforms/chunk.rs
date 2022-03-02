use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

use crate::interactables::spawn_terminal;

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
    pub terminals: Vec<Vec2>,
}

pub fn spawn_chunk(
    chunk: &Chunk,
    commands: &mut Commands,
    rapier_config: &RapierConfiguration,
    asset_server: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlas>,
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

    for terminal_position in chunk.terminals.iter() {
        spawn_terminal(commands, asset_server, texture_atlases, terminal_position)
    }
}

pub fn chunk_test_system(
    chunks_resource: Res<ChunksResource>,
    mut commands: Commands,
    rapier_config: Res<RapierConfiguration>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let chunk_to_spawn = chunks_resource.chunks.get(&ChunkName::JumpPrelude);

    if let Some(chunk) = chunk_to_spawn {
        spawn_chunk(
            chunk,
            &mut commands,
            &rapier_config,
            &asset_server,
            &mut texture_atlases,
        );
    }
}
