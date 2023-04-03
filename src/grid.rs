use std::collections::HashMap;

use bevy::prelude::*;

use crate::state::loading::GameAssets;

#[derive(Resource, Debug)]
pub struct Map {
    pub grid: Vec<Vec<u8>>,
    pub start_pos: (i8, i8),
    pub placements: HashMap<(i8, i8), Entity>,
    pub enemies: HashMap<(i8, i8), Vec<Entity>>,
    pub path: Vec<(u8, u8)>,
}
impl Map {
    pub fn new(grid: Vec<Vec<u8>>, start_pos: (i8, i8), path: Vec<(u8, u8)>) -> Self {
        Self {
            grid,
            start_pos,
            placements: HashMap::new(),
            enemies: HashMap::new(),
            path,
        }
    }

    pub fn get_grid_pos(pos: Vec2) -> (i8, i8) {
        let x = pos.x + 16.0;
        let y = pos.y + 16.0;
        ((x / 32.0) as i8, (y / 32.0) as i8)
    }

    pub fn grid_to_world_pos(pos: (f32, f32)) -> Vec2 {
        let (x, y) = pos;
        Vec2::new(x * 32.0, y * 32.0)
    }

    pub fn is_valid_placement(&self, pos: (i8, i8)) -> bool {
        let (x, y) = pos;
        if (x as usize) >= self.grid[0].len()
            || (y as usize) >= self.grid.len()
            || x < 0
            || y < 0
            || self.grid[y as usize][x as usize] == 1
            || self.placements.contains_key(&(x, y))
        {
            return false;
        }
        true
    }

    pub fn place_tower(&mut self, pos: (i8, i8), entity: Entity) -> Result<(), ()> {
        let (x, y) = pos;
        if self.is_valid_placement(pos) {
            self.placements.insert((x, y), entity);
            return Ok(());
        }

        Err(())
    }
}

pub fn load_map(mut commands: Commands, game_assets: Res<GameAssets>) {
    let map = Map::new(
        vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 1, 1, 1, 0, 0],
            vec![0, 1, 0, 1, 0, 0],
            vec![0, 1, 0, 1, 1, 1],
            vec![1, 1, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
        ],
        (-1, 4),
        vec![(1, 4), (1, 1), (3, 1), (3, 3), (6, 3)],
    );

    for (y, row) in map.grid.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if *tile == 1 {
                commands.spawn(SpriteBundle {
                    texture: game_assets.empty_tile.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        x as f32 * 32.0,
                        y as f32 * 32.0,
                        0.0,
                    )),
                    ..Default::default()
                });
            } else {
                commands.spawn(SpriteBundle {
                    texture: game_assets.buildable_tile.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        x as f32 * 32.0,
                        y as f32 * 32.0,
                        0.0,
                    )),
                    ..Default::default()
                });
            }
        }
    }
    commands.insert_resource(map);
}
