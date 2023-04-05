use bevy::prelude::*;

use crate::{grid::Map, state::loading::GameAssets, tower::debuffs::SpeedUpPoint};

#[derive(Debug, Component)]
pub struct Enemy {
    pub max_health: f32,
    pub current_health: f32,
    pub healthbar: Option<Entity>,
    pub move_speed: f32,
    pub path_target: usize,
    pub current_grid_pos: (i8, i8),
    pub distance_travelled: f32,
}
impl Enemy {
    pub fn new(health: f32, move_speed: f32, grid_pos: (i8, i8)) -> Self {
        Self {
            max_health: health,
            current_health: health,
            healthbar: None,
            move_speed,
            path_target: 0,
            current_grid_pos: grid_pos,
            distance_travelled: 0.0,
        }
    }
}

pub fn update_enemy_grid_pos(
    mut enemies: Query<(Entity, &mut Enemy, &Transform)>,
    mut map: ResMut<Map>,
) {
    for (entity, mut enemy, transform) in enemies.iter_mut() {
        let pos = transform.translation.truncate();
        let grid_pos = Map::get_grid_pos(pos);
        let grid_pos = (grid_pos.0, grid_pos.1);
        if grid_pos != enemy.current_grid_pos {
            if let Some(entities) = map.enemies.get_mut(&enemy.current_grid_pos) {
                entities.retain(|e| *e != entity);
            }
            enemy.current_grid_pos = (grid_pos.0, grid_pos.1);
            map.enemies
                .entry(enemy.current_grid_pos)
                .or_insert_with(Vec::new)
                .push(entity);
        }
    }
}

#[derive(Component)]
pub struct HealthBar(pub f32);

pub fn spawn_enemies(
    mut commands: Commands,
    map: Res<Map>,
    game_assets: Res<GameAssets>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        let spawn_pos = Map::grid_to_world_pos((map.start_pos.0 as f32, map.start_pos.1 as f32));
        commands
            .spawn(SpriteBundle {
                texture: game_assets.enemy.clone(),
                transform: Transform::from_translation(Vec3::new(spawn_pos.x, spawn_pos.y, 1.0)),
                ..Default::default()
            })
            .insert(Enemy::new(5.0, 20.0, map.start_pos));
    }
}

pub fn enemy_movement(
    mut commands: Commands,
    mut map: ResMut<Map>,
    mut enemies: Query<(&mut Enemy, &mut Transform, Entity), Without<SpeedUpPoint>>,
    speed_up_points: Query<(&SpeedUpPoint, &Transform), Without<Enemy>>,
    time: Res<Time>,
) {
    for (mut enemy, mut transform, entity) in enemies.iter_mut() {
        let mut distance_to_travel = enemy.move_speed * time.delta_seconds();
        for (speed_up_point, transform2) in speed_up_points.iter() {
            let distance = (transform2.translation - transform.translation).length();
            if distance <= 48.0 {
                distance_to_travel += (speed_up_point.0 / 100.0) * distance_to_travel;
            }
        }
        let mut current_pos = transform.translation.truncate();
        let next_pos = map.path[enemy.path_target];
        let next_pos = Map::grid_to_world_pos((next_pos.0 as f32, next_pos.1 as f32));
        let distance_to_next_pos =
            (next_pos.x - current_pos.x).abs() + (next_pos.y - current_pos.y).abs();

        if distance_to_travel >= distance_to_next_pos {
            // Enemy has reached the next point along the path
            current_pos = next_pos;
            transform.translation.x = current_pos.x;
            transform.translation.y = current_pos.y;
            enemy.path_target += 1;
            if enemy.path_target >= map.path.len() {
                // Enemy has reached the end
                // TODO: Game over checking
                map.enemies
                    .get_mut(&enemy.current_grid_pos)
                    .unwrap()
                    .retain(|e| *e != entity);
                commands.entity(entity).despawn_recursive();
            }
        } else if next_pos.x == current_pos.x {
            if next_pos.y > current_pos.y {
                transform.translation.y += distance_to_travel;
            } else {
                transform.translation.y -= distance_to_travel;
            }
        } else if next_pos.x > current_pos.x {
            transform.translation.x += distance_to_travel;
        } else {
            transform.translation.x -= distance_to_travel;
        }
        enemy.distance_travelled += distance_to_travel;
    }
}

pub fn update_healthbar(
    mut commands: Commands,
    mut enemies: Query<(Entity, &mut Enemy)>,
    mut healthbars: Query<&mut HealthBar>,
) {
    for (entity, mut enemy) in enemies.iter_mut() {
        if enemy.current_health < enemy.max_health {
            if let Some(hb_entity) = enemy.healthbar {
                if let Ok(mut healthbar) = healthbars.get_mut(hb_entity) {
                    healthbar.0 = enemy.current_health / enemy.max_health;
                }
            } else {
                let mut hb_entity = None;
                commands.entity(entity).with_children(|parent| {
                    parent
                        .spawn(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::new(22.0, 4.0)),
                                color: Color::rgb(0.0, 0.0, 0.0),
                                ..Default::default()
                            },
                            transform: Transform::from_translation(Vec3::new(0.0, -16.0, 6.0)),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            hb_entity = Some(
                                parent
                                    .spawn(SpriteBundle {
                                        sprite: Sprite {
                                            custom_size: Some(Vec2::new(20.0, 2.0)),
                                            color: Color::rgb(0.0, 1.0, 0.0),
                                            ..Default::default()
                                        },
                                        transform: Transform::from_translation(Vec3::new(
                                            0.0, 0.0, 1.0,
                                        )),
                                        ..Default::default()
                                    })
                                    .insert(HealthBar(enemy.current_health / enemy.max_health))
                                    .id(),
                            );
                        });
                });
                enemy.healthbar = hb_entity;
            }
        }
    }
}

pub fn scale_healthbar(mut healthbars: Query<(&HealthBar, &mut Transform), Changed<HealthBar>>) {
    for (healthbar, mut transform) in healthbars.iter_mut() {
        transform.scale.x = healthbar.0;
        transform.translation.x = -10.0 + (10.0 * healthbar.0);
    }
}

pub fn check_killed(
    mut commands: Commands,
    enemies: Query<(Entity, &Enemy)>,
    mut map: ResMut<Map>,
) {
    for (entity, enemy) in enemies.iter() {
        if enemy.current_health <= 0.0 {
            if let Some(entities) = map.enemies.get_mut(&enemy.current_grid_pos) {
                entities.retain(|e| *e != entity);
            }
            commands.entity(entity).despawn_recursive();
        }
    }
}
