use bevy::log::*;
use bevy::prelude::*;

mod console;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(console::ConsolePlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    trace!("Setting up cameras");
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
