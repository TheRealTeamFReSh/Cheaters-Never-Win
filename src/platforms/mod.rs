use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_ground.after("setup_physics"));
    }
}

/// Spawn ground under player
fn spawn_ground(
    mut commands: Commands,
    rapier_config: Res<RapierConfiguration>,
    asset_server: Res<AssetServer>,
) {
    let collider_size_hx = 2000.0 / rapier_config.scale / 2.0;
    let collider_size_hy = 24.0 / rapier_config.scale / 2.0;

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("ground.png"),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 50.0)),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            position: Vec2::new(0.0, -250.0 / rapier_config.scale).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(collider_size_hx, collider_size_hy).into(),
            material: ColliderMaterial {
                friction: 0.0,
                restitution: 0.0,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(Name::new("Ground"));
}
