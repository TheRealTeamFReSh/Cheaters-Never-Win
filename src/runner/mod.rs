use bevy::prelude::*;
mod backgroundlayer;
mod player;

pub use self::player::Player;

pub struct RunnerPlugin;

impl Plugin for RunnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(backgroundlayer::BackgroundLayerPlugin)
            .add_plugin(player::PlayerPlugin);
    }
}

pub struct CollectedChars {
    pub values: Vec<char>,
}
