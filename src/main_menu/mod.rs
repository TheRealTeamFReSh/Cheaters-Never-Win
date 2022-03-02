use bevy::prelude::*;

use crate::pause_menu::button;
use crate::states::GameStates;
mod ui;

#[derive(Component)]
pub struct MainMenuEntity;

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        // on enter
        app.add_system_set(SystemSet::on_enter(GameStates::MainMenu).with_system(ui::build_ui));
        // on update
        app.add_system_set(
            SystemSet::on_update(GameStates::MainMenu).with_system(button::button_system),
        );
        // on exit
        app.add_system_set(SystemSet::on_exit(GameStates::MainMenu).with_system(destroy_menu));
    }
}

fn destroy_menu(mut commands: Commands, query: Query<Entity, With<MainMenuEntity>>) {
    info!("[MainMenuPlugin] Destroying state entities before exiting...");
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    info!("[MainMenuPlugin] Exiting state");
}
