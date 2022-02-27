use bevy::log::*;
use bevy::prelude::*;

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ConsoleData {
            _command: String::from("my command"),
            lines: vec![String::from("hello"), String::from("world")],
        })
        .add_startup_system(setup)
        .add_system(hide_foreground)
        .add_system(update_lines_area);
    }
}

pub struct ConsoleData {
    _command: String,
    lines: Vec<String>,
}

// Debug function to hide the foreground
fn hide_foreground(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut Visibility, With<ConsoleForeground>>,
) {
    if keyboard.just_pressed(KeyCode::H) {
        if let Ok(mut visibility) = query.get_single_mut() {
            trace!("[ConsolePlugin] toggling foreground visibility");
            visibility.is_visible = !visibility.is_visible;
        } else {
            error!("[ConsolePlugin] Foreground not found!");
        }
    }
}

#[derive(Component)]
pub struct ConsoleForeground;

#[derive(Component)]
pub struct LinesArea;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, window: Res<Windows>) {
    info!("[ConsolePlugin] Enabling");

    let current_window = window.get_primary().unwrap();

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
            ..Default::default()
        },
        color: Color::rgba_u8(0, 0, 0, 0).into(),
        ..Default::default()
    };

    // crt overlay
    let foreground_component = ImageBundle {
        style: Style {
            position_type: PositionType::Absolute,
            size: Size::new(
                Val::Px(current_window.width()),
                Val::Px(current_window.height()),
            ),
            ..Default::default()
        },
        image: asset_server.load("crt.png").into(),
        ..Default::default()
    };

    // console root
    let console_component = NodeBundle {
        style: Style {
            size: Size::new(
                Val::Px(current_window.width() / 2.0),
                Val::Px(current_window.height() / 1.5),
            ),
            padding: Rect {
                left: Val::Percent(2.0),
                right: Val::Percent(2.0),
                top: Val::Percent(2.0),
                bottom: Val::Percent(2.0),
            },
            ..Default::default()
        },
        color: Color::rgb_u8(5, 17, 0).into(),
        ..Default::default()
    };

    // lines area
    let lines_area_component = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..Default::default()
        },
        color: Color::rgba_u8(0, 0, 0, 0).into(),
        ..Default::default()
    };

    // ---------- UI TREE CONSTRUCTION ----------//
    commands
        .spawn_bundle(parent_component)
        .with_children(|parent| {
            // console
            parent
                .spawn_bundle(console_component)
                .with_children(|parent| {
                    // console lines
                    parent
                        .spawn_bundle(lines_area_component)
                        .with_children(|parent| {
                            // placeholder to be populated with lines
                            parent
                                .spawn_bundle(TextBundle {
                                    ..Default::default()
                                })
                                .insert(LinesArea);
                        });
                });
            // foreground
            parent
                .spawn_bundle(foreground_component)
                .insert(ConsoleForeground);
        });

    info!("[ConsolePlugin] Enabled");
}

pub fn update_lines_area(
    data: Res<ConsoleData>,
    asset_server: Res<AssetServer>,
    mut logs_area_query: Query<&mut Text, With<LinesArea>>,
) {
    let sections = data
        .lines
        .iter()
        .flat_map(|msg| {
            let mut msg = msg.clone();
            msg.push('\n');

            IntoIterator::into_iter([TextSection {
                value: msg.clone(),
                style: TextStyle {
                    font: asset_server.load("fonts/VT323-Regular.ttf"),
                    font_size: 16.,
                    color: Color::rgba_u8(76, 207, 76, 255),
                },
            }])
        })
        .collect::<Vec<_>>();

    let mut text = logs_area_query.single_mut();
    text.sections = sections;
}
