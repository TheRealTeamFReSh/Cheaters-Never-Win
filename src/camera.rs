use bevy::prelude::*;
use bevy_parallax::ParallaxCameraComponent;

#[derive(Component)]
pub struct UICameraComponent;

pub fn add_camera(mut commands: Commands) {
    info!("Spawning cameras");
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(UICameraComponent);
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(ParallaxCameraComponent);
}
