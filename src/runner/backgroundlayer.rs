use crate::runner::player::Player;
use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct BackgroundLayer;

pub struct BackgroundLayerPlugin;

impl Plugin for BackgroundLayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_initial_layers)
            .add_system(update_layers);
    }
}

fn spawn_initial_layers(
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

fn update_layers(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    camera: Query<&Transform, With<Player>>,
    mut query: Query<(Entity, &Transform, &BackgroundLayer, Without<Player>)>,
) {
    if let Some(cam) = camera.iter().next() {
        for (entity, transform, _, _) in query.iter_mut() {
            let left_bound = transform.translation.x + 816. + 168. * 2.;
            let right_bound = transform.translation.x - 816. - 168. * 2.;

            let texture_handle = asset_server.load("cyberpunk-city.png");
            let texture_atlas =
                TextureAtlas::from_grid(texture_handle, Vec2::new(816.0, 480.0), 1, 1);
            let texture_atlas_handle = texture_atlases.add(texture_atlas);

            if cam.translation.x > left_bound {
              let distance = cam.translation.x - transform.translation.x;
                println!("distance {}: ", distance);
                spawn_layer(
                    &mut commands,
                    texture_atlas_handle.clone(),
                    cam.translation.x + 1060.,
                );
                commands.entity(entity).despawn();
            } else if cam.translation.x < right_bound {
                spawn_layer(
                    &mut commands,
                    texture_atlas_handle.clone(),
                    cam.translation.x - 1060.,
                );
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn spawn_layer(
    commands: &mut Commands,
    texture_atlas_handle: Handle<TextureAtlas>,
    cam_x_offset: f32,
) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform {
                scale: Vec3::new(1.0, 1.5, 1.0),
                translation: Vec3::new(cam_x_offset, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(BackgroundLayer);
}
