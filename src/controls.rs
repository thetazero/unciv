use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::{config::CONFIG, tile, world_gen};

#[derive(Resource)]
pub struct SelectorState {
    pub selected_tile: Option<Entity>,
    pub selected_empire: Option<Entity>,
}

pub fn init_state(mut commands: Commands, world_state: ResMut<world_gen::WorldState>) {
    match world_state.empires.get(&0) {
        Some(entity) => {
            commands.insert_resource(SelectorState {
                selected_tile: None,
                selected_empire: Some(*entity),
            });
        }
        None => {
            commands.insert_resource(SelectorState {
                selected_tile: None,
                selected_empire: None,
            });
        }
    }
}


pub fn handle_keyboard(
    mut camera: Query<&mut Transform, With<Camera2d>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    time: Res<Time>,
) {
    if keyboard_input.just_pressed(CONFIG.keys.quit) {
        app_exit_events.send(bevy::app::AppExit);
    }

    let mut delta_x = 0.;
    let mut delta_y = 0.;
    if keyboard_input.pressed(KeyCode::KeyW) {
        delta_y += CONFIG.camera.pan_speed;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        delta_y -= CONFIG.camera.pan_speed;
    }

    if keyboard_input.pressed(KeyCode::KeyA) {
        delta_x -= CONFIG.camera.pan_speed;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        delta_x += CONFIG.camera.pan_speed;
    }

    let mut delta_scale = 0.;
    if keyboard_input.pressed(CONFIG.keys.zoom_in) {
        delta_scale -= CONFIG.camera.zoom_speed;
    } else if keyboard_input.pressed(CONFIG.keys.zoom_out) {
        delta_scale += CONFIG.camera.zoom_speed;
    }

    for mut transform in camera.iter_mut() {
        transform.scale *= 1. + delta_scale * time.delta_seconds();
        transform.scale = transform
            .scale
            .clamp(CONFIG.camera.min_zoom, CONFIG.camera.max_zoom);

        let scale_magnitude = transform.scale.length();

        transform.translation.x += delta_x * time.delta_seconds() * scale_magnitude;
        transform.translation.y += delta_y * time.delta_seconds() * scale_magnitude;
    }
}

pub fn move_camera_to(mut camera: Query<&mut Transform, With<Camera2d>>, target: Transform) {
    for mut transform in camera.iter_mut() {
        transform.translation.x = target.translation.x;
        transform.translation.y = target.translation.y;
    }
}

pub fn update_selection(
    mut ev_inspect: EventReader<InspectTileEvent>,
    mut ui_state: ResMut<SelectorState>,
    tile_query: Query<(Entity, &tile::Tile)>,
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

#[derive(Event)]
pub struct InspectTileEvent(Entity);

impl From<ListenerInput<Pointer<Click>>> for InspectTileEvent {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        InspectTileEvent(event.target)
    }
}


#[derive(Event)]
pub struct SelectUnit(Entity);

impl From<ListenerInput<Pointer<Click>>> for SelectUnit {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        SelectUnit(event.target)
    }
}
