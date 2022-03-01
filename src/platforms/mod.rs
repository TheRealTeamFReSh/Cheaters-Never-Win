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
        .add_startup_system(chunk::chunk_test_system.after("setup_physics"));
    }
}
