use crate::states::GameStates;
use bevy::prelude::*;

use self::event::{PrintToConsoleEvent, SendCommandEvent};

mod commands;
mod event;
mod input;
mod ui;
mod utils;

#[derive(Component)]
pub struct ConsoleStateEntity;

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ConsoleData {
            input: String::from(""),
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
                .with_system(update_lines_area),
        )
        .add_system_set(
            SystemSet::on_update(GameStates::Console)
                .with_system(input::handle_input_keys)
                .with_system(commands::command_handler)
                .with_system(event::add_message_events_to_console),
        )
        // on exit
        .add_system_set(
            SystemSet::on_exit(GameStates::Console).with_system(destroy_console_state_entities),
        );
    }
}

pub struct ConsoleData {
    input: String,
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

fn close_console_handler(keyboard: Res<Input<KeyCode>>, mut game_state: ResMut<State<GameStates>>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        game_state.pop().unwrap();
    }
}

pub fn update_lines_area(
    data: Res<ConsoleData>,
    asset_server: Res<AssetServer>,
    mut lines_area_query: Query<&mut Text, With<ui::LinesArea>>,
) {
    let sections_text = data.lines.join("\n");
    let sections = vec![TextSection {
        value: sections_text,
        style: TextStyle {
            font: asset_server.load("fonts/VT323-Regular.ttf"),
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
    asset_server: Res<AssetServer>,
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
            font: asset_server.load("fonts/VT323-Regular.ttf"),
            font_size: 16.,
            color: Color::rgba_u8(102, 255, 102, 255),
        },
    });
}
