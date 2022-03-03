use bevy::prelude::*;

use crate::{
    cheat_codes::{CheatCodeKind, CheatCodeResource},
    stats::GameStatsResource,
};

use super::{TabMenuAssets, TabMenuContent};

#[derive(Component)]
pub struct FirstPageComponent;

pub fn build_ui(
    mut commands: Commands,
    assets: ResMut<TabMenuAssets>,
    query: Query<Entity, With<TabMenuContent>>,
    cheat_codes_res: Res<CheatCodeResource>,
    stats_res: Res<GameStatsResource>,
    window: Res<Windows>,
) {
    let current_window = window.get_primary().unwrap();

    // despawning previous content
    let content_entity = query.single();
    let mut content = commands.entity(content_entity);
    content.despawn_descendants();

    // ---------- UI COMPONENTS ----------//

    let background_component = ImageBundle {
        style: Style {
            position_type: PositionType::Absolute,
            size: Size::new(
                Val::Px(current_window.width()),
                Val::Px(current_window.height()),
            ),
            ..Default::default()
        },
        image: assets.first_page.clone().into(),
        ..Default::default()
    };

    let book = NodeBundle {
        style: Style {
            size: Size::new(Val::Px(720.), Val::Px(610.)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        color: Color::rgba_u8(0, 0, 0, 0).into(),
        ..Default::default()
    };

    let left_page = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(50.), Val::Percent(60.)),
            justify_content: JustifyContent::Center,
            align_self: AlignSelf::FlexEnd,
            align_items: AlignItems::Center,
            position: Rect {
                top: Val::Px(30.),
                ..Default::default()
            },
            ..Default::default()
        },
        color: Color::rgba_u8(0, 0, 0, 0).into(),
        ..Default::default()
    };

    let right_page = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(50.), Val::Percent(60.)),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            align_content: AlignContent::FlexStart,
            flex_wrap: FlexWrap::WrapReverse,
            position: Rect {
                top: Val::Px(20.),
                ..Default::default()
            },
            ..Default::default()
        },
        color: Color::rgba_u8(0, 0, 0, 0).into(),
        ..Default::default()
    };

    let run_stats = TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: format!(
                    "Score: {}\n\nDistance: {:.2}m\n\nTime: {}\n\nAvg speed: {:.2}m/s\n\nCodes activated: {}/{}",
                    stats_res.get_score(),
                    stats_res.distance,
                    format_time(stats_res.run_time),
                    stats_res.avg_speed,
                    stats_res.cheats_activated,
                    cheat_codes_res.codes.len()
                ),
                style: TextStyle {
                    font: assets.font.clone(),
                    color: Color::rgb_u8(74, 28, 33).into(),
                    font_size: 20.,
                },
                ..Default::default()
            }],
            ..Default::default()
        },
        ..Default::default()
    };

    let code_icon = |kind: &CheatCodeKind| ImageBundle {
        image: assets.icons.get(kind).unwrap().clone().into(),
        style: Style {
            size: Size::new(Val::Px(48.), Val::Auto),
            margin: Rect {
                right: Val::Px(5.),
                bottom: Val::Px(5.),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    };

    // ---------- UI TREE CONSTRUCTION ----------//

    content
        .with_children(|parent| {
            parent.spawn_bundle(background_component);
            parent.spawn_bundle(book).with_children(|parent| {
                parent.spawn_bundle(left_page).with_children(|parent| {
                    parent.spawn_bundle(run_stats);
                });
                parent.spawn_bundle(right_page).with_children(|parent| {
                    for kind in cheat_codes_res.codes.keys() {
                        if cheat_codes_res.is_code_activated(kind) {
                            parent.spawn_bundle(code_icon(kind).clone());
                        }
                    }
                });
            });
        })
        .insert(FirstPageComponent);
}

fn format_time(duration: f64) -> String {
    let seconds = duration as usize % 60;
    let minutes = duration as usize / 60;
    format!("{:02}:{:02}", minutes, seconds)
}
