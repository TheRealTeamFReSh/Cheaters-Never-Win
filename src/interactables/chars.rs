use bevy::prelude::*;

use super::{InteractableComponent, InteractableType};

#[derive(Component)]
pub struct CharTextComponent {
    pub value: char,
}

pub fn spawn_char(
    commands: &mut Commands,
    asset_server: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlas>,
    value: char,
    position: &Vec2,
) {
    let interactable_type = InteractableType::CharText;
    let path = format!("chars/{}_key.png", value);

    let texture_handle = asset_server.load(&path);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 2, 2);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                scale: Vec3::new(2.0, 2.0, 1.0),
                translation: Vec3::new(position.x, position.y, 99.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(InteractableComponent {
            interactable_type,
            range: 16.0,
        })
        .insert(CharTextComponent { value });
}
