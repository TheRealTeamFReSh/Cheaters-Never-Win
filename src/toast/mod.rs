use std::time::Duration;

use bevy::prelude::*;
use bevy_ninepatch::*;
use bevy_tweening::{
    lens::UiPositionLens, Animator, Delay, EaseFunction, Sequence, Tween, TweeningPlugin,
    TweeningType,
};

use crate::cheat_codes::CheatCodeResource;
use crate::runner::CollectedChars;

pub struct ShowToast {
    pub value: String,
    pub duration: Duration,
}

#[derive(Component)]
pub struct ToastComponent;

#[derive(Component)]
pub struct ToastContentComponent;

pub struct ToastQueueResource {
    queue: Vec<ShowToast>,
}

pub struct ToastPlugin;
impl Plugin for ToastPlugin {
    fn build(&self, app: &mut App) {
        // always here so no need for a systemset
        app.add_startup_system(build_ui);
        app.add_system(update_content)
            .add_system(test)
            .add_system(display_queue);
        app.add_event::<ShowToast>();
        app.add_plugin(NinePatchPlugin::<()>::default());
        app.add_plugin(TweeningPlugin);
        app.insert_resource(ToastQueueResource { queue: Vec::new() });
    }
}

fn get_toast_animation(duration: Duration) -> Sequence<Style> {
    let close_animation = Tween::new(
        EaseFunction::CubicInOut,
        TweeningType::Once,
        std::time::Duration::from_secs(1),
        UiPositionLens {
            start: Rect {
                left: Val::Auto,
                top: Val::Px(10.),
                right: Val::Px(10.),
                bottom: Val::Auto,
            },
            end: Rect {
                left: Val::Auto,
                top: Val::Px(-100.),
                right: Val::Px(10.),
                bottom: Val::Auto,
            },
        },
    );

    let delay = Delay::new(duration);

    let open_animation = Tween::new(
        EaseFunction::CubicInOut,
        TweeningType::Once,
        std::time::Duration::from_secs_f32(0.5),
        UiPositionLens {
            end: Rect {
                left: Val::Auto,
                top: Val::Px(10.),
                right: Val::Px(10.),
                bottom: Val::Auto,
            },
            start: Rect {
                left: Val::Auto,
                top: Val::Px(-100.),
                right: Val::Px(10.),
                bottom: Val::Auto,
            },
        },
    );

    open_animation.then(delay.then(close_animation))
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
                top: Val::Px(-100.),
                ..Default::default()
            },
            margin: Rect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            size: Size::new(Val::Px(400.), Val::Px(60.)),
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
    commands
        .spawn_bundle(background)
        .insert(ToastComponent)
        .insert(Animator::<Style>::default());
}

fn update_content(
    mut show_toast_ev: EventReader<ShowToast>,
    mut toast_queue: ResMut<ToastQueueResource>,
) {
    for toast_ev in show_toast_ev.iter() {
        // anti spam
        let matching_toast = toast_queue
            .queue
            .iter()
            .filter(|toast| toast.value.eq(&toast_ev.value))
            .collect::<Vec<_>>();

        if matching_toast.is_empty() {
            info!("New toast detected");
            toast_queue.queue.insert(
                0,
                ShowToast {
                    value: toast_ev.value.clone(),
                    duration: toast_ev.duration,
                },
            );
        }
    }
}

fn display_queue(
    mut toast_queue: ResMut<ToastQueueResource>,
    mut anim_query: Query<&mut Animator<Style>, With<ToastComponent>>,
    mut text_query: Query<&mut Text, With<ToastContentComponent>>,
) {
    let mut animator = anim_query.get_single_mut().unwrap();
    let mut text = text_query.get_single_mut().unwrap();

    // if finished
    if (animator.progress() == 0.0 || animator.progress() == 1.0) && !toast_queue.queue.is_empty() {
        let next_toast = toast_queue.queue.pop().unwrap();
        text.sections[0].value = next_toast.value.clone();
        animator.set_tweenable(get_toast_animation(next_toast.duration));
        animator.rewind();
    }
}

fn test(
    keyboard: Res<Input<KeyCode>>,
    mut toast_writer: EventWriter<ShowToast>,
    collected_chars: ResMut<CollectedChars>,
    cheat_codes: ResMut<CheatCodeResource>,
) {
    if keyboard.just_pressed(KeyCode::T) && keyboard.pressed(KeyCode::LControl) {
        for (kind, code) in &cheat_codes.codes {
            for i in 0..code.text.len() {
                let ch = &code.text.chars().nth(i).unwrap();

                if !collected_chars.values.contains(ch) {
                    break;
                }

                if i == code.text.len() - 1 {
                    let value = format!("[{:?}]: {}", kind, code.text);
                    toast_writer.send(ShowToast {
                        value,
                        duration: Duration::from_secs(5),
                    });
                }
            }
        }
    }
}
