use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

mod building;
mod colors;
mod config;
mod controls;
mod empire;
mod resource;
mod tick;
mod tile;
mod ui;
mod unit;
mod utils;
mod world_gen;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(low_latency_window_plugin()))
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(DebugPickingMode::Normal);

    // app.add_systems(Startup, ui::setup_fps_counter);
    // app.add_systems(Update, (ui::fps_text_update_system, ui::fps_counter_showhide));

    app.add_systems(
        Startup,
        (
            ui::setup_fps_counter,
            (setup, add_resources, tick::init_tick),
            world_gen::spawn,
            ui::init_state,
            (ui::init, ui::init_tile_inspector),
        )
            .chain(),
    )
    .add_systems(
        Update,
        ((
            controls::handle_keyboard,
            ui::update_selection,
            (ui::update_tile_inspector, ui::update_empire_panel),
            (ui::fps_text_update_system, ui::fps_counter_showhide),
            tick::tick_world,
        ))
            .chain(),
    );

    app.add_event::<ui::InspectEvent>();

    app.run();
}

fn add_resources(
    mut commands: Commands,
    materials: ResMut<Assets<ColorMaterial>>,
    meshes: ResMut<Assets<Mesh>>,
) {
    let (tile_resources, materials, meshes) = tile::create_tile_resources(materials, meshes);
    commands.insert_resource(tile_resources);

    let (unit_resources, _materials, _meshes) = unit::create_resources(materials, meshes);
    commands.insert_resource(unit_resources);
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

#[cfg(test)]
mod test {
    use bevy::prelude::*;
    // #[test]
    // fn basic_ecs () {
    //     let mut app = App::new();

    //     app.update();
    // }

    #[test]
    fn spawns_right_number_of_empires() {
        let mut app = App::new();

        app.add_systems(
            Update,
            (crate::add_resources, crate::world_gen::spawn).chain(),
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
