use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

mod cheat_codes;
mod console;
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
        .insert_resource(cheat_codes::CheatCodeResource::new())
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(console::ConsolePlugin)
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

    println!(
        "Random code : {}",
        cheat_codes::generate_random_code(cheat_codes::CheatCodeRarity::Legendary)
    );
}
