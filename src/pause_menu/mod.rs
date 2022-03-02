use bevy::prelude::*;

use crate::states::GameStates;

pub mod button;
mod ui;

#[derive(Component)]
pub struct PauseMenuEntity;

pub struct PauseMenuPlugin;
impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        // on enter
        app.add_system_set(SystemSet::on_enter(GameStates::PauseMenu).with_system(ui::build_ui));
        // on update
        app.add_system_set(SystemSet::on_update(GameStates::Main).with_system(open_pause_menu));
        app.add_system_set(
            SystemSet::on_update(GameStates::PauseMenu)
                .with_system(close_pause_menu)
                .with_system(button::button_system),
        );
        // on exit
        app.add_system_set(SystemSet::on_exit(GameStates::PauseMenu).with_system(destroy_menu));
    }
}

fn destroy_menu(mut commands: Commands, query: Query<Entity, With<PauseMenuEntity>>) {
    info!("[PauseMenuPlugin] Destroying state entities before exiting...");
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    info!("[PauseMenuPlugin] Exiting state");
}

fn open_pause_menu(
    mut keyboard: ResMut<Input<KeyCode>>,
    mut game_state: ResMut<State<GameStates>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        game_state.push(GameStates::PauseMenu).unwrap();
        keyboard.reset(KeyCode::Escape);
    }
}

fn close_pause_menu(
    mut keyboard: ResMut<Input<KeyCode>>,
    mut game_state: ResMut<State<GameStates>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        game_state.pop().unwrap();
        keyboard.reset(KeyCode::Escape);
    }
}
