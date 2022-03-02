use crate::states::GameStates;
use bevy::prelude::*;

use super::{InteractableComponent, InteractableType};

#[derive(Component)]
pub struct TerminalAnimationTimer(Timer);

pub struct InteractableTerminalPlugin;

impl Plugin for InteractableTerminalPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameStates::Main).with_system(animate_sprite));
    }
}

pub fn spawn_terminal(
    commands: &mut Commands,
    asset_server: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlas>,
    position: &Vec2,
) {
    let interactable_type = InteractableType::Terminal;

    let texture_handle = asset_server.load("terminal.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(90.0, 60.0), 4, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                scale: Vec3::new(1.0, 1.0, 1.0),
                translation: Vec3::new(position.x, position.y, 99.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(TerminalAnimationTimer(Timer::from_seconds(0.1, true)))
        .insert(InteractableComponent {
            interactable_type,
            range: 65.0,
        })
        .insert(Name::new("Terminal"));
}

pub fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut TerminalAnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}
