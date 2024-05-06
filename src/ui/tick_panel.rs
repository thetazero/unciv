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
                align_content: AlignContent::Center,
                ..default()
            },
            background_color: Color::rgb(0.1, 0.1, 0.1).into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(button::make_button(&actions::Action::EndTurn))
                .with_children(|parent| {
                    parent.spawn(button::make_button_text("End Turn".to_string()));
                });
        });
}
