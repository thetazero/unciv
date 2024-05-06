use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::pbr::ClusterConfig;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

mod actions;
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
        .add_plugins(FrameTimeDiagnosticsPlugin::default());

    // app.insert_resource(DebugPickingMode::Normal);

    app.add_systems(
        Startup,
        (
            ui::fps::setup,
            (setup, add_resources),
            world_gen::spawn,
            controls::init_state,
            (ui::panels::init, ui::panels::init_tile_inspector, ui::tick_panel::init),
        )
            .chain(),
    )
    .add_systems(
        Update,
        ((
            (
                controls::handle_keyboard,
                controls::update_selection,
                controls::handle_drag,
                controls::handle_mouse_scroll,
            ),
            (
                ui::panels::update_tile_inspector,
                ui::panels::update_empire_panel,
                ui::button::button_system,
            ),
            (
                ui::fps::fps_text_update_system,
                ui::fps::fps_counter_showhide,
            ),
            tick::execute_actions,
            tick::tick_world,
        ))
            .chain(),
    );

    app.add_event::<controls::InspectTileEvent>();
    app.add_event::<controls::SelectUnit>();
    app.add_event::<controls::DragEvent>();
    app.add_event::<tick::ActionEvent>();
    app.add_event::<tick::EndTurnEvent>();

    app.run();
}

fn add_resources(
    mut commands: Commands,
    materials: ResMut<Assets<StandardMaterial>>,
    meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    let (tile_resources, materials, meshes) = tile::create_tile_resources(materials, meshes);
    commands.insert_resource(tile_resources);

    let (unit_resources, materials, _meshes) = unit::create_resources(materials, meshes);
    commands.insert_resource(unit_resources);

    let (building_resources, _materials) =
        building::create_building_resources(materials, &asset_server);
    commands.insert_resource(building_resources);
}

fn setup(mut commands: Commands, mut ambient_light: ResMut<AmbientLight>) {
    commands.spawn((Camera3dBundle::default(), ClusterConfig::Single));

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::new(0.1, 0.1, 0.), Vec3::Y),
        directional_light: DirectionalLight {
            color: Color::WHITE,
            shadows_enabled: false,
            ..default()
        },
        ..default()
    });

    ambient_light.color = Color::WHITE;
    ambient_light.brightness = 500.;
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
