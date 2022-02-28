use crate::states::GameStates;
use bevy::prelude::*;

#[derive(Component)]
pub struct LoadingScreenEntities;

pub struct LoadingScreenPlugin;
impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameStates::ConsoleLoading).with_system(build_ui));
        app.add_system_set(SystemSet::on_exit(GameStates::ConsoleLoading).with_system(destroy_ui));
    }
}

fn destroy_ui(mut commands: Commands, query: Query<Entity, With<LoadingScreenEntities>>) {
    info!("[LoadingScreenPlugin] Destroying state entities before exiting...");
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    info!("[LoadingScreenPlugin] Exiting state");
}

fn build_ui(
    mut commands: Commands,
    window: Res<Windows>,
    camera: Query<&Transform>,
    asset_server: Res<AssetServer>,
) {
    info!("[LoadingScreenPlugin] Building loading screen");

    // setting the initial position of the window
    let current_window = window.get_primary().unwrap();
    let mut camera_pos = 0.0;
    for transform in camera.iter() {
        camera_pos = transform.translation.x;
    }

    let background = NodeBundle {
        style: Style {
            size: Size::new(
                Val::Px(current_window.width()),
                Val::Px(current_window.height()),
            ),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::ColumnReverse,
            position: Rect {
                left: Val::Px(camera_pos),
                ..Default::default()
            },
            ..Default::default()
        },
        color: Color::rgba_u8(0, 0, 0, 255).into(),
        ..Default::default()
    };

    let loading_text = TextBundle {
        style: Style {
            flex_direction: FlexDirection::ColumnReverse,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        text: Text {
            sections: vec![
                TextSection {
                    style: TextStyle {
                        font: asset_server.load("fonts/VT323-Regular.ttf"),
                        font_size: 64.,
                        color: Color::rgba_u8(211, 211, 207, 255),
                    },
                    value: "[Booting up for the first time]\n".to_string(),
                },
                TextSection {
                    style: TextStyle {
                        font: asset_server.load("fonts/VT323-Regular.ttf"),
                        font_size: 48.,
                        color: Color::rgba_u8(211, 211, 207, 255),
                    },
                    value: "Can take some time ...".to_string(),
                },
            ],
            alignment: TextAlignment {
                horizontal: HorizontalAlign::Center,
                ..Default::default()
            },
        },
        ..Default::default()
    };

    commands
        .spawn_bundle(background)
        .with_children(|parent| {
            parent.spawn_bundle(loading_text);
        })
        .insert(LoadingScreenEntities);

    info!("[LoadingScreenPlugin] UI constructed");
}
