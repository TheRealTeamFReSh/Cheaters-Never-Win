use crate::states::GameStates;
use bevy::prelude::*;
pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameStates::Main)
                .with_system(animate_effect)
                .after("setup_physics"),
        );
    }
}

#[derive(Debug, Component)]
pub struct OneShotEffect;

#[derive(Component)]
pub struct EffectAnimationTimer(Timer);

pub fn spawn_explosion(
    position: Vec2,
    commands: &mut Commands,
    //rapier_config: &RapierConfiguration,
    asset_server: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlas>,
) {
    let texture_handle = asset_server.load("explosion.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(55.0, 53.0), 6, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                scale: Vec3::new(1.5, 1.5, 1.0),
                translation: Vec3::new(
                    //position.x * rapier_config.scale,
                    //position.y * rapier_config.scale,
                    position.x, position.y, 60.0,
                ),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(OneShotEffect)
        .insert(EffectAnimationTimer(Timer::from_seconds(0.1, true)))
        .insert(Name::new("Explosion"));
}

fn animate_effect(
    mut commands: Commands,
    time: Res<Time>,
    mut effect_query: Query<(Entity, &mut EffectAnimationTimer, &mut TextureAtlasSprite)>,
) {
    for (entity, mut timer, mut sprite) in effect_query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            if sprite.index == 5 {
                commands.entity(entity).despawn();
            } else {
                sprite.index += 1;
            }
        }
    }
}
