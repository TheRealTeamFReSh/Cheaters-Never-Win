use bevy::prelude::*;

use crate::states::GameStates;

pub struct GameOverPlugin;

mod ui;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameStates::GameOver).with_system(ui::build_ui));
    }
}
