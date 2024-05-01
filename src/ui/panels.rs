use bevy::prelude::*;

use super::button;
use crate::{building, controls, empire, resource, tile};
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
                        display: Display::Grid,
                        grid_template_rows: RepeatedGridTrack::flex(2, 1.0),
                        ..default()
                    },
                    background_color: Color::rgb(0.1, 0.1, 0.1).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                border: UiRect::all(Val::Px(2.)),
                                margin: UiRect::all(Val::Px(10.)),
                                display: Display::Grid,
                                ..default()
                            },
                            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
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

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                border: UiRect::all(Val::Px(2.)),
                                margin: UiRect::all(Val::Px(10.)),
                                display: Display::Grid,
                                ..default()
                            },
                            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            init_entity_spawner_tab(parent);
                        });
                });
        });
}

fn init_entity_spawner_tab(parent: &mut ChildBuilder) {
    parent.spawn(button::make_bundle()).with_children(|parent| {
        parent.spawn(button::make_text("End Turn".to_string()));
    });
}

#[derive(Component)]
pub struct TileInspectorTitle;

#[derive(Component)]
pub struct TileInspectorBuildingList;

#[derive(Component)]
pub struct EmpireName;

pub fn init_tile_inspector(mut commands: Commands) {
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
                        width: Val::Percent(50.0),
                        border: UiRect::all(Val::Px(2.)),
                        ..default()
                    },
                    background_color: Color::rgb(0.08, 0.08, 0.08).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TileInspectorTitle,
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

            init_builder_tab(parent);
        });
}

fn init_builder_tab(root: &mut ChildBuilder) {
    root.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(50.0),
            border: UiRect::all(Val::Px(2.)),
            padding: UiRect::all(Val::Px(10.)),
            ..default()
        },
        background_color: Color::rgb(0.1, 0.1, 0.1).into(),
        ..default()
    })
    .with_children(|parent| {
        parent
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    border: UiRect::all(Val::Px(2.)),
                    ..default()
                },
                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn((
                    TileInspectorBuildingList,
                    TextBundle::from_section(
                        "Building List",
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

fn tile_to_string(tile: &tile::Tile) -> String {
    let kind = tile::tile_string(&tile.kind);
    format!("{}: ({}, {})", kind, tile.location.x, tile.location.y)
}

pub fn update_tile_inspector(
    ui_state: ResMut<controls::SelectorState>,
    mut tile_title_query: Query<
        &mut Text,
        (With<TileInspectorTitle>, Without<TileInspectorBuildingList>),
    >,
    mut building_list_query: Query<
        &mut Text,
        (With<TileInspectorBuildingList>, Without<TileInspectorTitle>),
    >,
    tile_query: Query<(Entity, &tile::Tile)>,
) {
    match ui_state.selected_tile {
        Some(entity) => {
            let (_, tile) = tile_query.get(entity).unwrap();

            let string_to_display = tile_to_string(tile);

            set_query_text(&mut tile_title_query, &string_to_display);

            let building_list: Vec<String> = tile
                .building
                .iter()
                .map(|building| building::building_name(building))
                .collect();

            let building_list = building_list.join("\t");

            set_query_text(&mut building_list_query, &building_list);
        }
        None => {
            set_query_text(&mut tile_title_query, "No tile selected");
            set_query_text(&mut building_list_query, "");
        }
    }
}

fn set_query_text<T: bevy::ecs::query::QueryFilter>(
    query: &mut Query<&mut Text, T>,
    new_text: &str,
) {
    for mut text in query.iter_mut() {
        text.sections[0].value = new_text.to_string();
    }
}

pub fn update_empire_panel(
    ui_state: ResMut<controls::SelectorState>,
    mut resources_inspector_query: Query<&mut Text, With<ResourceUi>>,
    empire_query: Query<&empire::Empire>,
) {
    match ui_state.selected_empire {
        Some(entity) => {
            let empire = empire_query.get(entity).unwrap();

            for mut text in resources_inspector_query.iter_mut() {
                text.sections[0].value = format!(
                    "Empire: {}\nWood: {}\nStone: {}",
                    empire.id,
                    empire
                        .inventory
                        .inv
                        .get(&resource::Resource::Wood)
                        .unwrap_or(&0),
                    empire
                        .inventory
                        .inv
                        .get(&resource::Resource::Stone)
                        .unwrap_or(&0),
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
