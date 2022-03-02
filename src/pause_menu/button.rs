use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
pub struct UIButtonComponent;

#[allow(dead_code)]
pub struct UIButton {
    text: String,
    font_handle: Handle<Font>,
    on_click: fn() -> (),
}

impl UIButton {
    pub fn new(text: String, font_handle: Handle<Font>, on_click: fn() -> ()) -> Self {
        Self {
            text,
            font_handle,
            on_click,
        }
    }

    pub fn spawn(&self, parent: &mut ChildBuilder) {
        let container_component = ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
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
            .insert(UIButtonComponent);
    }
}

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<UIButtonComponent>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
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
