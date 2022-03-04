use bevy::prelude::*;
mod backgroundlayer;
mod player;

use std::collections::HashMap;

pub use self::player::Player;

pub struct RunnerPlugin;

impl Plugin for RunnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(backgroundlayer::BackgroundLayerPlugin)
            .add_plugin(player::PlayerPlugin);
    }
}

const LETTERS: [char; 36] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0',
];
pub struct CollectedChars {
    pub values: Vec<char>,
    pub values_map: HashMap<char, u32>,
}

impl CollectedChars {
    pub fn initialize_map(&mut self) {
        for c in LETTERS {
            self.values_map.insert(c, 0);
        }
    }
}
