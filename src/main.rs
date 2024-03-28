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
            (empire::spawn, ui::init),
        )
            .chain(),
    )
    .add_systems(Update, ui::update);

    app.run();
}

fn add_resources(
    mut commands: Commands,
    materials: ResMut<Assets<ColorMaterial>>,
    meshes: ResMut<Assets<Mesh>>,
) {
    commands.insert_resource(tile::create_tile_resources(materials, meshes));
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
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
