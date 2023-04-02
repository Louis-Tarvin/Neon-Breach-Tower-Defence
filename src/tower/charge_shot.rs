use bevy::prelude::*;

use crate::{enemies::Enemy, grid::Map, state::loading::GameAssets};

use super::Tower;

#[derive(Component, Debug)]
pub struct ChargeShot {
    pub grid_pos: (i8, i8),
    pub range: f32,
    pub timer: Timer,
}
impl ChargeShot {
    pub fn new(grid_pos: (i8, i8), range: f32, cooldown: f32) -> Self {
        Self {
            grid_pos,
            range,
            timer: Timer::from_seconds(cooldown, TimerMode::Repeating),
        }
    }
}

#[derive(Component, Debug)]
pub struct ChargeShotProjectile {
    pub damage: f32,
    pub speed: f32,
    pub target: Entity,
}

pub fn shoot(
    mut commands: Commands,
    mut query: Query<(&mut ChargeShot, &Transform), Without<Enemy>>,
    enemies: Query<(&Enemy, &Transform), Without<Tower>>,
    map: Res<Map>,
    time: Res<Time>,
    game_assets: Res<GameAssets>,
) {
    for (mut charge_shot, transform) in query.iter_mut() {
        charge_shot.timer.tick(time.delta());
        if charge_shot.timer.finished() {
            let tower = map
                .placements
                .get(&charge_shot.grid_pos)
                .expect("Charge shot location mismatch");
            // Find the enemy that has travelled the furthest and is within range
            let max_range = charge_shot.range.ceil() as i32;
            let mut furthest_enemy = None;
            let mut furthest_distance = 0.0;
            for x in -max_range..=max_range {
                for y in -max_range..=max_range {
                    let grid_pos = Map::get_grid_pos(transform.translation.truncate());
                    let grid_x = grid_pos.0 as i32 + x;
                    let grid_y = grid_pos.1 as i32 + y;
                    if let Some(entities) = map.enemies.get(&(grid_x as i8, grid_y as i8)) {
                        for entity in entities {
                            if let Ok((enemy, enemy_transform)) = enemies.get(*entity) {
                                let distance =
                                    transform.translation.distance(enemy_transform.translation);
                                if distance <= charge_shot.range * 32.0 + 16.0
                                    && enemy.distance_travelled > furthest_distance
                                {
                                    furthest_distance = enemy.distance_travelled;
                                    furthest_enemy = Some(entity);
                                }
                            }
                        }
                    }
                }
            }
            if let Some(entity) = furthest_enemy {
                // Spawn projectile
                if let Ok((_enemy, enemy_transform)) = enemies.get(*entity) {
                    let direction =
                        (enemy_transform.translation - transform.translation).normalize();
                    let projectile_pos = transform.translation + direction * 0.5;
                    commands
                        .spawn(SpriteBundle {
                            texture: game_assets.bullet.clone(),
                            transform: Transform::from_translation(projectile_pos),
                            ..Default::default()
                        })
                        .insert(ChargeShotProjectile {
                            damage: tower.damage,
                            speed: 100.0,
                            target: *entity,
                        });
                }
            }
        }
    }
}

pub fn handle_projectiles(
    mut commands: Commands,
    mut query: Query<(Entity, &mut ChargeShotProjectile, &mut Transform), Without<Enemy>>,
    mut enemies: Query<(&mut Enemy, &Transform), Without<ChargeShotProjectile>>,
    time: Res<Time>,
) {
    for (entity, projectile, mut transform) in query.iter_mut() {
        if let Ok((mut enemy, enemy_transform)) = enemies.get_mut(projectile.target) {
            let direction = (enemy_transform.translation - transform.translation).normalize();
            transform.translation += direction * projectile.speed * time.delta_seconds();
            if transform.translation.distance(enemy_transform.translation) < 0.5 {
                // Hit enemy
                enemy.current_health -= projectile.damage;
                commands.entity(entity).despawn();
            }
        } else {
            // Enemy is dead
            commands.entity(entity).despawn();
        }
    }
}
