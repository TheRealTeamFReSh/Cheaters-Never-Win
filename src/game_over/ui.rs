use bevy::prelude::*;

use crate::{camera::UICameraComponent, pause_menu::button::UIButton, stats::GameStatsResource};

#[derive(Component)]
pub struct GameOverScreenComponent;

pub fn build_ui(
    mut commands: Commands,
    window: Res<Windows>,
    camera: Query<&Transform, With<UICameraComponent>>,
    stats_res: Res<GameStatsResource>,
    asset_server: Res<AssetServer>,
) {
    let font_handle = asset_server.load("fonts/VT323-Regular.ttf");

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
                font: font_handle.clone(),
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
            format!("Your score : {}", stats_res.get_score()),
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

    let btn_group = NodeBundle {
        style: Style {
            margin: Rect {
                top: Val::Px(100.),
                ..Default::default()
            },
            flex_direction: FlexDirection::ColumnReverse,
            justify_content: JustifyContent::SpaceBetween,
            ..Default::default()
        },
        color: Color::rgba_u8(0, 0, 0, 0).into(),
        ..Default::default()
    };

    let restart_btn = UIButton::new(
        "Restart".to_string(),
        font_handle.clone(),
        "restart".to_string(),
    );
    let quit_btn = UIButton::new(
        "Main menu".to_string(),
        font_handle.clone(),
        "main_menu".to_string(),
    );

    // building tree
    commands
        .spawn_bundle(parent_component)
        .with_children(|parent| {
            parent.spawn_bundle(game_over_text);
            parent.spawn_bundle(score_text);
            parent.spawn_bundle(btn_group).with_children(|parent| {
                restart_btn.spawn(parent);
                quit_btn.spawn(parent);
            });
        })
        .insert(GameOverScreenComponent);
}
