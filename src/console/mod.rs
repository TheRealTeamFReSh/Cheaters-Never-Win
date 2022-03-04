use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel};
use bevy_loading::prelude::*;

use self::{
    event::{PrintToConsoleEvent, SendCommandEvent},
    loading_screen::LoadingScreenPlugin,
};
use crate::interactables::{InteractableComponent, InteractableType};
use crate::runner::Player;
use crate::states::GameStates;

mod commands;
mod event;
mod input;
mod loading_screen;
mod ui;
mod utils;

#[derive(Component)]
pub struct ConsoleStateEntity;

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        // assets loading
        app.add_plugin(LoadingScreenPlugin);
        app.add_plugin(LoadingPlugin {
            loading_state: GameStates::ConsoleLoading,
            next_state: GameStates::Console,
        });

        app.add_system_set(
            SystemSet::on_enter(GameStates::ConsoleLoading).with_system(load_overlay),
        );

        app.add_system_set(
            SystemSet::on_update(GameStates::Main).with_system(open_console_handler),
        );

        // plugin building
        app.insert_resource(ConsoleData {
            input: String::from(""),
            history_index: 0,
            history: Vec::new(),
            lines: utils::welcome_lines(),
        })
        .add_event::<PrintToConsoleEvent>()
        .add_event::<SendCommandEvent>()
        // on enter
        .add_system_set(SystemSet::on_enter(GameStates::Console).with_system(ui::build_ui))
        // on update
        .add_system_set(
            SystemSet::on_update(GameStates::Console)
                .with_system(close_console_handler)
                .with_system(ui::hide_foreground),
        )
        .add_system_set(
            SystemSet::on_update(GameStates::Console)
                .with_system(update_input_area)
                .with_system(update_lines_area)
                .with_system(event::add_message_events_to_console)
                .label("update_ui"),
        )
        .add_system_set(
            SystemSet::on_update(GameStates::Console)
                .with_system(input::handle_input_keys)
                .with_system(commands::command_handler)
                .after("update_ui"),
        )
        // on exit
        .add_system_set(
            SystemSet::on_exit(GameStates::Console).with_system(destroy_console_state_entities),
        );
    }
}

pub struct ConsoleAssets {
    overlay: Handle<Image>,
    crt_font: Handle<Font>,
}

fn load_overlay(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
) {
    let overlay = asset_server.load("crt.png");
    let crt_font = asset_server.load("fonts/VT323-Regular.ttf");

    loading.add(&overlay);
    loading.add(&crt_font);

    commands.insert_resource(ConsoleAssets { overlay, crt_font })
}

pub struct ConsoleData {
    input: String,
    history_index: usize,
    history: Vec<String>,
    lines: Vec<String>,
}

fn destroy_console_state_entities(
    mut commands: Commands,
    entities_query: Query<Entity, With<ConsoleStateEntity>>,
) {
    info!("[ConsolePlugin] Destroying state entities before exiting...");
    for entity in entities_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    info!("[ConsolePlugin] Exiting state");
}

fn close_console_handler(
    mut keyboard: ResMut<Input<KeyCode>>,
    mut game_state: ResMut<State<GameStates>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        game_state.pop().unwrap();
        keyboard.reset(KeyCode::Escape);
    }
}

fn open_console_handler(
    mut keyboard: ResMut<Input<KeyCode>>,
    mut game_state: ResMut<State<GameStates>>,
    player_query: Query<&Transform, With<Player>>,
    interactable_query: Query<(&InteractableComponent, &Transform)>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    if keyboard.just_released(KeyCode::E) {
        // Only open the terminal when in range
        if let Some(player) = player_query.iter().next() {
            for (interactable, transform) in interactable_query.iter() {
                match interactable.interactable_type {
                    InteractableType::Terminal => {
                        let distance_x = player.translation.x - transform.translation.x;
                        let distance_y = player.translation.y - transform.translation.y;
                        let range = interactable.range;

                        if distance_x <= range
                            && distance_x >= -range
                            && distance_y <= range
                            && distance_y >= -range
                        {
                            game_state.push(GameStates::ConsoleLoading).unwrap();
                            keyboard.reset(KeyCode::E);
                            let audio_channel = AudioChannel::new("sfx-channel".to_owned());
                            audio.set_volume_in_channel(10.0, &audio_channel);
                            audio.play_in_channel(asset_server.load("crt.ogg"), &audio_channel);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

pub fn update_lines_area(
    data: Res<ConsoleData>,
    console_assets: Res<ConsoleAssets>,
    mut lines_area_query: Query<&mut Text, With<ui::LinesArea>>,
) {
    let sections_text = data.lines.join("\n");
    let sections = vec![TextSection {
        value: sections_text,
        style: TextStyle {
            font: console_assets.crt_font.clone(),
            font_size: 16.,
            color: Color::rgba_u8(76, 207, 76, 255),
        },
    }];

    let mut text = lines_area_query.single_mut();
    text.sections = sections;
}

pub fn update_input_area(
    mut command_input_query: Query<&mut Text, With<ui::CommandInput>>,
    mut state: ResMut<ConsoleData>,
    console_assets: Res<ConsoleAssets>,
    time: Res<Time>,
) {
    let mut text = command_input_query.single_mut();
    text.sections = vec![];

    if state.input.len() > 144 {
        let trimmed_command = state.input[..144].to_string();
        state.input = trimmed_command;
    }

    let mut to_show = String::from("user@hacked_pc > ");
    to_show.push_str(&state.input);

    if (time.seconds_since_startup() * 3.0) as u64 % 2 == 0 {
        to_show.push('_');
    }

    text.sections.push(TextSection {
        value: to_show,
        style: TextStyle {
            font: console_assets.crt_font.clone(),
            font_size: 16.,
            color: Color::rgba_u8(102, 255, 102, 255),
        },
    });
}
