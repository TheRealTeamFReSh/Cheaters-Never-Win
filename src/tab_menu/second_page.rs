use bevy::prelude::*;

use super::{TabMenuAssets, TabMenuContent};

#[derive(Component)]
pub struct SecondPageComponent;

pub fn build_ui(
    mut commands: Commands,
    assets: ResMut<TabMenuAssets>,
    query: Query<Entity, With<TabMenuContent>>,
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
        image: assets.second_page.clone().into(),
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

    let found_codes = TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: "some text".to_string(),
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

    // ---------- UI TREE CONSTRUCTION ----------//

    content
        .with_children(|parent| {
            parent.spawn_bundle(background_component);
            parent.spawn_bundle(book).with_children(|parent| {
                parent.spawn_bundle(left_page).with_children(|parent| {
                    parent.spawn_bundle(found_codes);
                });
                parent.spawn_bundle(right_page);
            });
        })
        .insert(SecondPageComponent);
}
