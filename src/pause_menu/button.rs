use bevy::prelude::*;

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component, Clone)]
#[allow(dead_code)]
pub struct UIButton {
    text: String,
    font_handle: Handle<Font>,
    pub name: String,
}

impl UIButton {
    pub fn new(text: String, font_handle: Handle<Font>, name: String) -> Self {
        Self {
            text,
            font_handle,
            name,
        }
    }

    pub fn spawn(&self, parent: &mut ChildBuilder) {
        let container_component = ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(200.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Px(10.)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: NORMAL_BUTTON.into(),
            ..Default::default()
        };

        let text_component = TextBundle {
            text: Text::with_section(
                self.text.clone(),
                TextStyle {
                    font: self.font_handle.clone(),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
                Default::default(),
            ),
            ..Default::default()
        };

        parent
            .spawn_bundle(container_component)
            .with_children(|parent| {
                parent.spawn_bundle(text_component);
            })
            .insert(self.clone());
    }
}

#[allow(dead_code)]
pub fn button_system(
    mut interaction_query: Query<(&Interaction, &mut UiColor, &UIButton), Changed<Interaction>>,
) {
    for (interaction, mut color, _) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
