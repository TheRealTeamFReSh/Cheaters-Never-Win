use bevy::prelude::*;

use crate::cheat_codes::CheatCodeResource;
use crate::runner::{CollectedChars, LETTERS};

use super::{TabMenuAssets, TabMenuContent};

#[derive(Component)]
pub struct SecondPageComponent;

pub fn build_ui(
    mut commands: Commands,
    assets: ResMut<TabMenuAssets>,
    query: Query<Entity, With<TabMenuContent>>,
    cheat_codes_res: Res<CheatCodeResource>,
    collected_chars_res: Res<CollectedChars>,
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
                top: Val::Px(125.),
                ..Default::default()
            },
            ..Default::default()
        },
        color: Color::rgba_u8(0, 0, 0, 0).into(),
        ..Default::default()
    };

    let right_page = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(50.), Val::Px(100.)),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            align_content: AlignContent::Center,
            flex_wrap: FlexWrap::Wrap,
            position: Rect {
                top: Val::Px(170.),
                ..Default::default()
            },
            ..Default::default()
        },
        color: Color::rgba_u8(0, 0, 0, 0).into(),
        ..Default::default()
    };

    let sections = cheat_codes_res
        .codes
        .values()
        .map(|code| TextSection {
            value: format!("{:?}: {}\n", code.kind, code.text.to_lowercase()),
            style: TextStyle {
                font: assets.font_2.clone(),
                color: Color::rgb_u8(74, 28, 33).into(),
                font_size: 20.,
            },
            ..Default::default()
        })
        .collect::<Vec<TextSection>>();

    let found_codes = TextBundle {
        text: Text {
            sections,
            ..Default::default()
        },
        ..Default::default()
    };

    // TODO: refactor creating page for collected letters
    let mut collected_chars_section: Vec<TextSection> = Vec::new();
    for i in 0..LETTERS.len() {
        let c = LETTERS[i];

        if let Some(count) = collected_chars_res.values_map.get(&c) {
            let section = TextSection {
                value: format!("{}: {}\n", c, count),
                style: TextStyle {
                    font: assets.font_2.clone(),
                    color: Color::rgb_u8(74, 28, 33).into(),
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
            position: Rect {
                left: Val::Px(30.),
                ..Default::default()
            },
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
            position: Rect {
                left: Val::Px(130.),
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
                    parent.spawn_bundle(found_codes);
                });
                parent.spawn_bundle(right_page).with_children(|parent| {
                    parent.spawn_bundle(collected_letters_1);
                    parent.spawn_bundle(collected_letters_2);
                });
            });
        })
        .insert(SecondPageComponent);
}
