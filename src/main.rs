use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

mod console;
mod physics;
mod platforms;
mod runner;
mod states;

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
        .add_plugin(runner::RunnerPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(physics::PhysicsPlugin)
        .add_plugin(platforms::PlatformsPlugin)
        .add_state(states::GameStates::Main)
        .add_startup_system(setup)
        // handling console state change
        .add_system_set(SystemSet::on_update(states::GameStates::Main).with_system(open_console))
        .run();
}

fn open_console(keyboard: Res<Input<KeyCode>>, mut game_state: ResMut<State<states::GameStates>>) {
    if keyboard.just_pressed(KeyCode::E) {
        game_state.push(states::GameStates::Console).unwrap();
    }
}

fn setup(mut commands: Commands) {
    info!("Setting up cameras");
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
