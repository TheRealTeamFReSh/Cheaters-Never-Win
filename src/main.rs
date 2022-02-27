use bevy::prelude::*;

mod runner;

fn main() {
    let window = WindowDescriptor {
        title: "Bevy Jam".to_string(),
        width: 1280.0,
        height: 720.0,
        vsync: true,
        resizable: false,
        ..Default::default()
    };

    App::new()
        .insert_resource(window)
        .add_plugins(DefaultPlugins)
        .add_plugin(runner::RunnerPlugin)
        .run();
}
