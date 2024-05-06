use bevy::prelude::*;

use super::button;
use crate::actions;

pub fn init(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(20.0),
                height: Val::Percent(5.0),
                justify_content: JustifyContent::SpaceBetween,
                top: Val::Px(0.),
                right: Val::Percent(30.),
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: Color::rgb(0.1, 0.1, 0.1).into(),
            ..default()
        })
        .with_children(|parent| {
            let noop = actions::Action::Noop;

            parent
                .spawn(button::make_button(&noop))
                .with_children(|parent| {
                    parent.spawn(button::make_button_text("End Turn".to_string()));
                });
        });
}
