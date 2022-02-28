use crate::{physics::jump, states::GameStates};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_test_enemy.after("setup_physics"))
            .add_system_set(
                SystemSet::on_update(GameStates::Main).with_system(slime_enemy_behavior),
            );
    }
}

pub enum EnemyKind {
    Slime,
}

#[derive(Debug, Component)]
pub struct Enemy;

#[derive(Debug, Component)]
pub struct SlimeEnemy {
    pub jump_timer: Timer,
    pub jump_impulse: f32,
    pub jump_torque_impulse: f32,
}

fn spawn_enemy(
    enemy_kind: EnemyKind,
    position: Vec2,
    commands: &mut Commands,
    rapier_config: &RapierConfiguration,
    asset_server: &AssetServer,
) {
    match enemy_kind {
        EnemyKind::Slime => {
            spawn_slime(position, commands, rapier_config, asset_server);
        }
    }
}

fn spawn_slime(
    position: Vec2,
    commands: &mut Commands,
    rapier_config: &RapierConfiguration,
    asset_server: &AssetServer,
) {
    let collider_radius = 24.0 * 2.0 / rapier_config.scale / 2.0;

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("slime.png"),
            transform: Transform {
                scale: Vec3::new(2.0, 2.0, 1.0),
                translation: Vec3::new(0.0, 0.0, 51.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Dynamic.into(),
            position: Vec2::new(
                position.x / rapier_config.scale,
                position.y / rapier_config.scale,
            )
            .into(),
            mass_properties: RigidBodyMassPropsFlags::TRANSLATION_LOCKED_X.into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::ball(collider_radius).into(),
            material: ColliderMaterial {
                friction: 0.5,
                restitution: 0.5,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(Enemy)
        .insert(SlimeEnemy {
            jump_timer: Timer::from_seconds(5.0, true),
            jump_impulse: 1000.0,
            jump_torque_impulse: 800.0,
        })
        .insert(Name::new("Enemy-Slime"));
}

fn spawn_test_enemy(
    mut commands: Commands,
    rapier_config: Res<RapierConfiguration>,
    asset_server: Res<AssetServer>,
) {
    spawn_enemy(
        EnemyKind::Slime,
        [75.0, -150.0].into(),
        &mut commands,
        &rapier_config,
        &asset_server,
    );
}

fn slime_enemy_behavior(
    time: Res<Time>,
    mut slime_query: Query<(
        &mut SlimeEnemy,
        &mut RigidBodyVelocityComponent,
        &RigidBodyMassPropsComponent,
    )>,
) {
    for (mut slime_enemy, mut rb_vel, rb_mprops) in slime_query.iter_mut() {
        slime_enemy.jump_timer.tick(time.delta());
        if slime_enemy.jump_timer.just_finished() {
            // Apply impulses.
            jump(slime_enemy.jump_impulse, &mut rb_vel, rb_mprops);
            rb_vel.apply_torque_impulse(rb_mprops, slime_enemy.jump_torque_impulse);
        }
    }
}
