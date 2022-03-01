use bevy::prelude::*;
use bevy_ninepatch::*;

pub struct ShowToast(String);

#[derive(Component)]
pub struct ToastContentComponent;

pub struct ToastPlugin;
impl Plugin for ToastPlugin {
    fn build(&self, app: &mut App) {
        // always here so no need for a systemset
        app.add_startup_system(build_ui);
        app.add_system(update_content);
        app.add_event::<ShowToast>();
        app.add_plugin(NinePatchPlugin::<()>::default());
    }
}

fn build_ui(
    mut commands: Commands,
    mut nine_patches: ResMut<Assets<NinePatchBuilder>>,
    asset_server: Res<AssetServer>,
) {
    // Texture for the base image
    let panel_texture_handle: Handle<Image> = asset_server.load("toast_background.png");

    // Create a basic 9-Patch UI element with margins of 20 pixels
    let nine_patch_handle = nine_patches.add(NinePatchBuilder::by_margins(20, 20, 20, 20));

    // This entity will be placed in the center of the 9-Patch UI element
    let content_entity = commands
        .spawn_bundle(TextBundle {
            text: Text::with_section(
                "This is a toast. Hello, world!",
                TextStyle {
                    font: asset_server.load("fonts/VT323-Regular.ttf"),
                    font_size: 24.,
                    color: Color::rgb_u8(255, 255, 255).into(),
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    vertical: VerticalAlign::Center,
                },
            ),
            ..Default::default()
        })
        .insert(ToastContentComponent)
        .id();

    // UI component
    let background = NinePatchBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                right: Val::Px(10.),
                top: Val::Px(10.),
                ..Default::default()
            },
            margin: Rect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            size: Size::new(Val::Px(400.), Val::Px(80.)),
            ..Default::default()
        },
        nine_patch_data: NinePatchData::with_single_content(
            panel_texture_handle,
            nine_patch_handle,
            content_entity,
        ),
        ..Default::default()
    };

    // Building UI tree
    commands.spawn_bundle(background);
}

fn update_content(
    mut show_toast_ev: EventReader<ShowToast>,
    mut query: Query<&mut Text, With<ToastContentComponent>>,
) {
    for ShowToast(content) in show_toast_ev.iter() {
        let mut text = query.get_single_mut().unwrap();
        text.sections[0].value = content.clone();
    }
}
