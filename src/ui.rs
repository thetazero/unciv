use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::{empire, tile, world_gen};

#[derive(Resource)]
pub struct UiState {
    pub selected_tile: Option<Entity>,
    pub selected_empire: Option<Entity>,
}

#[derive(Component)]
pub struct ResourceUi;

pub fn init(mut commands: Commands) {
    commands.insert_resource(UiState {
        selected_tile: None,
        selected_empire: None,
    });

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

pub fn update_selection(
    mut ev_inspect: EventReader<InspectEvent>,
    mut ui_state: ResMut<UiState>,
    tile_query: Query<(Entity, &tile::Tile)>,
    empire_query: Query<(Entity, &empire::Empire)>,
    world_state: Res<world_gen::WorldState>,
) {
    for ev in ev_inspect.read() {
        ui_state.selected_tile = Some(ev.0);
        let (_, tile) = tile_query.get(ev.0).unwrap();
        if let Some(owner) = tile.owner {
            let empire_entity = world_state.empires.get(&owner).unwrap();
            ui_state.selected_empire = Some(*empire_entity);
        }
    }
}

pub fn update_inspector(
    ui_state: ResMut<UiState>,
    mut inspector_query: Query<&mut Text, With<InspectorTitle>>,
    tile_query: Query<(Entity, &tile::Tile)>,
    empire_query: Query<(Entity, &empire::Empire)>,
    world_state: Res<world_gen::WorldState>,
) {
    match ui_state.selected_tile {
        Some(entity) => {
            let (_, tile) = tile_query.get(entity).unwrap();

            for mut text in inspector_query.iter_mut() {
                let mut string_to_display = format!("Tile: ({}, {})", tile.x, tile.y);

                if let Some(empire_id) = tile.owner {
                    let empire_entity = world_state.empires.get(&empire_id).unwrap();
                    let (_, empire) = empire_query.get(*empire_entity).unwrap();
                    string_to_display.push_str(&format!("\nEmpire: {}", empire.id));
                } else {
                    string_to_display.push_str("\nUnowned");
                }

                text.sections[0].value = string_to_display;
            }
        }
        None => {
            for mut text in inspector_query.iter_mut() {
                text.sections[0].value = "Tile: (0, 0)\nUnowned".to_string();
            }
        }
    }
}

pub fn update_empire_panel(
    ui_state: ResMut<UiState>,
    mut resources_inspector_query: Query<&mut Text, With<ResourceUi>>,
    empire_query: Query<(Entity, &empire::Empire)>,
) {
    match ui_state.selected_empire {
        Some(entity) => {
            let (_, empire) = empire_query.get(entity).unwrap();

            for mut text in resources_inspector_query.iter_mut() {
                text.sections[0].value = format!(
                    "Wood: {}\nStone: {}\nEmpire: {}",
                    empire.inventory.wood, empire.inventory.stone, empire.id
                );
            }
        }
        None => {
            for mut text in resources_inspector_query.iter_mut() {
                text.sections[0].value = "No empire selected".to_string();
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
