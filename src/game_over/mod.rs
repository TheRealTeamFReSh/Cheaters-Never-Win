use bevy::{app::AppExit, prelude::*};

use crate::{
    pause_menu::button::{UIButton, HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON},
    states::GameStates,
};

use self::ui::GameOverScreenComponent;

pub struct GameOverPlugin;

mod ui;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameStates::GameOver).with_system(ui::build_ui));
        app.add_system_set(SystemSet::on_update(GameStates::GameOver).with_system(button_handler));
        app.add_system_set(SystemSet::on_exit(GameStates::GameOver).with_system(ui_destroyer));

        app.add_system_set(
            SystemSet::on_update(GameStates::Main).with_system(open_gameover_screen),
        );
    }
}

fn open_gameover_screen(
    mut keyboard: ResMut<Input<KeyCode>>,
    mut game_state: ResMut<State<GameStates>>,
) {
    if keyboard.just_pressed(KeyCode::G) {
        game_state.push(GameStates::GameOver).unwrap();
        keyboard.reset(KeyCode::G);
    }
}

fn ui_destroyer(mut commands: Commands, query: Query<Entity, With<GameOverScreenComponent>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
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
                    "restart" => {
                        game_state.set(GameStates::Main).unwrap();
                    }
                    "quit" => {
                        //game_state.set(GameStates::MainMenu).unwrap();
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
