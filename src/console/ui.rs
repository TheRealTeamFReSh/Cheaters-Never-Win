use bevy::prelude::*;

// Components
#[derive(Component)]
pub struct ConsoleForeground;

#[derive(Component)]
pub struct LinesArea;

#[derive(Component)]
pub struct CommandInput;

// Debug function to hide the foreground
pub fn hide_foreground(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut Visibility, With<ConsoleForeground>>,
) {
    if keyboard.just_pressed(KeyCode::H)
        && (keyboard.pressed(KeyCode::LControl) || keyboard.pressed(KeyCode::RControl))
    {
        if let Ok(mut visibility) = query.get_single_mut() {
            trace!("[ConsolePlugin] toggling foreground visibility");
            visibility.is_visible = !visibility.is_visible;
        } else {
            error!("[ConsolePlugin] Foreground not found!");
        }
    }
}

// building the UI of the console
pub fn build_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Res<Windows>,
    camera: Query<&Transform>,
) {
    info!("[ConsolePlugin] Building console UI");

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
            flex_direction: FlexDirection::ColumnReverse,
            overflow: Overflow::Hidden,
            ..Default::default()
        },
        color: Color::rgb_u8(5, 17, 0).into(),
        ..Default::default()
    };

    // lines area
    let lines_container_component = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..Default::default()
        },
        color: Color::rgba_u8(0, 0, 0, 0).into(),
        ..Default::default()
    };
    let lines_component = TextBundle {
        ..Default::default()
    };

    // command container
    let command_container_component = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Px(20.0)),
            flex_wrap: FlexWrap::Wrap,
            ..Default::default()
        },
        color: Color::rgba_u8(0, 0, 0, 0).into(),
        ..Default::default()
    };
    let command_component = TextBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Px(20.0)),
            flex_wrap: FlexWrap::Wrap,
            ..Default::default()
        },
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
                        .spawn_bundle(lines_container_component)
                        .with_children(|parent| {
                            // placeholder to be populated with lines
                            parent.spawn_bundle(lines_component).insert(LinesArea);
                        });
                    // console command input
                    parent
                        .spawn_bundle(command_container_component)
                        .with_children(|parent| {
                            // placeholder to be populated with the command input
                            parent.spawn_bundle(command_component).insert(CommandInput);
                        });
                });
            // foreground
            parent
                .spawn_bundle(foreground_component)
                .insert(ConsoleForeground);
        })
        .insert(super::ConsoleStateEntity);

    info!("[ConsolePlugin] UI constructed");
}
