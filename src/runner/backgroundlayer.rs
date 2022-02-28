use crate::runner::player::Player;
use bevy::prelude::*;

#[derive(Debug, Component)]
struct BackgroundLayer;

pub struct BackgroundLayerPlugin;

impl Plugin for BackgroundLayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_background)
            .add_system(move_background.system());
    }
}

fn spawn_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("city-background.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(1120.0, 800.0), 1, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform {
                scale: Vec3::new(2.0, 1.0, 1.0),
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(BackgroundLayer);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform {
                scale: Vec3::new(2.0, 1.0, 1.0),
                translation: Vec3::new(1280.0, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(BackgroundLayer);
}

fn move_background(
    _camera: Query<&Transform, With<Player>>,
    mut _background: Query<&mut Transform, (With<BackgroundLayer>, Without<Player>)>,
) {
    // TODO: Implement endless scrolling of background
}
