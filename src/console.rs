use bevy::log::*;
use bevy::prelude::*;

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(hide_foreground);
    }
}

// Debug function to hide the foreground
fn hide_foreground(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut Visibility, With<ConsoleForeground>>,
) {
    if keyboard.just_pressed(KeyCode::H) {
        if let Ok(mut visibility) = query.get_single_mut() {
            info!("found");
            visibility.is_visible = !visibility.is_visible;
        } else {
            error!("[ConsolePlugin] Foreground not found!");
        }
    }
}

#[derive(Component)]
pub struct ConsoleForeground;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, window: Res<Windows>) {
    info!("[ConsolePlugin] Enabling");

    let current_window = window.get_primary().unwrap();

    // UI components

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
                Val::Px(current_window.height() / 2.0),
            ),
            ..Default::default()
        },
        color: Color::rgb_u8(5, 17, 0).into(),
        ..Default::default()
    };

    // building the UI with the components
    commands
        .spawn_bundle(parent_component)
        .with_children(|parent| {
            parent.spawn_bundle(console_component);
            parent
                .spawn_bundle(foreground_component)
                .insert(ConsoleForeground);
        });

    info!("[ConsolePlugin] Enabled");
}
