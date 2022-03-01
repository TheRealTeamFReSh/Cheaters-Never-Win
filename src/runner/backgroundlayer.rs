use crate::runner::player::Player;
use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct BackgroundLayer;

pub struct BackgroundLayerPlugin;

impl Plugin for BackgroundLayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_background)
            .add_system(move_background);
    }
}

fn spawn_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("cyberpunk-city.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(816.0, 480.0), 1, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform {
                scale: Vec3::new(1.0, 1.5, 1.0),
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
                scale: Vec3::new(1.0, 1.5, 1.0),
                translation: Vec3::new(-816.0, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(BackgroundLayer);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform {
                scale: Vec3::new(1.0, 1.5, 1.0),
                translation: Vec3::new(816.0, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(BackgroundLayer);
}

fn move_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    camera: Query<&Transform, With<Player>>,
    mut query: Query<(Entity, &Transform, &BackgroundLayer, Without<Player>)>,
) {
    if let Some(cam) = camera.iter().next() {
        for (entity, transform, _, _) in query.iter_mut() {
            let length = transform.translation.x + 816. + 168. * 2.;

            if cam.translation.x > length {
                let texture_handle = asset_server.load("cyberpunk-city.png");
                let texture_atlas =
                    TextureAtlas::from_grid(texture_handle, Vec2::new(816.0, 480.0), 1, 1);
                let texture_atlas_handle = texture_atlases.add(texture_atlas);

                commands
                    .spawn_bundle(SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle.clone(),
                        transform: Transform {
                            scale: Vec3::new(1.0, 1.5, 1.0),
                            translation: Vec3::new(cam.translation.x + 1050., 0.0, 0.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(BackgroundLayer);
                commands.entity(entity).despawn();
            } else if cam.translation.x < transform.translation.x - 816. - 168. * 2. {
                let texture_handle = asset_server.load("cyberpunk-city.png");
                let texture_atlas =
                    TextureAtlas::from_grid(texture_handle, Vec2::new(816.0, 480.0), 1, 1);
                let texture_atlas_handle = texture_atlases.add(texture_atlas);

                commands
                    .spawn_bundle(SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle.clone(),
                        transform: Transform {
                            scale: Vec3::new(1.0, 1.5, 1.0),
                            translation: Vec3::new(cam.translation.x - 1060., 0.0, 0.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(BackgroundLayer);
                commands.entity(entity).despawn();
            }
        }
    }
}
