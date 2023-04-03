use bevy::prelude::*;

use crate::{
    grid::Map,
    state::loading::GameAssets,
    tower::{
        charge_shot::{spawn_charge_shot, RangeIndicator},
        laser::spawn_laser,
        Tower, TowerPlaced,
    },
    ui::UIState,
};

pub fn grid_click_handler(
    commands: Commands,
    map: ResMut<Map>,
    mouse_input: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
    game_assets: Res<GameAssets>,
    event_writer: EventWriter<TowerPlaced>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    if mouse_input.just_pressed(MouseButton::Right) {
        let window = windows.get_single().unwrap();
        let mouse_pos = window.cursor_position().unwrap();
        let (camera, camera_transform) = camera.get_single().unwrap();

        if let Some(world_position) = camera.viewport_to_world(camera_transform, mouse_pos) {
            let world_position = world_position.origin.truncate();
            let grid_pos = Map::get_grid_pos(world_position);
            if map.is_valid_placement(grid_pos) {
                spawn_laser(
                    commands,
                    grid_pos,
                    crate::tower::laser::Direction::Right,
                    game_assets,
                    event_writer,
                    meshes,
                    materials,
                    map,
                );
            }
        }
    } else if mouse_input.just_pressed(MouseButton::Left) {
        let window = windows.get_single().unwrap();
        let mouse_pos = window.cursor_position().unwrap();
        let (camera, camera_transform) = camera.get_single().unwrap();

        if let Some(world_position) = camera.viewport_to_world(camera_transform, mouse_pos) {
            let world_position = world_position.origin.truncate();
            let grid_pos = Map::get_grid_pos(world_position);
            if map.is_valid_placement(grid_pos) {
                spawn_charge_shot(
                    commands,
                    grid_pos,
                    game_assets,
                    event_writer,
                    meshes,
                    materials,
                    map,
                );
            }
        }
    }
}

pub fn mouse_hover_handler(
    map: Res<Map>,
    mut cursor_events: EventReader<CursorMoved>,
    camera: Query<(&Camera, &GlobalTransform)>,
    tower_query: Query<&Children, With<Tower>>,
    mut children_query: Query<&mut Visibility, With<RangeIndicator>>,
    mut ui_state: ResMut<UIState>,
) {
    for event in cursor_events.iter() {
        let mouse_pos = event.position;
        let (camera, camera_transform) = camera.get_single().unwrap();

        if let Some(world_position) = camera.viewport_to_world(camera_transform, mouse_pos) {
            let world_position = world_position.origin.truncate();
            let grid_pos = Map::get_grid_pos(world_position);
            if let Some(pos) = ui_state.hovered_pos {
                if pos != grid_pos {
                    // Hide the range of the previously hovered tower
                    if let Some(tower) = map.placements.get(&pos) {
                        if let Ok(children) = tower_query.get(*tower) {
                            for child in children.iter() {
                                if let Ok(mut visibility) = children_query.get_mut(*child) {
                                    *visibility = Visibility::Hidden;
                                }
                            }
                        }
                    }
                }
            }
            // Show the range of the newly hovered tower
            if let Some(tower) = map.placements.get(&grid_pos) {
                if let Ok(children) = tower_query.get(*tower) {
                    for child in children.iter() {
                        if let Ok(mut visibility) = children_query.get_mut(*child) {
                            *visibility = Visibility::Visible;
                        }
                    }
                }
                ui_state.hovered_pos = Some(grid_pos);
            } else {
                ui_state.hovered_pos = None;
            }
        }
    }
}
