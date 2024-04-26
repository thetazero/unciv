use bevy::prelude::*;

use crate::{building, empire, resource, tile, controls};
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

use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

/// Marker to find the container entity so we can show/hide the FPS counter
#[derive(Component)]
pub struct FpsRoot;

/// Marker to find the text entity so we can update it
#[derive(Component)]
pub struct FpsText;

pub fn setup_fps_counter(mut commands: Commands) {
    // create our UI root node
    // this is the wrapper/container for the text
    let root = commands
        .spawn((
            FpsRoot,
            NodeBundle {
                // give it a dark background for readability
                background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
                // make it "always on top" by setting the Z index to maximum
                // we want it to be displayed over all other UI
                z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    position_type: PositionType::Absolute,
                    // position it at the top-right corner
                    // 1% away from the top window edge
                    right: Val::Percent(1.),
                    top: Val::Percent(1.),
                    // set bottom/left to Auto, so it can be
                    // automatically sized depending on the text
                    bottom: Val::Auto,
                    left: Val::Auto,
                    // give it some padding for readability
                    padding: UiRect::all(Val::Px(4.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();
    // create our text
    let text_fps = commands
        .spawn((
            FpsText,
            TextBundle {
                // use two sections, so it is easy to update just the number
                text: Text::from_sections([
                    TextSection {
                        value: "FPS: ".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            // if you want to use your game's font asset,
                            // uncomment this and provide the handle:
                            // font: my_font_handle
                            ..default()
                        },
                    },
                    TextSection {
                        value: " N/A".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            // if you want to use your game's font asset,
                            // uncomment this and provide the handle:
                            // font: my_font_handle
                            ..default()
                        },
                    },
                ]),
                ..Default::default()
            },
        ))
        .id();
    commands.entity(root).push_children(&[text_fps]);
}

pub fn fps_text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        // try to get a "smoothed" FPS value from Bevy
        if let Some(value) = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            // Format the number as to leave space for 4 digits, just in case,
            // right-aligned and rounded. This helps readability when the
            // number changes rapidly.
            text.sections[1].value = format!("{value:>4.0}");

            // Let's make it extra fancy by changing the color of the
            // text according to the FPS value:
            text.sections[1].style.color = if value >= 120.0 {
                // Above 120 FPS, use green color
                Color::rgb(0.0, 1.0, 0.0)
            } else if value >= 60.0 {
                // Between 60-120 FPS, gradually transition from yellow to green
                Color::rgb((1.0 - (value - 60.0) / (120.0 - 60.0)) as f32, 1.0, 0.0)
            } else if value >= 30.0 {
                // Between 30-60 FPS, gradually transition from red to yellow
                Color::rgb(1.0, ((value - 30.0) / (60.0 - 30.0)) as f32, 0.0)
            } else {
                // Below 30 FPS, use red color
                Color::rgb(1.0, 0.0, 0.0)
            }
        } else {
            // display "N/A" if we can't get a FPS measurement
            // add an extra space to preserve alignment
            text.sections[1].value = " N/A".into();
            text.sections[1].style.color = Color::WHITE;
        }
    }
}

/// Toggle the FPS counter when pressing F12
pub fn fps_counter_showhide(
    mut q: Query<&mut Visibility, With<FpsRoot>>,
    kbd: Res<ButtonInput<KeyCode>>,
) {
    if kbd.just_pressed(KeyCode::F12) {
        let mut vis = q.single_mut();
        *vis = match *vis {
            Visibility::Hidden => Visibility::Visible,
            _ => Visibility::Hidden,
        };
    }
}
