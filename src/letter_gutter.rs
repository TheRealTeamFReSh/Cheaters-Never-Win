use bevy::prelude::*;
use std::time::Duration;

use crate::toast::ShowToast;
use crate::{runner::CollectedChars, states::GameStates};

#[derive(Component)]
pub struct GutterComponent;

#[derive(Component)]
pub struct GutterUIComponent;

pub struct LetterGutterPlugin;

impl Plugin for LetterGutterPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameStates::Main).with_system(build_ui));
        app.add_system_set(SystemSet::on_update(GameStates::Main).with_system(update_gutter));
        app.add_system_set(SystemSet::on_exit(GameStates::Main).with_system(gutter_destructor));
    }
}

fn gutter_destructor(mut commands: Commands, query: Query<Entity, With<GutterUIComponent>>) {
    for ent in query.iter() {
        commands.entity(ent).despawn_recursive();
    }
}

pub fn build_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server.load("fonts/212 Keyboard.otf");

    // UI comps
    let parent_component = NodeBundle {
        style: Style {
            size: Size::new(Val::Px(500.), Val::Px(100.)),
            flex_direction: FlexDirection::ColumnReverse,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            position_type: PositionType::Absolute,
            position: Rect {
                left: Val::Px(10.),
                bottom: Val::Px(-10.),
                ..Default::default()
            },
            ..Default::default()
        },
        color: Color::rgba_u8(0, 0, 0, 0).into(),
        ..Default::default()
    };

    let gutter_background = ImageBundle {
        image: asset_server.load("gutter.png").into(),
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                bottom: Val::Px(10.),
                left: Val::Px(10.),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    };

    let text_container = TextBundle {
        text: Text::with_section(
            "",
            TextStyle {
                font_size: 48.,
                font: font_handle.clone(),
                color: Color::rgb_u8(220, 220, 220).into(),
            },
            TextAlignment {
                ..Default::default()
            },
        ),
        style: Style {
            position: Rect {
                left: Val::Px(21.),
                top: Val::Px(22.),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    };

    // building tree
    commands
        .spawn_bundle(parent_component)
        .with_children(|parent| {
            parent.spawn_bundle(gutter_background);
            parent.spawn_bundle(text_container).insert(GutterComponent);
        })
        .insert(GutterUIComponent);
}

fn update_gutter(
    collected_chars: Res<CollectedChars>,
    mut query: Query<&mut Text, With<GutterComponent>>,
    asset_server: Res<AssetServer>,
    mut toast_writer: EventWriter<ShowToast>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let font_handle = asset_server.load("fonts/212 Keyboard.otf");

    for mut gutter_text in query.iter_mut() {
        let sections: Vec<TextSection> = collected_chars
            .values
            .iter()
            .map(|char| TextSection {
                value: char.to_string().to_uppercase(),
                style: TextStyle {
                    font: font_handle.clone(),
                    font_size: 64.,
                    color: Color::rgb_u8(220, 220, 220),
                },
            })
            .collect();

        if sections.len() > 10 {
            gutter_text.sections = sections[sections.len() - 10..sections.len()].to_vec();
            /*
            // Show a notification for user to view book to see a list of all letters
            let right = keyboard_input.just_released(KeyCode::D);
            let left = keyboard_input.just_released(KeyCode::A);
            let value = String::from("See book to view all collected letters");

            if right || left {
                toast_writer.send(ShowToast {
                    value,
                    duration: Duration::from_secs(3),
                });
            }
            */
        } else {
            gutter_text.sections = sections;
        }
    }
}
