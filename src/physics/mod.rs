use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::states::GameStates;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameStates::Main).with_system(setup_physics.label("setup_physics")),
        );
    }
}

fn setup_physics(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = [0.0, -140.0].into();
    rapier_config.scale = 10.0;
}

pub fn jump(
    y_impulse: f32,
    rb_vel: &mut RigidBodyVelocityComponent,
    rb_mprops: &RigidBodyMassPropsComponent,
) {
    rb_vel.apply_impulse(rb_mprops, Vec2::new(0.0, y_impulse).into());
}
