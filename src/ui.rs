use bevy::prelude::*;

#[derive(Component)]
pub struct ResourceUi;

pub fn init(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(20.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        border: UiRect::all(Val::Px(2.)),
                        ..default()
                    },
                    background_color: Color::rgb(0.1, 0.1, 0.1).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        ResourceUi,
                        TextBundle::from_section(
                            "Text Example\nlol",
                            TextStyle {
                                font_size: 20.0,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(5.)),
                            ..default()
                        }),
                        Label,
                    ));
                });
        });

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(80.0),
                height: Val::Percent(20.0),
                justify_content: JustifyContent::SpaceBetween,
                right: Val::Px(0.),
                bottom: Val::Px(0.),
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    border: UiRect::all(Val::Px(2.)),
                    ..default()
                },
                background_color: Color::rgb(0.08, 0.08, 0.08).into(),
                ..default()
            });
        });
}

pub fn update(mut query: Query<&mut Text, With<ResourceUi>>) {
    for mut text in query.iter_mut() {
        // Update the text
        text.sections[0].value = "New Text".to_string();
    }
}
