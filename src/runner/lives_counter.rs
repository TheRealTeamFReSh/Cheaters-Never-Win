use bevy::prelude::*;

use super::Player;

#[derive(Component)]
pub struct LivesCounterComponent;

pub fn build_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(10.),
                    left: Val::Px(10.),
                    ..Default::default()
                },
                ..Default::default()
            },
            color: Color::rgba_u8(0, 0, 0, 0).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        format!("Remaining Lives: {}", 3),
                        TextStyle {
                            font: asset_server.load("fonts/SpaceMadness.ttf"),
                            font_size: 24.,
                            color: Color::rgb_u8(255, 255, 255).into(),
                        },
                        TextAlignment {
                            ..Default::default()
                        },
                    ),
                    ..Default::default()
                })
                .insert(LivesCounterComponent);
        });
}

pub fn update_counter(
    mut text_query: Query<&mut Text, With<LivesCounterComponent>>,
    player_query: Query<&Player>,
) {
    let player = player_query.iter().last().unwrap();

    if let Some(mut text) = text_query.iter_mut().last() {
        text.sections[0].value = format!("Remaining Lives: {}", player.lives);
    }
}
