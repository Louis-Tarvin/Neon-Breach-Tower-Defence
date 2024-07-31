use std::time::Duration;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{enemies::Enemy, grid::Map, state::loading::GameAssets};

use super::{Tower, TowerPlaced};

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Debug)]
pub struct Laser {
    pub direction: Direction,
    pub timer: Timer,
}
impl Laser {
    pub fn new(direction: Direction, rate: f32) -> Self {
        Self {
            direction,
            timer: Timer::from_seconds(1.0 / rate, TimerMode::Once),
        }
    }

    pub fn toggle_direction(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

pub fn shoot(
    mut query: Query<(&Tower, &mut Laser, &Transform), Without<Enemy>>,
    mut enemies: Query<&mut Enemy, Without<Tower>>,
    map: Res<Map>,
    time: Res<Time>,
) {
    for (tower, mut laser, transform) in query.iter_mut() {
        laser.timer.tick(time.delta());
        if laser.timer.finished() {
            laser
                .timer
                .set_duration(Duration::from_secs_f32(1.0 / tower.rate));
            laser.timer.reset();
            if tower.overheating {
                continue; // Don't shoot if overheating
            }
            let grid_pos = Map::get_grid_pos(transform.translation.truncate());
            match laser.direction {
                Direction::Up => {
                    let mut current_y = grid_pos.1 + 1;
                    while current_y < map.height.try_into().unwrap() {
                        if let Some(grid_enemies) = map.enemies.get(&(grid_pos.0, current_y)) {
                            for entity in grid_enemies {
                                if let Ok(mut enemy) = enemies.get_mut(*entity) {
                                    enemy.current_health -= tower.damage;
                                }
                            }
                        }
                        current_y += 1;
                    }
                }
                Direction::Down => {
                    let mut current_y = grid_pos.1 - 1;
                    while current_y >= 0 {
                        if let Some(grid_enemies) = map.enemies.get(&(grid_pos.0, current_y)) {
                            for entity in grid_enemies {
                                if let Ok(mut enemy) = enemies.get_mut(*entity) {
                                    enemy.current_health -= tower.damage;
                                }
                            }
                        }
                        current_y -= 1;
                    }
                }
                Direction::Left => {
                    let mut current_x = grid_pos.0 - 1;
                    while current_x >= 0 {
                        if let Some(grid_enemies) = map.enemies.get(&(current_x, grid_pos.1)) {
                            for entity in grid_enemies {
                                if let Ok(mut enemy) = enemies.get_mut(*entity) {
                                    enemy.current_health -= tower.damage;
                                }
                            }
                        }
                        current_x -= 1;
                    }
                }
                Direction::Right => {
                    let mut current_x = grid_pos.0 + 1;
                    while current_x < map.width.try_into().unwrap() {
                        if let Some(grid_enemies) = map.enemies.get(&(current_x, grid_pos.1)) {
                            for entity in grid_enemies {
                                if let Ok(mut enemy) = enemies.get_mut(*entity) {
                                    enemy.current_health -= tower.damage;
                                }
                            }
                        }
                        current_x += 1;
                    }
                }
            }
        }
    }
}

pub fn spawn_laser(
    tower: Tower,
    mut commands: Commands,
    grid_pos: (i8, i8),
    direction: Direction,
    game_assets: Res<GameAssets>,
    mut event_writer: EventWriter<TowerPlaced>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    mut map: ResMut<Map>,
) {
    let rotation = match direction {
        Direction::Up => Quat::from_rotation_z(std::f32::consts::PI),
        Direction::Down => Quat::from_rotation_z(0.0),
        Direction::Left => Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2),
        Direction::Right => Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
    };
    let entity = commands
        .spawn(SpriteBundle {
            texture: game_assets.laser.clone(),
            transform: Transform::from_translation(Vec3::new(
                grid_pos.0 as f32 * 32.0,
                grid_pos.1 as f32 * 32.0,
                1.0,
            ))
            .with_rotation(rotation),
            ..Default::default()
        })
        .insert(Laser::new(direction, tower.rate))
        .insert(tower)
        .with_children(|parent| {
            // Laser beam
            spawn_laser_beam(parent, grid_pos, direction, meshes, materials, &map);
        })
        .id();
    map.place_tower(grid_pos, entity).unwrap();
    event_writer.send(TowerPlaced { grid_pos });
}

#[derive(Component)]
pub struct LaserBeam;

pub fn spawn_laser_beam(
    parent: &mut ChildBuilder,
    grid_pos: (i8, i8),
    direction: Direction,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    map: &Map,
) {
    let rotation = match direction {
        Direction::Up => Quat::from_rotation_z(std::f32::consts::PI),
        Direction::Down => Quat::from_rotation_z(0.0),
        Direction::Left => Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2),
        Direction::Right => Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
    };
    let (beam_dimensions, beam_location) = match direction {
        Direction::Up => {
            let height = (map.height as i8 - grid_pos.1) as f32 * 32.0 - 16.0;
            let y = (map.height as i8 - grid_pos.1 - 1) as f32 * 32.0 - height / 2.0;
            (Vec2::new(5.0, height), Vec3::new(0.0, -y - 16.0, 2.0))
        }
        Direction::Down => {
            let height = (grid_pos.1 + 1) as f32 * 32.0 - 16.0;
            let y = grid_pos.1 as f32 * 32.0 - height / 2.0;
            (Vec2::new(5.0, height), Vec3::new(0.0, -y - 16.0, 2.0))
        }
        Direction::Left => {
            let width = (grid_pos.0 + 1) as f32 * 32.0 - 16.0;
            let y = grid_pos.0 as f32 * 32.0 - width / 2.0;
            (Vec2::new(width, 5.0), Vec3::new(0.0, -y - 16.0, 2.0))
        }
        Direction::Right => {
            let width = (map.width as i8 - grid_pos.0) as f32 * 32.0 - 16.0;
            let y = (map.width as i8 - grid_pos.0 - 1) as f32 * 32.0 - width / 2.0;
            (Vec2::new(width, 5.0), Vec3::new(0.0, -y - 16.0, 2.0))
        }
    };
    parent
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::from_size(beam_dimensions)).into(),
            material: materials.add(ColorMaterial::from(Color::rgba(
                1.74 * 2.0,
                1.15 * 2.0,
                0.74 * 2.0,
                1.0,
            ))),
            transform: Transform::from_translation(beam_location).with_rotation(-rotation),
            ..Default::default()
        })
        .insert(LaserBeam);
}
