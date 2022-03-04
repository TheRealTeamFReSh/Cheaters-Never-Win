use bevy::prelude::*;
use bevy_ninepatch::{NinePatchBuilder, NinePatchBundle, NinePatchData};

use crate::pause_menu::button::UIButton;

// building the UI of the console
pub fn build_ui(
    mut commands: Commands,
    window: Res<Windows>,
    mut nine_patches: ResMut<Assets<NinePatchBuilder>>,
    asset_server: Res<AssetServer>,
) {
    info!("[MainMenuPlugin] Building console UI");

    let background_nine_patch_handle =
        nine_patches.add(NinePatchBuilder::by_margins(20, 20, 20, 20));

    let font_handle: Handle<Font> = asset_server.load("fonts/HateYourWriting.ttf");

    let current_window = window.get_primary().unwrap();

    // ---------- UI COMPONENTS ----------//

    // root component
    let parent_component = NodeBundle {
        style: Style {
            size: Size::new(
                Val::Px(current_window.width()),
                Val::Px(current_window.height()),
            ),
            flex_direction: FlexDirection::ColumnReverse,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        color: Color::rgba_u8(0, 0, 0, 150).into(),
        ..Default::default()
    };

    // container
    let container = NodeBundle {
        style: Style {
            position_type: PositionType::Relative,
            margin: Rect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::ColumnReverse,
            size: Size::new(Val::Px(500.), Val::Px(400.)),
            ..Default::default()
        },
        color: Color::rgba(0., 0., 0., 0.).into(),
        ..Default::default()
    };

    // background
    let background = NinePatchBundle {
        style: Style {
            position_type: PositionType::Absolute,
            margin: Rect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            size: Size::new(Val::Px(500.), Val::Px(400.)),
            ..Default::default()
        },
        nine_patch_data: NinePatchData {
            nine_patch: background_nine_patch_handle,
            ..Default::default()
        },
        ..Default::default()
    };

    // main title
    let pause_title = TextBundle {
        text: Text {
            sections: vec![TextSection {
                style: TextStyle {
                    font: font_handle.clone(),
                    font_size: 64.,
                    color: Color::rgb_u8(205, 205, 205).into(),
                    ..Default::default()
                },
                value: "Cheaters Never Win".to_string(),
                ..Default::default()
            }],
            alignment: TextAlignment {
                horizontal: HorizontalAlign::Center,
                ..Default::default()
            },
        },
        ..Default::default()
    };

    let play_btn = UIButton::new("Play".to_string(), font_handle.clone(), "play".to_string());
    let options_btn = UIButton::new(
        "Options".to_string(),
        font_handle.clone(),
        "options".to_string(),
    );
    let quit_btn = UIButton::new("Quit".to_string(), font_handle.clone(), "quit".to_string());

    // ---------- UI TREE CONSTRUCTION ----------//

    commands
        .spawn_bundle(parent_component)
        .with_children(|parent| {
            parent.spawn_bundle(background);
            parent.spawn_bundle(container).with_children(|parent| {
                parent.spawn_bundle(pause_title);
                play_btn.spawn(parent);
                options_btn.spawn(parent);
                quit_btn.spawn(parent);
            });
        })
        .insert(super::MainMenuEntity);

    info!("[MainMenuPlugin] UI constructed");
}
