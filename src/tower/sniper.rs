use std::time::Duration;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_kira_audio::{AudioChannel, AudioControl};
use rand::seq::SliceRandom;

use crate::{
    audio::{AudioAssets, SoundChannel},
    enemies::Enemy,
    grid::Map,
    state::loading::GameAssets,
    ui::constants::RED,
};

use super::{Projectile, RangeIndicator, RotatingTurret, TargetMode, Tower, TowerPlaced};

#[derive(Component, Debug)]
pub struct Sniper {
    pub range: f32,
    pub timer: Timer,
}
impl Sniper {
    pub fn new(range: f32, rate: f32) -> Self {
        Self {
            range,
            timer: Timer::from_seconds(1.0 / rate, TimerMode::Once),
        }
    }
}

pub fn shoot(
    mut commands: Commands,
    mut query: Query<(&Tower, &mut Sniper, &Transform, &Children), Without<Enemy>>,
    enemies: Query<(&Enemy, &Transform), Without<Tower>>,
    mut rotator_query: Query<
        &mut Transform,
        (With<RotatingTurret>, Without<Tower>, Without<Enemy>),
    >,
    map: Res<Map>,
    time: Res<Time>,
    game_assets: Res<GameAssets>,
    sound_channel: Res<AudioChannel<SoundChannel>>,
    audio_assets: Res<AudioAssets>,
) {
    for (tower, mut sniper, transform, children) in query.iter_mut() {
        sniper.timer.tick(time.delta());
        if sniper.timer.finished() {
            sniper
                .timer
                .set_duration(Duration::from_secs_f32(1.0 / tower.rate));
            sniper.timer.reset();
            if tower.overheating {
                continue; // Don't shoot if overheating
            }
            // Find the enemy that has travelled the furthest and is within range
            let max_range = sniper.range.ceil() as i32;
            let mut target_enemy = None;
            match tower.target_mode {
                TargetMode::First => {
                    let mut furthest_distance = 0.0;
                    for x in -max_range..=max_range {
                        for y in -max_range..=max_range {
                            let grid_pos = Map::get_grid_pos(transform.translation.truncate());
                            let grid_x = grid_pos.0 as i32 + x;
                            let grid_y = grid_pos.1 as i32 + y;
                            if let Some(entities) = map.enemies.get(&(grid_x as i8, grid_y as i8)) {
                                for entity in entities {
                                    if let Ok((enemy, enemy_transform)) = enemies.get(*entity) {
                                        let distance = transform
                                            .translation
                                            .distance(enemy_transform.translation);
                                        if distance <= sniper.range * 32.0 + 16.0
                                            && enemy.distance_travelled > furthest_distance
                                        {
                                            furthest_distance = enemy.distance_travelled;
                                            target_enemy = Some(entity);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                TargetMode::Closest => {
                    let mut closest_distance = std::f32::MAX;
                    for x in -max_range..=max_range {
                        for y in -max_range..=max_range {
                            let grid_pos = Map::get_grid_pos(transform.translation.truncate());
                            let grid_x = grid_pos.0 as i32 + x;
                            let grid_y = grid_pos.1 as i32 + y;
                            if let Some(entities) = map.enemies.get(&(grid_x as i8, grid_y as i8)) {
                                for entity in entities {
                                    if let Ok((_enemy, enemy_transform)) = enemies.get(*entity) {
                                        let distance = transform
                                            .translation
                                            .distance(enemy_transform.translation);
                                        if distance <= sniper.range * 32.0 + 16.0
                                            && distance < closest_distance
                                        {
                                            closest_distance = distance;
                                            target_enemy = Some(entity);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                TargetMode::Random => {
                    let mut rng = rand::thread_rng();
                    let mut possible_targets = Vec::new();
                    for x in -max_range..=max_range {
                        for y in -max_range..=max_range {
                            let grid_pos = Map::get_grid_pos(transform.translation.truncate());
                            let grid_x = grid_pos.0 as i32 + x;
                            let grid_y = grid_pos.1 as i32 + y;
                            if let Some(entities) = map.enemies.get(&(grid_x as i8, grid_y as i8)) {
                                for entity in entities {
                                    if let Ok((_enemy, enemy_transform)) = enemies.get(*entity) {
                                        let distance = transform
                                            .translation
                                            .distance(enemy_transform.translation);
                                        if distance <= sniper.range * 32.0 + 16.0 {
                                            possible_targets.push(entity);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if !possible_targets.is_empty() {
                        target_enemy = Some(*possible_targets.choose(&mut rng).unwrap());
                    }
                }
            }
            if let Some(entity) = target_enemy {
                // Spawn projectile
                if let Ok((_enemy, enemy_transform)) = enemies.get(*entity) {
                    let direction =
                        (enemy_transform.translation - transform.translation).normalize();
                    let mut projectile_pos = transform.translation + direction * 0.5;
                    projectile_pos.z = 2.0;
                    commands
                        .spawn(SpriteBundle {
                            texture: game_assets.bullet.clone(),
                            sprite: Sprite {
                                color: RED * 5.0,
                                ..Default::default()
                            },
                            transform: Transform::from_translation(projectile_pos),
                            ..Default::default()
                        })
                        .insert(Projectile {
                            damage: tower.damage,
                            speed: 350.0,
                            target: *entity,
                        });
                    // Rotate the turret
                    for child in children.iter() {
                        if let Ok(mut rotator) = rotator_query.get_mut(*child) {
                            rotator.rotation = Quat::from_rotation_z(
                                direction.y.atan2(direction.x) - std::f32::consts::FRAC_PI_2,
                            );
                        }
                    }
                    sound_channel.play(audio_assets.sniper_shoot.clone());
                }
            }
        }
    }
}

pub fn spawn_sniper(
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
            texture: game_assets.pivot.clone(),
            transform: Transform::from_translation(Vec3::new(
                grid_pos.0 as f32 * 32.0,
                grid_pos.1 as f32 * 32.0,
                1.0,
            )),
            ..Default::default()
        })
        .insert(Sniper::new(3.0, tower.rate))
        .insert(tower)
        .with_children(|parent| {
            // Circle used to show the range of the tower
            parent
                .spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(Circle::new(3.5 * 32.0)).into(),
                    material: materials.add(ColorMaterial::from(Color::rgba(0.8, 0.4, 0.4, 0.2))),
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)),
                    visibility: Visibility::Hidden,
                    ..Default::default()
                })
                .insert(RangeIndicator);
            parent
                .spawn(SpriteBundle {
                    texture: game_assets.sniper.clone(),
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                    ..Default::default()
                })
                .insert(RotatingTurret);
        })
        .id();
    map.place_tower(grid_pos, entity).unwrap();
    event_writer.send(TowerPlaced { grid_pos });
}
