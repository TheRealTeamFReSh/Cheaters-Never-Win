use bevy::{app::AppExit, prelude::*};
use bevy_kira_audio::{Audio, AudioChannel};
use bevy_ninepatch::NinePatchPlugin;

use crate::states::GameStates;

use self::button::{UIButton, HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON};

pub mod button;
mod ui;

#[derive(Component)]
pub struct PauseMenuEntity;

pub struct PauseMenuPlugin;
impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(NinePatchPlugin::<()>::default());

        // on enter
        app.add_system_set(SystemSet::on_enter(GameStates::PauseMenu).with_system(ui::build_ui));
        // on update
        app.add_system_set(SystemSet::on_update(GameStates::Main).with_system(open_pause_menu));
        app.add_system_set(
            SystemSet::on_update(GameStates::PauseMenu)
                .with_system(close_pause_menu)
                .with_system(button_handler),
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

pub fn button_handler(
    mut interaction_query: Query<(&Interaction, &mut UiColor, &UIButton), Changed<Interaction>>,
    mut game_state: ResMut<State<GameStates>>,
    mut exit: EventWriter<AppExit>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for (interaction, mut color, button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                let audio_channel = AudioChannel::new("sfx-channel".to_owned());
                audio.set_volume_in_channel(5.0, &audio_channel);
                audio.play_in_channel(asset_server.load("button.ogg"), &audio_channel);
                *color = PRESSED_BUTTON.into();
                match button.name.as_str() {
                    "resume" => {
                        game_state.pop().unwrap();
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
