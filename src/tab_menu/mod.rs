use bevy::prelude::*;
use bevy_loading::{prelude::AssetsLoading, LoadingPlugin};

use crate::{camera::UICameraComponent, states::GameStates};

pub struct TabMenuPlugin;
impl Plugin for TabMenuPlugin {
    fn build(&self, app: &mut App) {
        // assets loading
        app.add_plugin(LoadingPlugin {
            loading_state: GameStates::TabMenuLoading,
            next_state: GameStates::TabMenu,
        });

        app.add_system_set(
            SystemSet::on_enter(GameStates::TabMenuLoading).with_system(load_assets),
        );

        // open menu trigger
        app.add_system_set(SystemSet::on_update(GameStates::Main).with_system(open_menu_trigger));
        app.add_system_set(
            SystemSet::on_update(GameStates::TabMenu).with_system(close_menu_trigger),
        );

        // on enter
        app.add_system_set(SystemSet::on_enter(GameStates::TabMenu).with_system(build_ui));

        // on exit
        app.add_system_set(SystemSet::on_exit(GameStates::TabMenu).with_system(destroy_menu));
    }
}

#[derive(Component)]
pub struct TabMenuComponent;

fn destroy_menu(mut commands: Commands, query: Query<Entity, With<TabMenuComponent>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn open_menu_trigger(
    mut keyboard: ResMut<Input<KeyCode>>,
    mut game_state: ResMut<State<GameStates>>,
) {
    if keyboard.just_pressed(KeyCode::Tab) {
        game_state.push(GameStates::TabMenuLoading).unwrap();
        keyboard.reset(KeyCode::Tab);
    }
}

fn close_menu_trigger(
    mut keyboard: ResMut<Input<KeyCode>>,
    mut game_state: ResMut<State<GameStates>>,
) {
    if keyboard.just_pressed(KeyCode::Tab) || keyboard.just_pressed(KeyCode::Escape) {
        game_state.pop().unwrap();
        keyboard.reset(KeyCode::Tab);
        keyboard.reset(KeyCode::Escape);
    }
}

pub struct TabMenuAssets {
    background: Handle<Image>,
    font: Handle<Font>,
    icon: Handle<Image>,
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
) {
    let background = asset_server.load("open_book.png");
    loading.add(&background);

    let font = asset_server.load("fonts/OldLondon.ttf");
    loading.add(&font);

    let icon = asset_server.load("cheat_codes/jump.png");
    loading.add(&icon);

    commands.insert_resource(TabMenuAssets {
        background,
        font,
        icon,
    })
}

fn build_ui(
    mut commands: Commands,
    assets: Res<TabMenuAssets>,
    window: Res<Windows>,
    camera: Query<&Transform, With<UICameraComponent>>,
) {
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
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            position: Rect {
                left: Val::Px(camera_pos),
                ..Default::default()
            },
            ..Default::default()
        },
        color: Color::rgba_u8(0, 0, 0, 0).into(),
        ..Default::default()
    };

    let background_component = ImageBundle {
        style: Style {
            position_type: PositionType::Absolute,
            size: Size::new(
                Val::Px(current_window.width()),
                Val::Px(current_window.height()),
            ),
            ..Default::default()
        },
        image: assets.background.clone().into(),
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
                top: Val::Px(20.),
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
            align_items: AlignItems::FlexEnd,
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
                value: "Distance: 0m\n\nTime: 00:00s\n\nCodes activated: 0".to_string(),
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

    let code_icon = ImageBundle {
        image: assets.icon.clone().into(),
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

    commands
        .spawn_bundle(parent_component)
        .with_children(|parent| {
            parent.spawn_bundle(background_component);
            parent.spawn_bundle(book).with_children(|parent| {
                parent.spawn_bundle(left_page).with_children(|parent| {
                    parent.spawn_bundle(run_stats);
                });
                parent.spawn_bundle(right_page).with_children(|parent| {
                    for _ in 0..15 {
                        parent.spawn_bundle(code_icon.clone());
                    }
                });
            });
        })
        .insert(TabMenuComponent);
}
