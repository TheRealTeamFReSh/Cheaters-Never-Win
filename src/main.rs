use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use cheat_codes::CheatCodeResource;

mod camera;
mod cheat_codes;
mod console;
mod enemies;
mod interactables;
mod letter_gutter;
mod main_menu;
mod pause_menu;
mod physics;
mod platforms;
mod runner;
mod states;
mod stats;
mod tab_menu;
mod toast;

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
        .add_plugin(main_menu::MainMenuPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(tab_menu::TabMenuPlugin)
        .add_plugin(console::ConsolePlugin)
        .add_plugin(runner::RunnerPlugin)
        .add_plugin(pause_menu::PauseMenuPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(physics::PhysicsPlugin)
        .add_plugin(platforms::PlatformsPlugin)
        .add_plugin(enemies::EnemiesPlugin)
        .add_plugin(toast::ToastPlugin)
        .add_plugin(interactables::InteractablesPlugin)
        .add_plugin(letter_gutter::LetterGutterPlugin)
        .add_state(states::GameStates::MainMenu)
        .add_plugin(stats::GameStatsPlugin)
        .add_startup_system(camera::add_camera)
        // TODO: remove
        .add_startup_system(test_codes)
        .run();
}

fn test_codes(mut cheat_codes_res: ResMut<CheatCodeResource>) {
    println!(
        "Random text : {}",
        cheat_codes::generate_random_code(cheat_codes::CheatCodeRarity::Legendary)
    );

    let next_code = cheat_codes_res.get_next_code();
    let next_code_code = cheat_codes_res.codes.get(&next_code).unwrap();
    println!(
        "Get next cheat code: {:?} with code: {}",
        next_code, next_code_code.text
    );

    println!(
        "Is code activated: {}",
        cheat_codes_res.is_code_activated(&next_code)
    );

    let result = cheat_codes_res.activate_code("jump");
    println!("Trying to activate code : {:?}", &result);

    for (_, code) in cheat_codes_res.codes.iter() {
        println!("Code: {:?}, text: {}", code.kind, code.text);
    }
}
