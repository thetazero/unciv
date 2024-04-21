use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::empire;
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
                            "Wood: 0\nStone: 0\nUnowned".to_string(),
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

#[derive(Component)]
pub struct EmpireName;

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
                            "Inspector Title",
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

pub fn update_inspector(
    mut inspector_query: Query<&mut Text, With<InspectorTitle>>,
    mut resources_inspector_query: Query<&mut Text, (With<ResourceUi>, Without<InspectorTitle>)>,
    tile_query: Query<(Entity, &tile::Tile)>,
    mut ev_inspect: EventReader<InspectEvent>,
    mut empires: Query<(Entity, &empire::Empire)>,
) {
    for ev in ev_inspect.read() {
        let entity = ev.0;
        let (_, tile) = tile_query.get(entity).unwrap();

        for mut text in inspector_query.iter_mut() {
            // Update the text
            let mut string_to_display = format!("Tile: ({}, {})", tile.x, tile.y);

            println!("Inspecting tile ({}, {})", tile.x, tile.y);
            if let Some(empire) = tile.owner {
                let (_, empire) = empires.get(empire).unwrap();
                string_to_display.push_str(&format!("\nEmpire: {}", empire.id));
            } else {
                string_to_display.push_str("\nUnowned");
            }

            text.sections[0].value = string_to_display;
        }

        for mut text in resources_inspector_query.iter_mut() {
            if let Some(empire) = tile.owner {
                let (_, empire) = empires.get(empire).unwrap();
                text.sections[0].value = format!(
                    "Wood: {}\nStone: {}\nEmpire: {}",
                    empire.inventory.wood, empire.inventory.stone, empire.id
                );
            } else {
                text.sections[0].value = "Wood: 0\nStone: 0\nUnowned".to_string();
            }
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
