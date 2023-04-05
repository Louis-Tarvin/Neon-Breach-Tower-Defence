use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{enemies::Enemy, grid::Map, state::loading::GameAssets};

use super::{Tower, TowerPlaced};

#[derive(Component, Debug)]
pub struct ChargeShot {
    pub range: f32,
    pub timer: Timer,
}
impl ChargeShot {
    pub fn new(range: f32, cooldown: f32) -> Self {
        Self {
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

#[derive(Component)]
pub struct RangeIndicator {}

pub fn shoot(
    mut commands: Commands,
    mut query: Query<(&Tower, &mut ChargeShot, &Transform), Without<Enemy>>,
    enemies: Query<(&Enemy, &Transform), Without<Tower>>,
    map: Res<Map>,
    time: Res<Time>,
    game_assets: Res<GameAssets>,
) {
    for (tower, mut charge_shot, transform) in query.iter_mut() {
        charge_shot.timer.tick(time.delta());
        if charge_shot.timer.finished() {
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
            if transform.translation.distance(enemy_transform.translation) < 15.0 {
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

pub fn spawn_charge_shot(
    tower: Tower,
    mut commands: Commands,
    grid_pos: (i8, i8),
    game_assets: Res<GameAssets>,
    mut event_writer: EventWriter<TowerPlaced>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut map: ResMut<Map>,
) {
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
        .insert(ChargeShot::new(1.0, 0.5))
        .with_children(|parent| {
            // Circle used to show the range of the tower
            parent
                .spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(1.5 * 32.0).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::rgba(1.0, 0.0, 0.0, 0.2))),
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)),
                    visibility: Visibility::Hidden,
                    ..Default::default()
                })
                .insert(RangeIndicator {});
        })
        .id();
    map.place_tower(grid_pos, entity).unwrap();
    event_writer.send(TowerPlaced { grid_pos });
}
