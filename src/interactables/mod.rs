use bevy::prelude::*;
mod chars;
mod terminal;

pub enum InteractableType {
    Terminal,
    CharText,
}

#[derive(Component)]
pub struct InteractableComponent {
    pub interactable_type: InteractableType,
    pub range: f32,
}

pub struct InteractablesPlugin;

impl Plugin for InteractablesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(terminal::InteractableTerminalPlugin);
    }
}

pub use chars::{spawn_char, CharTextComponent};
pub use terminal::spawn_terminal;
