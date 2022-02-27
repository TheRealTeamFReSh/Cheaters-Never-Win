use bevy::prelude::*;

mod input;
mod ui;

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ConsoleData {
            input: String::from("my command"),
            lines: vec![String::from("hello"), String::from("world")],
        })
        .add_startup_system(ui::build_ui)
        .add_system(ui::hide_foreground)
        .add_system(input::handle_input_keys)
        .add_system(update_input_area)
        .add_system(update_lines_area);
    }
}

pub struct ConsoleData {
    input: String,
    lines: Vec<String>,
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
