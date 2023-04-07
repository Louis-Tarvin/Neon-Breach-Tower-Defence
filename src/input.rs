use bevy::prelude::*;

use crate::{
    grid::Map,
    state::loading::GameAssets,
    tower::{
        charge_shot::spawn_charge_shot,
        jammer::spawn_jammer,
        laser::{spawn_laser, Direction},
        missile::spawn_silo,
        sniper::spawn_sniper,
        RangeIndicator, Tower, TowerPlaced, TowerType,
    },
    ui::{inventory::Inventory, UiData, UiState, UiStateResource},
};

pub fn grid_click_handler(
    commands: Commands,
    map: ResMut<Map>,
    mut ui_data: ResMut<UiData>,
    mut ui_state: ResMut<UiStateResource>,
    mut inventory: ResMut<Inventory>,
    mouse_input: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
    game_assets: Res<GameAssets>,
    event_writer: EventWriter<TowerPlaced>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    match ui_state.state {
        UiState::PlacingTower(i) => {
            if mouse_input.just_pressed(MouseButton::Left) {
                let window = windows.get_single().unwrap();
                let mouse_pos = window.cursor_position().unwrap();
                let (camera, camera_transform) = camera.get_single().unwrap();

                if let Some(world_position) = camera.viewport_to_world(camera_transform, mouse_pos)
                {
                    let world_position = world_position.origin.truncate();
                    let grid_pos = Map::get_grid_pos(world_position);
                    if map.is_valid_placement(grid_pos) {
                        let tower = inventory.towers.remove(i);
                        match tower.variant {
                            TowerType::ChargeShot => {
                                spawn_charge_shot(
                                    tower,
                                    commands,
                                    grid_pos,
                                    game_assets,
                                    event_writer,
                                    meshes,
                                    materials,
                                    map,
                                );
                            }
                            TowerType::Laser => {
                                spawn_laser(
                                    tower,
                                    commands,
                                    grid_pos,
                                    Direction::Down,
                                    game_assets,
                                    event_writer,
                                    meshes,
                                    materials,
                                    map,
                                );
                            }
                            TowerType::Sniper => {
                                spawn_sniper(
                                    tower,
                                    commands,
                                    grid_pos,
                                    game_assets,
                                    event_writer,
                                    meshes,
                                    materials,
                                    map,
                                );
                            }
                            TowerType::Jammer => {
                                spawn_jammer(
                                    tower,
                                    commands,
                                    grid_pos,
                                    game_assets,
                                    event_writer,
                                    meshes,
                                    materials,
                                    map,
                                );
                            }
                            TowerType::Missile => spawn_silo(
                                tower,
                                commands,
                                grid_pos,
                                game_assets,
                                event_writer,
                                map,
                            ),
                        }
                        ui_state.state = UiState::Normal;
                    }
                }
            }
        }
        UiState::Normal => {
            if mouse_input.just_pressed(MouseButton::Left) {
                let window = windows.get_single().unwrap();
                let mouse_pos = window.cursor_position().unwrap();
                let (camera, camera_transform) = camera.get_single().unwrap();

                if let Some(world_position) = camera.viewport_to_world(camera_transform, mouse_pos)
                {
                    let world_position = world_position.origin.truncate();
                    let grid_pos = Map::get_grid_pos(world_position);
                    if map.is_within_bounds(grid_pos) {
                        if map.placements.contains_key(&grid_pos) {
                            // Select the clicked tower
                            ui_data.selected_pos = Some(grid_pos);
                        } else {
                            ui_data.selected_pos = None;
                        }
                    }
                }
            }
        }
        _ => (),
    }
}

#[derive(Default, Resource)]
pub struct HoverPosition(pub Option<(i8, i8)>);

pub fn mouse_hover_handler(
    map: Res<Map>,
    mut cursor_events: EventReader<CursorMoved>,
    camera: Query<(&Camera, &GlobalTransform)>,
    tower_query: Query<&Children, With<Tower>>,
    mut children_query: Query<&mut Visibility, With<RangeIndicator>>,
    mut hover_pos: ResMut<HoverPosition>,
) {
    for event in cursor_events.iter() {
        let mouse_pos = event.position;
        let (camera, camera_transform) = camera.get_single().unwrap();

        if let Some(world_position) = camera.viewport_to_world(camera_transform, mouse_pos) {
            let world_position = world_position.origin.truncate();
            let grid_pos = Map::get_grid_pos(world_position);
            if let Some(pos) = hover_pos.0 {
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
            }
            // Update the hover position
            if hover_pos.0 != Some(grid_pos) {
                if map.is_within_bounds(grid_pos) {
                    hover_pos.0 = Some(grid_pos);
                } else {
                    hover_pos.0 = None;
                }
            }
        }
    }
}
