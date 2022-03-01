use bevy::prelude::*;

#[derive(Component)]
pub struct UICameraComponent;

#[derive(Component)]
pub struct TwoDCameraComponent;

pub fn add_camera(mut commands: Commands) {
    info!("Spawning cameras");
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(UICameraComponent);
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(TwoDCameraComponent);
}
