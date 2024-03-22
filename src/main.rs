use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

mod tile;
use crate::tile::Tile;

mod colors;
use crate::colors::{bright_hue, normal_hue};

mod ui;
use crate::ui::ResourceUi;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Startup, add_tiles)
        .add_systems(Startup, init_ui)
        .add_systems(Update, draw_tiles)
        .add_systems(Update, update_ui)
        .run();
}

fn distribute(i: i32, count: i32, extent: f32) -> f32 {
    -extent / 2. + i as f32 / (count - 1) as f32 * extent
}

fn add_tiles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let x_count = 10;
    let y_count = 10;
    for x in 0..x_count {
        for y in 0..y_count {
            let square = Mesh2dHandle(meshes.add(Rectangle::new(50.0, 50.0)));
            let color: Color = bright_hue(((x + y) % x_count) as f32 / (x_count) as f32);

            commands.spawn((
                Tile { x, y },
                MaterialMesh2dBundle {
                    mesh: square,
                    material: materials.add(color),
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

fn draw_tiles(query: Query<&Tile>) {}

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
