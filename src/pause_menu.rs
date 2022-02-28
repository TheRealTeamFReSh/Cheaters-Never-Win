use bevy::prelude::*;
use bevy_ninepatch::{NinePatchBuilder, NinePatchBundle, NinePatchData, NinePatchPlugin};

use crate::states::GameStates;

#[derive(Component)]
pub struct PauseMenuEntity;

pub struct PauseMenuPlugin;
impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(NinePatchPlugin::<()>::default());

        // on enter
        app.add_system_set(SystemSet::on_enter(GameStates::PauseMenu).with_system(build_ui));
        // on update
        app.add_system_set(SystemSet::on_update(GameStates::Main).with_system(open_pause_menu));
        app.add_system_set(
            SystemSet::on_update(GameStates::PauseMenu).with_system(close_pause_menu),
        );
        // on exit
        app.add_system_set(SystemSet::on_exit(GameStates::PauseMenu).with_system(destroy_menu));
    }
}

fn destroy_menu(mut commands: Commands, query: Query<Entity, With<PauseMenuEntity>>) {
    info!("[PauseMenuPlugin] Destroying state entities before exiting...");
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    info!("[PauseMenuPlugin] Exiting state");
}

fn open_pause_menu(
    mut keyboard: ResMut<Input<KeyCode>>,
    mut game_state: ResMut<State<GameStates>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        game_state.push(GameStates::PauseMenu).unwrap();
        keyboard.reset(KeyCode::Escape);
    }
}

fn close_pause_menu(
    mut keyboard: ResMut<Input<KeyCode>>,
    mut game_state: ResMut<State<GameStates>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        game_state.pop().unwrap();
        keyboard.reset(KeyCode::Escape);
    }
}

// building the UI of the console
pub fn build_ui(
    mut commands: Commands,
    window: Res<Windows>,
    camera: Query<&Transform>,
    mut nine_patches: ResMut<Assets<NinePatchBuilder>>,
    asset_server: Res<AssetServer>,
) {
    info!("[PauseMenuPlugin] Building console UI");

    let background_texture_handle: Handle<Image> = asset_server.load("ui_background.png");
    let background_nine_patch_handle =
        nine_patches.add(NinePatchBuilder::by_margins(20, 20, 20, 20));

    let font_handle: Handle<Font> = asset_server.load("fonts/HateYourWriting.ttf");

    let current_window = window.get_primary().unwrap();
    let mut camera_pos = 0.0;
    for transform in camera.iter() {
        camera_pos = transform.translation.x;
    }

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
            position: Rect {
                left: Val::Px(camera_pos),
                ..Default::default()
            },
            ..Default::default()
        },
        color: Color::rgba_u8(0, 0, 0, 150).into(),
        ..Default::default()
    };

    // background
    let background = NinePatchBundle {
        style: Style {
            position_type: PositionType::Absolute,
            margin: Rect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            size: Size::new(Val::Px(500.), Val::Px(300.)),
            ..Default::default()
        },
        nine_patch_data: NinePatchData {
            nine_patch: background_nine_patch_handle,
            texture: background_texture_handle,
            ..Default::default()
        },
        ..Default::default()
    };

    // pause title
    let pause_title = TextBundle {
        text: Text {
            sections: vec![TextSection {
                style: TextStyle {
                    font: font_handle.clone(),
                    font_size: 64.,
                    color: Color::rgb_u8(205, 205, 205).into(),
                    ..Default::default()
                },
                value: "Pause".to_string(),
                ..Default::default()
            }],
            alignment: TextAlignment {
                horizontal: HorizontalAlign::Center,
                ..Default::default()
            },
        },
        ..Default::default()
    };

    // ---------- UI TREE CONSTRUCTION ----------//

    commands
        .spawn_bundle(parent_component)
        .with_children(|parent| {
            parent.spawn_bundle(background);
            parent.spawn_bundle(pause_title);
        })
        .insert(PauseMenuEntity);

    info!("[PauseMenuPlugin] UI constructed");
}
