use bevy::prelude::*;

use crate::{runner::CollectedChars, states::GameStates};

#[derive(Component)]
pub struct GutterComponent;

pub struct LetterGutterPlugin;

impl Plugin for LetterGutterPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameStates::Main).with_system(build_ui));
        app.add_system_set(SystemSet::on_update(GameStates::Main).with_system(update_gutter));
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
        });
}

fn update_gutter(
    collected_chars: Res<CollectedChars>,
    mut query: Query<&mut Text, With<GutterComponent>>,
    asset_server: Res<AssetServer>,
) {
    let font_handle = asset_server.load("fonts/212 Keyboard.otf");
    let mut gutter_text = query.get_single_mut().unwrap();

    let sections = collected_chars
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

    gutter_text.sections = sections;
}
