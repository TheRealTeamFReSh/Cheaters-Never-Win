use crate::states::GameStates;
use bevy::prelude::*;
use ron::de::from_bytes;
pub struct PlatformsPlugin;

mod chunk;
mod platform;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(
            from_bytes::<chunk::ChunksResource>(include_bytes!("../../data/chunks.ron")).unwrap(),
        )
        .add_system_set(
            SystemSet::on_enter(GameStates::Main).with_system(
                chunk::generate_prelude_chunk
                    .after("setup_physics")
                    .label("generate_prelude_chunk"),
            ),
        )
        .add_system_set(
            SystemSet::on_update(GameStates::Main)
                .with_system(chunk::generate_chunks.after("generate_prelude_chunk"))
                .with_system(chunk::despawn_platforms.after("generate_prelude_chunk"))
                .with_system(chunk::despawn_enemies.after("generate_prelude_chunk"))
                .with_system(chunk::despawn_interactables.after("generate_prelude_chunk")),
        );
    }
}
