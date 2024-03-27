use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use fastrand;
use std::collections::HashMap;

use colors::dark_hue;

mod tile;
use crate::tile::{create_tile_resources, Tile, TileResources};

mod colors;
use crate::colors::{bright_hue, normal_hue};

mod ui;
use crate::ui::ResourceUi;

mod empire;
use crate::empire::Empire;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            ((setup, add_resources), add_tiles, link_tiles, (add_empires, init_ui)).chain(),
        )
        .add_systems(Update, update_ui)
        .run();
}

fn distribute(i: i32, count: i32, extent: f32) -> f32 {
    -extent / 2. + i as f32 / (count - 1) as f32 * extent
}

fn add_resources(
    mut commands: Commands,
    materials: ResMut<Assets<ColorMaterial>>,
    meshes: ResMut<Assets<Mesh>>,
) {
    commands.insert_resource(create_tile_resources(materials, meshes));
}

fn add_tiles(mut commands: Commands, tile_resources: Res<TileResources>) {
    let x_count = 10;
    let y_count = 10;

    for x in 0..x_count {
        for y in 0..y_count {
            let tile_bundle = (
                Tile {
                    x,
                    y,
                    neighbors: vec![],
                },
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(tile_resources.square.clone()),
                    material: tile_resources.dark_green.clone(),
                    transform: Transform::from_xyz(
                        // Distribute shapes from -X_EXTENT to +X_EXTENT.
                        distribute(x, x_count, X_EXTENT),
                        distribute(y, x_count, X_EXTENT),
                        0.0,
                    ),
                    ..default()
                },
            );
            commands.spawn(tile_bundle);
        }
    }
}

fn link_tiles(mut query: Query<(Entity, &mut Tile)>) {
    let mut tile_ids: HashMap<(i32, i32), Entity> = HashMap::new();
    for (entity, tile) in query.iter_mut() {
        tile_ids.insert((tile.x, tile.y), entity);
    }

    for (_, mut tile) in query.iter_mut() {
        let neighbors = [
            (tile.x - 1, tile.y),
            (tile.x + 1, tile.y),
            (tile.x, tile.y - 1),
            (tile.x, tile.y + 1),
        ];

        for (x, y) in neighbors.iter() {
            if let Some(neighbor) = tile_ids.get(&(*x, *y)) {
                tile.neighbors.push(*neighbor);
            }
        }
    }
}

fn add_empires(
    mut commands: Commands,
    query: Query<(Entity, &Tile)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    const NUMBER_OF_EMPIRES: i64 = 4;

    let mut empire_data = vec![];

    for i in 0..NUMBER_OF_EMPIRES {
        let color = materials.add(bright_hue(i as f32 / NUMBER_OF_EMPIRES as f32));
        let empire = commands
            .spawn((
                Empire {
                    id: i as i32,
                    color: color.clone(),
                },
                TransformBundle::default(),
                InheritedVisibility::default(),
            ))
            .id();

        empire_data.push((empire, color));
    }

    let mut empire_idx = 0;
    for (entity, _tile) in query.iter() {
        if fastrand::f32() < 1.0 / 10.0 {
            let (empire, color) = &empire_data[empire_idx];

            commands.entity(*empire).push_children(&[entity]);
            commands.entity(entity).insert(color.clone());

            empire_idx += 1;
            if (empire_idx as i64) >= NUMBER_OF_EMPIRES {
                break;
            }
        }
    }
}

const X_EXTENT: f32 = 600.;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn init_ui(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
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
                        width: Val::Px(200.),
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

fn update_ui(mut query: Query<&mut Text, With<ResourceUi>>) {
    for mut text in query.iter_mut() {
        // Update the text
        text.sections[0].value = "New Text".to_string();
    }
}

#[cfg(test)]
mod test {
    use bevy::prelude::*;
    #[test]
    fn spawns_right_number_of_empires() {
        let mut app = App::new();

        app.add_systems(
            Startup,
            (crate::add_resources, crate::add_tiles, crate::add_empires).chain(),
        );

        app.update();

        assert_eq!(
            app.world
                .query::<&crate::empire::Empire>()
                .iter(&app.world)
                .len(),
            4
        );
    }
}
