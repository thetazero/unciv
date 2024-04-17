use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

mod colors;
mod empire;
mod tile;
mod ui;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(low_latency_window_plugin()))
        .add_plugins(DefaultPickingPlugins)
        .insert_resource(DebugPickingMode::Normal);

    app.add_systems(
        Startup,
        (
            (setup, add_resources),
            tile::spawn,
            tile::link,
            (empire::spawn, ui::init, ui::init_inspector),
        )
            .chain(),
    )
    .add_systems(
        Update,
        ((handle_keyboard_input, ui::update, ui::update_inspector)).chain(),
    );

    app.add_event::<ui::InspectEvent>();

    app.run();
}

fn add_resources(
    mut commands: Commands,
    materials: ResMut<Assets<ColorMaterial>>,
    meshes: ResMut<Assets<Mesh>>,
) {
    commands.insert_resource(tile::create_tile_resources(materials, meshes));
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(Camera2dBundle::default());

    let _background_plane: Handle<Mesh> = meshes.add(Rectangle::new(10000.0, 10000.0));
    let _ocean_dark: Handle<ColorMaterial> = materials.add(Color::hsl(200.0, 0.3, 0.07));

    // commands.spawn((
    //     MaterialMesh2dBundle {
    //         mesh: Mesh2dHandle(background_plane),
    //         transform: Transform::from_xyz(0.0, 0.0, -1.0),
    //         material: ocean_dark,
    //         ..default()
    //     },
    //     PickableBundle::default(),
    //     On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
    //         transform.translation.x += drag.delta.x; // Make the square follow the mouse
    //         transform.translation.y -= drag.delta.y;
    //     }),
    // ));
}

fn handle_keyboard_input(
    mut camera: Query<&mut Transform, With<Camera2d>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }

    let speed = 2.0;
    let mut deltaX = 0.;
    let mut deltaY = 0.;
    if keyboard_input.pressed(KeyCode::KeyW) {
        deltaY += speed;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        deltaY -= speed;
    }

    if keyboard_input.pressed(KeyCode::KeyA) {
        deltaX -= speed;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        deltaX += speed;
    }

    for mut transform in camera.iter_mut() {
        transform.translation.y += deltaY;
        transform.translation.x += deltaX;
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
            (
                crate::add_resources,
                crate::tile::spawn,
                crate::empire::spawn,
            )
                .chain(),
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
