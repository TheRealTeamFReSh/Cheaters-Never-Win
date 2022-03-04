use bevy::{app::AppExit, prelude::*};
use bevy_kira_audio::{Audio, AudioChannel};

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
        app.add_system_set(
            SystemSet::on_enter(GameStates::MainMenu)
                .with_system(ui::build_ui)
                .with_system(start_automation_audio),
        );
        // on update
        app.add_system_set(SystemSet::on_update(GameStates::MainMenu).with_system(button_handler));
        // on exit
        app.add_system_set(
            SystemSet::on_exit(GameStates::MainMenu)
                .with_system(destroy_menu)
                .with_system(start_gameplay_audio),
        );
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

fn start_automation_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.stop();
    audio.play_looped(asset_server.load("automation.ogg"));
}

fn start_gameplay_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.stop();
    audio.play_looped(asset_server.load("cyberpunk_moonlight_sonata.ogg"));
}
