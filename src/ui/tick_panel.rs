use bevy::prelude::*;

pub fn init(mut commands: Commands) {
    commands.spawn(NodeBundle {
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
    });
}
