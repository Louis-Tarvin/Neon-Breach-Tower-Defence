use std::time::Duration;

use bevy::prelude::*;

use crate::{enemies::Enemy, grid::Map, state::loading::GameAssets};

use super::{Tower, TowerPlaced};

#[derive(Component)]
pub struct Silo {
    pub timer: Timer,
}

pub fn spawn_silo(
    tower: Tower,
    mut commands: Commands,
    grid_pos: (i8, i8),
    game_assets: Res<GameAssets>,
    mut event_writer: EventWriter<TowerPlaced>,
    mut map: ResMut<Map>,
) {
    let entity = commands
        .spawn(SpriteBundle {
            texture: game_assets.silo.clone(),
            transform: Transform::from_translation(Vec3::new(
                grid_pos.0 as f32 * 32.0,
                grid_pos.1 as f32 * 32.0,
                1.0,
            )),
            ..Default::default()
        })
        .insert(Silo {
            timer: Timer::from_seconds(1.0 / tower.rate, TimerMode::Once),
        })
        .insert(tower)
        .id();
    map.place_tower(grid_pos, entity).unwrap();
    event_writer.send(TowerPlaced { grid_pos });
}

#[derive(Component)]
pub struct Missile {
    pub target: Option<Entity>,
    pub damage: f32,
    pub speed: f32,
    pub rotation_speed: f32,
}

pub fn spawn_missile(
    mut commands: Commands,
    mut query: Query<(&mut Silo, &Tower, &Transform)>,
    game_assets: Res<GameAssets>,
    time: Res<Time>,
) {
    for (mut silo, tower, transform) in query.iter_mut() {
        silo.timer.tick(time.delta());
        if silo.timer.finished() {
            silo.timer
                .set_duration(Duration::from_secs_f32(1.0 / tower.rate));
            silo.timer.reset();
            if tower.overheating {
                continue; // Don't shoot if overheating
            }
            let grid_pos = Map::get_grid_pos(transform.translation.truncate());
            commands
                .spawn(SpriteBundle {
                    texture: game_assets.missile.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        grid_pos.0 as f32 * 32.0,
                        grid_pos.1 as f32 * 32.0,
                        4.0,
                    )),
                    ..Default::default()
                })
                .insert(Missile {
                    target: None,
                    damage: tower.damage,
                    speed: 100.0,
                    rotation_speed: 4.0,
                });
        }
    }
}

pub fn handle_missile(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Missile, &mut Transform), Without<Enemy>>,
    mut enemies: Query<(Entity, &mut Enemy, &Transform), Without<Missile>>,
    time: Res<Time>,
) {
    fn find_next_target(
        enemies: &mut Query<(Entity, &mut Enemy, &Transform), Without<Missile>>,
    ) -> Option<Entity> {
        let mut strongest_enemy = None;
        let mut strongest_enemy_health = 0.0;
        for (entity, enemy, _enemy_transform) in enemies.iter() {
            if enemy.current_health > strongest_enemy_health {
                strongest_enemy_health = enemy.current_health;
                strongest_enemy = Some(entity);
            }
        }
        strongest_enemy
    }
    for (entity, mut missile, mut transform) in query.iter_mut() {
        if let Some(target) = missile.target {
            if let Ok((_enemy_entity, mut enemy, enemy_transform)) = enemies.get_mut(target) {
                let target_direction = enemy_transform.translation - transform.translation;
                let distance = target_direction.length();
                let target_direction = target_direction.normalize();
                let current_direction = transform.rotation * Vec3::new(0.0, 1.0, 0.0);
                let rotation = current_direction.angle_between(target_direction);
                if rotation > 0.0 {
                    let rotation_direction = current_direction.cross(target_direction).z;
                    let rotation_direction = if rotation_direction > 0.0 { 1.0 } else { -1.0 };
                    transform.rotate(Quat::from_rotation_z(
                        rotation_direction * missile.rotation_speed * time.delta_seconds(),
                    ));
                }
                transform.translation += current_direction * missile.speed * time.delta_seconds();
                if distance < 15.0 {
                    enemy.current_health -= missile.damage;
                    commands.entity(entity).despawn();
                }
            } else {
                let strongest_enemy = find_next_target(&mut enemies);
                match strongest_enemy {
                    Some(entity) => missile.target = Some(entity),
                    None => commands.entity(entity).despawn(),
                }
            }
        } else {
            let strongest_enemy = find_next_target(&mut enemies);
            match strongest_enemy {
                Some(entity) => missile.target = Some(entity),
                None => commands.entity(entity).despawn(),
            }
        }
    }
}
