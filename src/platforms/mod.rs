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
            SystemSet::on_enter(GameStates::Main)
                .with_system(chunk::generate_prelude_chunk.after("setup_physics")),
        );
    }
}
