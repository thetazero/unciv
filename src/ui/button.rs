use bevy::prelude::*;

use crate::{actions, tick};

const BUTTON_BORDER: Color = Color::hsl(0.0, 0.0, 0.5);
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn button_system(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
    actions_query: Query<&actions::Action>,
    mut actions_writer: EventWriter<tick::ActionEvent>,
) {
    for (button_entity, interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;

                if let Ok(action) = actions_query.get(button_entity) {
                    println!("{:?}", action);
                    actions_writer.send(tick::ActionEvent {
                        action: action.clone(),
                    });
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = BUTTON_BORDER;
            }
        }
    }
}

pub fn make_button(action: &actions::Action) -> (ButtonBundle, actions::Action) {
    (
        ButtonBundle {
            style: Style {
                height: Val::Px(30.0),
                border: UiRect::all(Val::Px(1.0)),
                margin: UiRect::all(Val::Px(20.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            border_color: BorderColor(BUTTON_BORDER),
            background_color: NORMAL_BUTTON.into(),
            ..default()
        },
        action.clone(),
    )
}

pub fn make_button_text(text: String) -> TextBundle {
    TextBundle::from_section(
        text,
        TextStyle {
            font_size: 20.0,
            color: Color::rgb(0.9, 0.9, 0.9),
            ..Default::default()
        },
    )
}
