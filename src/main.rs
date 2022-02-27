use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

mod console;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            resizable: false,
            height: 720.,
            width: 1280.,
            title: "Bevy Jam #1".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(console::ConsolePlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    trace!("Setting up cameras");
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
