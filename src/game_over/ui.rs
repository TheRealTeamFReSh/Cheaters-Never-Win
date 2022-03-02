use bevy::prelude::*;

use crate::camera::UICameraComponent;

pub fn build_ui(
    mut commands: Commands,
    window: Res<Windows>,
    camera: Query<&Transform, With<UICameraComponent>>,
    asset_server: Res<AssetServer>,
) {
    let current_window = window.get_primary().unwrap();
    let mut camera_pos = 0.0;
    for transform in camera.iter() {
        camera_pos = transform.translation.x;
    }

    // UI comps
    let parent_component = NodeBundle {
        style: Style {
            size: Size::new(
                Val::Px(current_window.width()),
                Val::Px(current_window.height()),
            ),
            flex_direction: FlexDirection::ColumnReverse,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            position: Rect {
                left: Val::Px(camera_pos),
                ..Default::default()
            },
            ..Default::default()
        },
        color: Color::rgba_u8(50, 2, 2, 200).into(),
        ..Default::default()
    };

    let game_over_text = TextBundle {
        text: Text::with_section(
            "Game Over",
            TextStyle {
                font_size: 96.,
                font: asset_server.load("fonts/VT323-Regular.ttf"),
                color: Color::rgb_u8(220, 220, 220).into(),
            },
            TextAlignment {
                ..Default::default()
            },
        ),
        ..Default::default()
    };

    let score_text = TextBundle {
        text: Text::with_section(
            format!("Your score : {}", 0),
            TextStyle {
                font_size: 48.,
                font: asset_server.load("fonts/VT323-Regular.ttf"),
                color: Color::rgb_u8(220, 220, 220).into(),
            },
            TextAlignment {
                ..Default::default()
            },
        ),
        ..Default::default()
    };

    // building tree
    commands
        .spawn_bundle(parent_component)
        .with_children(|parent| {
            parent.spawn_bundle(game_over_text);
            parent.spawn_bundle(score_text);
        });
}
