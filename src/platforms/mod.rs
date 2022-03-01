use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_test_platforms.after("setup_physics"));
    }
}

pub enum PlatformKind {
    Platform1,
    Platform2,
    Platform3,
    Platform4,
    Platform5,
    Platform6,
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

/// Test spawn platform
fn spawn_test_platforms(
    mut commands: Commands,
    rapier_config: Res<RapierConfiguration>,
    asset_server: Res<AssetServer>,
) {
    spawn_platform(
        PlatformKind::Platform1,
        [0.0, -300.0].into(),
        &mut commands,
        &rapier_config,
        &asset_server,
    );
    spawn_platform(
        PlatformKind::Platform2,
        [112.0, -300.0].into(),
        &mut commands,
        &rapier_config,
        &asset_server,
    );
    spawn_platform(
        PlatformKind::Platform3,
        [64.0 + 112.0, -300.0].into(),
        &mut commands,
        &rapier_config,
        &asset_server,
    );
    spawn_platform(
        PlatformKind::Platform4,
        [64.0 + 112.0 + 32.0 + 42.5, -300.0].into(),
        &mut commands,
        &rapier_config,
        &asset_server,
    );
    spawn_platform(
        PlatformKind::Platform5,
        [64.0 + 112.0 + 32.0 + 85.0 + 42.5, -325.0].into(),
        &mut commands,
        &rapier_config,
        &asset_server,
    );
    spawn_platform(
        PlatformKind::Platform6,
        [64.0 + 112.0 + 32.0 + 85.0 + 85.0 + 68.5, -325.0].into(),
        &mut commands,
        &rapier_config,
        &asset_server,
    );
}

/// Spawn an individual platform at a location
fn spawn_platform(
    platform_kind: PlatformKind,
    position: Vec2,
    commands: &mut Commands,
    rapier_config: &RapierConfiguration,
    asset_server: &AssetServer,
) {
    match platform_kind {
        PlatformKind::Platform1 => {
            spawn_platform_1(position, commands, rapier_config, asset_server);
        }
        PlatformKind::Platform2 => {
            spawn_platform_2(position, commands, rapier_config, asset_server);
        }
        PlatformKind::Platform3 => {
            spawn_platform_3(position, commands, rapier_config, asset_server);
        }
        PlatformKind::Platform4 => {
            spawn_platform_4(position, commands, rapier_config, asset_server);
        }
        PlatformKind::Platform5 => {
            spawn_platform_5(position, commands, rapier_config, asset_server);
        }
        PlatformKind::Platform6 => {
            spawn_platform_6(position, commands, rapier_config, asset_server);
        }
    }
}

fn spawn_platform_1(
    position: Vec2,
    commands: &mut Commands,
    rapier_config: &RapierConfiguration,
    asset_server: &AssetServer,
) {
    let collider_size_hx = 160.0 / rapier_config.scale / 2.0;
    let collider_size_hy = 34.0 / rapier_config.scale / 2.0;

    commands
        .spawn_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            position: Vec2::new(
                position.x / rapier_config.scale,
                position.y / rapier_config.scale,
            )
            .into(),
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
        .insert(Name::new("Platform 1"))
        .with_children(|parent| {
            parent.spawn_bundle(SpriteBundle {
                texture: asset_server.load("platform_1.png"),
                transform: Transform::from_translation(Vec3::new(0.0, -23.0, 50.0)),
                ..Default::default()
            });
        });
}

fn spawn_platform_2(
    position: Vec2,
    commands: &mut Commands,
    rapier_config: &RapierConfiguration,
    asset_server: &AssetServer,
) {
    let collider_size_hx = 64.0 / rapier_config.scale / 2.0;
    let collider_size_hy = 34.0 / rapier_config.scale / 2.0;

    commands
        .spawn_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            position: Vec2::new(
                position.x / rapier_config.scale,
                position.y / rapier_config.scale,
            )
            .into(),
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
        .insert(Name::new("Platform 2"))
        .with_children(|parent| {
            parent.spawn_bundle(SpriteBundle {
                texture: asset_server.load("platform_2.png"),
                transform: Transform::from_translation(Vec3::new(0.0, 4.0, 50.0)),
                ..Default::default()
            });
        });
}

fn spawn_platform_3(
    position: Vec2,
    commands: &mut Commands,
    rapier_config: &RapierConfiguration,
    asset_server: &AssetServer,
) {
    let collider_size_hx = 64.0 / rapier_config.scale / 2.0;
    let collider_size_hy = 34.0 / rapier_config.scale / 2.0;

    commands
        .spawn_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            position: Vec2::new(
                position.x / rapier_config.scale,
                position.y / rapier_config.scale,
            )
            .into(),
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
        .insert(Name::new("Platform 3"))
        .with_children(|parent| {
            parent.spawn_bundle(SpriteBundle {
                texture: asset_server.load("platform_3.png"),
                transform: Transform::from_translation(Vec3::new(0.0, 4.0, 50.0)),
                ..Default::default()
            });
        });
}

fn spawn_platform_4(
    position: Vec2,
    commands: &mut Commands,
    rapier_config: &RapierConfiguration,
    asset_server: &AssetServer,
) {
    let collider_size_hx = 85.0 / rapier_config.scale / 2.0;
    let collider_size_hy = 34.0 / rapier_config.scale / 2.0;

    commands
        .spawn_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            position: Vec2::new(
                position.x / rapier_config.scale,
                position.y / rapier_config.scale,
            )
            .into(),
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
        .insert(Name::new("Platform 4"))
        .with_children(|parent| {
            parent.spawn_bundle(SpriteBundle {
                texture: asset_server.load("platform_4.png"),
                transform: Transform::from_translation(Vec3::new(0.0, 4.0, 50.0)),
                ..Default::default()
            });
        });
}

fn spawn_platform_5(
    position: Vec2,
    commands: &mut Commands,
    rapier_config: &RapierConfiguration,
    asset_server: &AssetServer,
) {
    let collider_size_hx = 85.0 / rapier_config.scale / 2.0;
    let collider_size_hy = 87.0 / rapier_config.scale / 2.0;

    commands
        .spawn_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            position: Vec2::new(
                position.x / rapier_config.scale,
                position.y / rapier_config.scale,
            )
            .into(),
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
        .insert(Name::new("Platform 5"))
        .with_children(|parent| {
            parent.spawn_bundle(SpriteBundle {
                texture: asset_server.load("platform_5.png"),
                transform: Transform::from_translation(Vec3::new(0.0, 4.0, 50.0)),
                ..Default::default()
            });
        });
}

fn spawn_platform_6(
    position: Vec2,
    commands: &mut Commands,
    rapier_config: &RapierConfiguration,
    asset_server: &AssetServer,
) {
    let collider_size_hx = 137.0 / rapier_config.scale / 2.0;
    let collider_size_hy = 87.0 / rapier_config.scale / 2.0;

    commands
        .spawn_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            position: Vec2::new(
                position.x / rapier_config.scale,
                position.y / rapier_config.scale,
            )
            .into(),
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
        .insert(Name::new("Platform 6"))
        .with_children(|parent| {
            parent.spawn_bundle(SpriteBundle {
                texture: asset_server.load("platform_6.png"),
                transform: Transform::from_translation(Vec3::new(0.0, 4.0, 50.0)),
                ..Default::default()
            });
        });
}
