use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{
    grid::Map,
    state::loading::GameAssets,
    tower::{charge_shot::ChargeShot, Debuff, Tower, TowerPlaced, TowerType},
    ui::UIState,
};

pub fn grid_click_handler(
    mut commands: Commands,
    mut map: ResMut<Map>,
    mouse_input: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
    game_assets: Res<GameAssets>,
    mut event_writer: EventWriter<TowerPlaced>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if mouse_input.just_pressed(MouseButton::Right) {
        let window = windows.get_single().unwrap();
        let mouse_pos = window.cursor_position().unwrap();
        let (camera, camera_transform) = camera.get_single().unwrap();

        if let Some(world_position) = camera.viewport_to_world(camera_transform, mouse_pos) {
            let world_position = world_position.origin.truncate();
            let grid_pos = Map::get_grid_pos(world_position);
            let tower = Tower::new(
                1.0,
                1.0,
                TowerType::ChargeShot,
                Debuff::ReduceNeighbourDamage(20.0),
            );
            if map.is_valid_placement(grid_pos) {
                let entity = commands
                    .spawn(SpriteBundle {
                        texture: game_assets.charge_shot.clone(),
                        transform: Transform::from_translation(Vec3::new(
                            grid_pos.0 as f32 * 32.0,
                            grid_pos.1 as f32 * 32.0,
                            1.0,
                        )),
                        ..Default::default()
                    })
                    .insert(tower)
                    .insert(ChargeShot::new(grid_pos, 1.0, 0.5))
                    .with_children(|parent| {
                        // Circle used to show the range of the tower
                        parent.spawn(MaterialMesh2dBundle {
                            mesh: meshes.add(shape::Circle::new(1.5 * 32.0).into()).into(),
                            material: materials
                                .add(ColorMaterial::from(Color::rgba(1.0, 0.0, 0.0, 0.2))),
                            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)),
                            visibility: Visibility::Hidden,
                            ..Default::default()
                        });
                    })
                    .id();
                map.place_tower(grid_pos, entity).unwrap();
                event_writer.send(TowerPlaced { grid_pos });
            }
        }
    }
}

pub fn mouse_hover_handler(
    map: Res<Map>,
    mut cursor_events: EventReader<CursorMoved>,
    camera: Query<(&Camera, &GlobalTransform)>,
    tower_query: Query<&Children, With<Tower>>,
    mut children_query: Query<&mut Visibility>,
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
                        let children = tower_query.get(*tower).unwrap();
                        for child in children.iter() {
                            let mut visibility = children_query.get_mut(*child).unwrap();
                            *visibility = Visibility::Hidden;
                        }
                    }
                }
            }
            // Show the range of the newly hovered tower
            if let Some(tower) = map.placements.get(&grid_pos) {
                let children = tower_query.get(*tower).unwrap();
                for child in children.iter() {
                    let mut visibility = children_query.get_mut(*child).unwrap();
                    *visibility = Visibility::Visible;
                }
                ui_state.hovered_pos = Some(grid_pos);
            } else {
                ui_state.hovered_pos = None;
            }
        }
    }
}
