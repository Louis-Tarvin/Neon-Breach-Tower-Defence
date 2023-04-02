use bevy::prelude::*;

use crate::{
    grid::Map,
    state::loading::GameAssets,
    tower::{charge_shot::ChargeShot, Debuff, Tower, TowerType},
};

pub fn grid_click_handler(
    mut commands: Commands,
    mut map: ResMut<Map>,
    mouse_input: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
    game_assets: Res<GameAssets>,
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
            if map.place_tower(grid_pos, tower).is_ok() {
                commands
                    .spawn(SpriteBundle {
                        texture: game_assets.charge_shot.clone(),
                        transform: Transform::from_translation(Vec3::new(
                            grid_pos.0 as f32 * 32.0,
                            grid_pos.1 as f32 * 32.0,
                            1.0,
                        )),
                        ..Default::default()
                    })
                    .insert(ChargeShot::new(grid_pos, 1.0, 0.5));
            }
        }
    }
}
