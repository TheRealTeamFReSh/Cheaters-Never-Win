use crate::runner::player::Player;
use crate::states::GameStates;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const BACKGROUND_SPRITE_WIDTH: f32 = 816.;
const BACKGROUND_SPRITE_HEIGHT: f32 = 480.;
const PLAYER_SPRITE_WIDTH: f32 = 168.;
// TODO: remove magic number
const CAMERA_OFFSET_X: f32 = 1060.;

#[derive(Debug, Component)]
pub struct BackgroundLayer;

pub struct BackgroundResource {
    pub next_x: f32,
}

pub struct BackgroundLayerPlugin;

impl Plugin for BackgroundLayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BackgroundResource {
            next_x: BACKGROUND_SPRITE_WIDTH * 2.0,
        })
        .add_system_set(SystemSet::on_enter(GameStates::Main).with_system(spawn_initial_layers))
        .add_system_set(
            SystemSet::on_update(GameStates::Main)
                .with_system(update_layers)
                .with_system(despawn_backgrounds),
        );
    }
}

fn spawn_initial_layers(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("cyberpunk-city.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(BACKGROUND_SPRITE_WIDTH, BACKGROUND_SPRITE_HEIGHT),
        1,
        1,
    );
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
        .insert(BackgroundLayer {});

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform {
                scale: Vec3::new(1.0, 1.5, 1.0),
                translation: Vec3::new(-BACKGROUND_SPRITE_WIDTH, 0.0, 0.0),
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
                translation: Vec3::new(BACKGROUND_SPRITE_WIDTH, 0.0, 0.0),
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
    mut background_resource: ResMut<BackgroundResource>,
) {
    for transform in camera.iter() {
        if transform.translation.x
            >= (background_resource.next_x - BACKGROUND_SPRITE_WIDTH) - CAMERA_OFFSET_X
        {
            spawn_layer(
                &mut commands,
                &asset_server,
                &mut texture_atlases,
                background_resource.next_x,
            );
            background_resource.next_x += BACKGROUND_SPRITE_WIDTH;
        }
    }
}

pub fn spawn_layer(
    commands: &mut Commands,
    asset_server: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlas>,
    x_pos: f32,
) {
    let texture_handle = asset_server.load("cyberpunk-city.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(BACKGROUND_SPRITE_WIDTH, BACKGROUND_SPRITE_HEIGHT),
        1,
        1,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform {
                scale: Vec3::new(1.0, 1.5, 1.0),
                translation: Vec3::new(x_pos, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(BackgroundLayer);
}

pub fn despawn_backgrounds(
    mut commands: Commands,
    rapier_config: Res<RapierConfiguration>,
    player_query: Query<&RigidBodyPositionComponent, With<Player>>,
    background_query: Query<(Entity, &Transform), With<BackgroundLayer>>,
) {
    for player_rb_pos in player_query.iter() {
        for (entity, background_transform) in background_query.iter() {
            if (player_rb_pos.position.translation.x * rapier_config.scale)
                - background_transform.translation.x
                > 10000.0
            {
                info!("despawning background");
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
