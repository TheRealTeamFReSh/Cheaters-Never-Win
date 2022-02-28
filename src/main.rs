use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use cheat_codes::CheatCodeResource;

mod cheat_codes;
mod console;
mod states;

mod runner;

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
        .add_plugin(runner::RunnerPlugin)
        .add_state(states::GameStates::Main)
        .add_startup_system(setup)
        // TODO: remove
        .add_startup_system(test_codes)
        // handling console state change
        .add_system_set(SystemSet::on_update(states::GameStates::Main).with_system(open_console))
        .run();
}

fn open_console(keyboard: Res<Input<KeyCode>>, mut game_state: ResMut<State<states::GameStates>>) {
    if keyboard.just_pressed(KeyCode::E) {
        game_state.push(states::GameStates::ConsoleLoading).unwrap();
    }
}

fn setup(mut commands: Commands) {
    info!("Setting up cameras");
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn test_codes(mut cheat_codes_res: ResMut<CheatCodeResource>) {
    println!(
        "Random text : {}",
        cheat_codes::generate_random_code(cheat_codes::CheatCodeRarity::Legendary)
    );

    let next_code = cheat_codes_res.get_next_code();
    println!("Get next cheat code : {:?}", next_code);

    println!(
        "Is code activated: {}",
        cheat_codes_res.is_code_activated(&next_code)
    );

    let result = cheat_codes_res.activate_code("jump");
    println!("Trying to activate code : {:?}", &result);
}
