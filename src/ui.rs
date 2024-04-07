use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::tile;

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
}

#[derive(Component)]
pub struct InspectorTitle;

pub fn init_inspector(mut commands: Commands) {
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
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        border: UiRect::all(Val::Px(2.)),
                        ..default()
                    },
                    background_color: Color::rgb(0.08, 0.08, 0.08).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        InspectorTitle,
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
}

pub fn update(
    mut query: Query<&mut Text, With<ResourceUi>>,
    mut ev_inspect: EventReader<InspectEvent>,
) {
    for mut text in query.iter_mut() {
        // Update the text
        text.sections[0].value = "New Text".to_string();
    }
}

pub fn update_inspector(
    mut query: Query<&mut Text, With<InspectorTitle>>,
    tile_query: Query<(Entity, &tile::Tile)>,
    mut ev_inspect: EventReader<InspectEvent>,
) {
    for ev in ev_inspect.read() {
        for mut text in query.iter_mut() {
            // Update the text
            let entity = ev.0;
            let (_, tile) = tile_query.get(entity).unwrap();
            text.sections[0].value = format!("Tile: ({}, {})", tile.x, tile.y);
            println!("Inspecting tile ({}, {})", tile.x, tile.y);
        }
    }
}

#[derive(Event)]
pub struct InspectEvent(Entity);

impl From<ListenerInput<Pointer<Click>>> for InspectEvent {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        InspectEvent(event.target)
    }
}
