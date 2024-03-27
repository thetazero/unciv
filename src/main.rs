use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use colors::dark_hue;

mod tile;
use crate::tile::{Tile, TileResources, create_tile_resources};

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
            ((setup, add_resources), (add_tiles, add_empires, init_ui)).chain(),
        )
        .add_systems(Update, draw_tiles)
        .add_systems(Update, update_ui)
        .run();
}

fn distribute(i: i32, count: i32, extent: f32) -> f32 {
    -extent / 2. + i as f32 / (count - 1) as f32 * extent
}

fn add_resources(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.insert_resource(create_tile_resources(materials, meshes));
}

fn add_tiles(mut commands: Commands, tile_resources: Res<TileResources>) {
    let x_count = 10;
    let y_count = 10;
    for x in 0..x_count {
        for y in 0..y_count {
            commands.spawn((
                Tile { x, y },
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
            ));
        }
    }
}

fn add_empires(mut commands: Commands) {
    let empire = commands.spawn(Empire {}).id();

    let tile1 = commands.spawn(Tile { x: 0, y: 0 }).id();
    let tile2 = commands.spawn(Tile { x: 3, y: 2 }).id();

    commands.entity(empire).push_children(&[tile1, tile2]);
}

fn draw_tiles(
    mut commands: Commands,
    query: Query<(Entity, &Tile)>,
    tile_resources: Res<TileResources>,
) {
    for (entity, tile) in query.iter() {
        // transform.translation.x += 0.2;
        // let color_mat = materials.get_mut(color_handle).unwrap();
        if (tile.x + tile.y) % 2 == 0 {
            commands.entity(entity).insert(tile_resources.empire_red.clone());
        }
    }
}

const X_EXTENT: f32 = 600.;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn init_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
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
