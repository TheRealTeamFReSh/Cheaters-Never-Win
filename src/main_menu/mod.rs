use bevy::{app::AppExit, prelude::*};

use crate::{
    pause_menu::button::{UIButton, HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON},
    states::GameStates,
};
mod ui;

#[derive(Component)]
pub struct MainMenuEntity;

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        // on enter
        app.add_system_set(SystemSet::on_enter(GameStates::MainMenu).with_system(ui::build_ui));
        // on update
        app.add_system_set(SystemSet::on_update(GameStates::MainMenu).with_system(button_handler));
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

pub fn button_handler(
    mut interaction_query: Query<(&Interaction, &mut UiColor, &UIButton), Changed<Interaction>>,
    mut game_state: ResMut<State<GameStates>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, mut color, button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                match button.name.as_str() {
                    "play" => {
                        game_state.set(GameStates::Main).unwrap();
                    }
                    "quit" => {
                        exit.send(AppExit);
                    }
                    _ => {}
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
