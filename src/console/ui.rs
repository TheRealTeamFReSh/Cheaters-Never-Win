use bevy::prelude::*;
use bevy_rapier2d::na::DimNameDiff;

use super::ConsoleAssets;

use crate::cheat_codes::CheatCodeResource;
use crate::runner::{CollectedChars, LETTERS};

// Components
#[derive(Component)]
pub struct ConsoleForeground;

#[derive(Component)]
pub struct LinesArea;

#[derive(Component)]
pub struct CommandInput;

// Debug function to hide the foreground
pub fn hide_foreground(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut Visibility, With<ConsoleForeground>>,
) {
    if keyboard.just_pressed(KeyCode::H)
        && (keyboard.pressed(KeyCode::LControl) || keyboard.pressed(KeyCode::RControl))
    {
        if let Ok(mut visibility) = query.get_single_mut() {
            trace!("[ConsolePlugin] toggling foreground visibility");
            visibility.is_visible = !visibility.is_visible;
        } else {
            error!("[ConsolePlugin] Foreground not found!");
        }
    }
}

// building the UI of the console
pub fn build_ui(
    mut commands: Commands,
    console_assets: Res<ConsoleAssets>,
    window: Res<Windows>,
    cheat_codes_res: Res<CheatCodeResource>,
    collected_chars_res: Res<CollectedChars>,
    assets: Res<AssetServer>,
) {
    info!("[ConsolePlugin] Building console UI");

    let current_window = window.get_primary().unwrap();

    // ---------- UI COMPONENTS ----------//

    // root component
    let parent_component = NodeBundle {
        style: Style {
            size: Size::new(
                Val::Px(current_window.width()),
                Val::Px(current_window.height()),
            ),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        color: Color::rgba_u8(0, 0, 0, 0).into(),
        ..Default::default()
    };

    // crt overlay
    let foreground_component = ImageBundle {
        style: Style {
            position_type: PositionType::Absolute,
            size: Size::new(
                Val::Px(current_window.width()),
                Val::Px(current_window.height()),
            ),
            ..Default::default()
        },
        image: console_assets.overlay.clone().into(),
        ..Default::default()
    };

    // console root
    let console_component = NodeBundle {
        style: Style {
            size: Size::new(
                Val::Px(current_window.width() / 2.0),
                Val::Px(current_window.height() / 1.5),
            ),
            padding: Rect {
                left: Val::Percent(2.0),
                right: Val::Percent(2.0),
                top: Val::Percent(2.0),
                bottom: Val::Percent(2.0),
            },
            flex_direction: FlexDirection::ColumnReverse,
            overflow: Overflow::Hidden,
            ..Default::default()
        },
        color: Color::rgb_u8(5, 17, 0).into(),
        ..Default::default()
    };

    // lines area
    let lines_container_component = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..Default::default()
        },
        color: Color::rgba_u8(0, 0, 0, 0).into(),
        ..Default::default()
    };
    let lines_component = TextBundle {
        ..Default::default()
    };

    // command container
    let command_container_component = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Px(20.0)),
            flex_wrap: FlexWrap::Wrap,
            ..Default::default()
        },
        color: Color::rgba_u8(0, 0, 0, 0).into(),
        ..Default::default()
    };
    let command_component = TextBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Px(20.0)),
            flex_wrap: FlexWrap::Wrap,
            ..Default::default()
        },
        ..Default::default()
    };

    // Displaying cheat codes and collected letter in margins.
    // TODO: REFACTOR!! Duplicated code from second_page.rs
    let cheat_codes_content_container = NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            size: Size::new(Val::Px(230.0), Val::Px(200.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            position: Rect {
                left: Val::Px(3.),
                top: Val::Px(100.),
                bottom: Val::Px(200.),
                ..Default::default()
            },
            ..Default::default()
        },
        color: Color::rgba(0.125, 0.125, 0.125, 0.8).into(),
        ..Default::default()
    };
    // Component for cheat codes
    let sections = cheat_codes_res
        .codes
        .values()
        .map(|code| TextSection {
            value: format!("{:?}: {}\n", code.kind, code.text.to_lowercase()),
            style: TextStyle {
                color: Color::WHITE,
                font_size: 20.,
                font: assets.load("fonts/VT323-Regular.ttf"),
                ..Default::default()
            },
            ..Default::default()
        })
        .collect::<Vec<TextSection>>();
    let found_codes_component = TextBundle {
        text: Text {
            sections,
            ..Default::default()
        },
        style: Style {
            margin: Rect {
                left: Val::Px(10.),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    };

    let collected_letters_content_container = NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            size: Size::new(Val::Px(200.0), Val::Px(500.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            position: Rect {
                left: Val::Px(1060.),
                top: Val::Px(100.),
                bottom: Val::Px(60.),
                ..Default::default()
            },
            ..Default::default()
        },
        color: Color::rgba(0.125, 0.125, 0.125, 0.8).into(),
        ..Default::default()
    };

    let mut collected_chars_section: Vec<TextSection> = Vec::new();
    for i in 0..LETTERS.len() {
        let c = LETTERS[i];

        if let Some(count) = collected_chars_res.values_map.get(&c) {
            let section = TextSection {
                value: format!("{}: {}\n", c, count),
                style: TextStyle {
                    font: assets.load("fonts/VT323-Regular.ttf"),
                    color: Color::WHITE,
                    font_size: 25.,
                },
                ..Default::default()
            };
            collected_chars_section.push(section);
        }
    }

    // Divide into chunks
    let mut section_1: Vec<TextSection> = Vec::new();
    let mut section_2: Vec<TextSection> = Vec::new();

    // Create first section
    for i in 0..18 {
        section_1.push(collected_chars_section[i].clone());
    }
    let collected_letters_1 = TextBundle {
        text: Text {
            sections: section_1,
            ..Default::default()
        },
        style: Style {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            align_content: AlignContent::Center,
            flex_wrap: FlexWrap::Wrap,
            ..Default::default()
        },
        ..Default::default()
    };

    // Create second section
    for i in 18..36 {
        section_2.push(collected_chars_section[i].clone());
    }
    let collected_letters_2 = TextBundle {
        text: Text {
            sections: section_2,
            ..Default::default()
        },
        style: Style {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            align_content: AlignContent::Center,
            flex_wrap: FlexWrap::Wrap,
            margin: Rect {
                left: Val::Px(30.),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    };

    // ---------- UI TREE CONSTRUCTION ----------//
    commands
        .spawn_bundle(parent_component)
        .with_children(|parent| {
            // console
            parent
                .spawn_bundle(console_component)
                .with_children(|parent| {
                    // console lines
                    parent
                        .spawn_bundle(lines_container_component)
                        .with_children(|parent| {
                            // placeholder to be populated with lines
                            parent.spawn_bundle(lines_component).insert(LinesArea);
                        });
                    // console command input
                    parent
                        .spawn_bundle(command_container_component)
                        .with_children(|parent| {
                            // placeholder to be populated with the command input
                            parent.spawn_bundle(command_component).insert(CommandInput);
                        });
                });
            // foreground
            parent
                .spawn_bundle(foreground_component)
                .insert(ConsoleForeground);
            //displaying cheat codes in left margin
            parent
                .spawn_bundle(cheat_codes_content_container)
                .with_children(|parent| {
                    parent.spawn_bundle(found_codes_component);
                });
            //displaying collected letters in right margin
            parent
                .spawn_bundle(collected_letters_content_container)
                .with_children(|parent| {
                    parent.spawn_bundle(collected_letters_1);
                    parent.spawn_bundle(collected_letters_2);
                });
        })
        .insert(super::ConsoleStateEntity);

    info!("[ConsolePlugin] UI constructed");
}
