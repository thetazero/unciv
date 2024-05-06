use bevy::{input::mouse::MouseWheel, prelude::*};
use bevy_mod_picking::prelude::*;

use crate::{actions, building, config::CONFIG, tile, unit, utils, world_gen};

#[derive(Resource)]
pub struct SelectorState {
    pub selected_unit: Option<Entity>,
    pub selected_tile: Option<Entity>,
    pub selected_empire: Option<Entity>,
}

pub fn init_state(mut commands: Commands, world_state: ResMut<world_gen::WorldState>) {
    match world_state.empires.get(&0) {
        Some(entity) => {
            commands.insert_resource(SelectorState {
                selected_unit: None,
                selected_tile: None,
                selected_empire: Some(*entity),
            });
        }
        None => {
            commands.insert_resource(SelectorState {
                selected_unit: None,
                selected_tile: None,
                selected_empire: None,
            });
        }
    }
}

pub fn handle_keyboard(
    mut commands: Commands,
    mut camera: Query<&mut Transform, With<Camera3d>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    time: Res<Time>,
    world_state: Res<world_gen::WorldState>,
    mut selector_state: ResMut<SelectorState>,
    unit_query: Query<&unit::Unit>,
    mut tile_query: Query<&mut tile::TileComponent>,
    building_resources: Res<building::BuildingResources>,
    unit_resources: Res<unit::UnitResources>,
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
        let mut z = transform.translation.z * (1. + delta_scale * time.delta_seconds());
        z = z.clamp(CONFIG.camera.min_z, CONFIG.camera.max_z);
        transform.translation.z = z;

        transform.translation.x += delta_x * time.delta_seconds() * z;
        transform.translation.y += delta_y * time.delta_seconds() * z;
    }

    if keyboard_input.just_pressed(CONFIG.keys.action) {
        println!("Action key pressed");
        if let Some(unit_entity) = selector_state.selected_unit {
            let unit = unit_query.get(unit_entity).unwrap();

            let tile_entity = world_state.tile_entities.get(&unit.location).unwrap();
            let tile = tile_query.get_mut(*tile_entity).unwrap();

            let actions = unit::tile_action(unit, tile, unit_entity, *tile_entity, 0);

            for action in actions {
                (tile_query, selector_state, commands) = actions::execute(
                    action,
                    tile_query,
                    selector_state,
                    commands,
                    &building_resources,
                    &unit_resources,
                    &world_state,
                );
            }
        }
    }
}

pub fn move_camera_to(mut camera: Query<&mut Transform, With<Camera3d>>, target: Transform) {
    for mut transform in camera.iter_mut() {
        transform.translation = target.translation;
        transform.translation.z = CONFIG.camera.max_z / 2.;
    }
}

pub fn update_selection(
    mut commands: Commands,
    mut ev_inspect: EventReader<InspectTileEvent>,
    mut unit_inspect: EventReader<SelectUnit>,
    mut selector_state: ResMut<SelectorState>,
    mut unit_query: Query<&mut unit::Unit>,
    tile_query: Query<&tile::TileComponent>,
    world_state: Res<world_gen::WorldState>,
    unit_resources: Res<unit::UnitResources>,
) {
    for ev in ev_inspect.read() {
        let tile = tile_query.get(ev.0);

        match tile {
            Ok(tile) => {
                selector_state.selected_tile = Some(ev.0);
                if let Some(owner) = tile.owner {
                    let empire_entity = world_state.empires.get(&owner).unwrap();
                    selector_state.selected_empire = Some(*empire_entity);
                }

                if let Some(unit) = selector_state.selected_unit {
                    let mut unit = unit_query.get_mut(unit).unwrap();

                    unit.target = Some(tile.tile.location);
                }

                if let Some(unit_enity) = selector_state.selected_unit {
                    (commands, selector_state) = deselect_unit(
                        commands,
                        selector_state,
                        &unit_query,
                        unit_enity,
                        &unit_resources,
                    );
                }
            }
            Err(_) => {
                println!("Tile not found")
            }
        }
    }

    for ev in unit_inspect.read() {
        if let Some(old_unit) = selector_state.selected_unit {
            // Deselect old unit
            (commands, selector_state) = deselect_unit(
                commands,
                selector_state,
                &unit_query,
                old_unit,
                &unit_resources,
            );
        }

        let unit_entity = ev.unit;

        (commands, selector_state) = select_unit(
            commands,
            selector_state,
            &unit_query,
            unit_entity,
            &unit_resources,
        );
    }
}

fn deselect_unit<'a, 'b, 'c>(
    mut commands: Commands<'a, 'b>,
    mut selector_state: ResMut<'c, SelectorState>,
    unit_query: &Query<&mut unit::Unit>,
    unit_entity: Entity,
    unit_resources: &Res<unit::UnitResources>,
) -> (Commands<'a, 'b>, ResMut<'c, SelectorState>) {
    let unit = unit_query.get(unit_entity).unwrap();

    commands
        .entity(unit_entity)
        .insert(unit::get_normal_material(&unit, &unit_resources));

    selector_state.selected_unit = None;

    (commands, selector_state)
}

fn select_unit<'a, 'b, 'c>(
    mut commands: Commands<'a, 'b>,
    mut selector_state: ResMut<'c, SelectorState>,
    unit_query: &Query<&mut unit::Unit>,
    unit_entity: Entity,
    unit_resources: &Res<unit::UnitResources>,
) -> (Commands<'a, 'b>, ResMut<'c, SelectorState>) {
    let unit = unit_query.get(unit_entity).unwrap();

    commands
        .entity(unit_entity)
        .insert(unit::get_selected_material(&unit, &unit_resources));

    selector_state.selected_unit = Some(unit_entity);

    (commands, selector_state)
}

#[derive(Event)]
pub struct InspectTileEvent(Entity);

impl From<ListenerInput<Pointer<Click>>> for InspectTileEvent {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        let listener = event.listener();
        InspectTileEvent(listener)
    }
}

#[derive(Event)]
pub struct SelectUnit {
    pub unit: Entity,
}

impl From<ListenerInput<Pointer<Click>>> for SelectUnit {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        SelectUnit { unit: event.target }
    }
}

#[derive(Event)]
pub struct DragEvent {
    pub dx: f32,
    pub dy: f32,
}
impl From<ListenerInput<Pointer<Drag>>> for DragEvent {
    fn from(event: ListenerInput<Pointer<Drag>>) -> Self {
        DragEvent {
            dx: event.delta.x,
            dy: event.delta.y,
        }
    }
}

pub fn handle_drag(
    mut drag_events: EventReader<DragEvent>,
    mut camera: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    selector_state: Res<SelectorState>,
    tile_query: Query<&tile::TileComponent>,
) {
    for ev in drag_events.read() {
        for mut transform in camera.iter_mut() {
            let dx = ev.dx
                * time.delta_seconds()
                * transform.translation.z
                * CONFIG.camera.mouse_drag_pan_speed;
            let dy = ev.dy
                * time.delta_seconds()
                * transform.translation.z
                * CONFIG.camera.mouse_drag_pan_speed;
            if keyboard_input.pressed(KeyCode::ControlLeft)
                || keyboard_input.pressed(KeyCode::ControlRight)
            {
                // rotate camera
                if let Some(selected_tile) = selector_state.selected_tile {
                    let tile_entity = selected_tile;
                    let tile = tile_query.get(tile_entity).unwrap();
                    let target_tile_translation = utils::to_transform(&tile.tile.location);

                    transform.rotation = transform
                        .looking_at(target_tile_translation.translation, Vec3::Y)
                        .rotation;
                }
            } else {
            }
            transform.translation.x -= dx;
            transform.translation.y += dy;
        }
    }
}

pub fn handle_mouse_scroll(
    mut scroll_events: EventReader<MouseWheel>,
    mut camera: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    for ev in scroll_events.read() {
        for mut transform in camera.iter_mut() {
            let mut z = transform.translation.z
                * (1. - ev.y * CONFIG.camera.mouse_wheel_zoom_speed * time.delta_seconds());
            z = z.clamp(CONFIG.camera.min_z, CONFIG.camera.max_z);
            transform.translation.z = z;
        }
    }
}
