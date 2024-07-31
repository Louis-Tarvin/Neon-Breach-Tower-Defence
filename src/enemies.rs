use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};

use crate::{
    audio::{AudioAssets, SoundChannel},
    gameplay::GameManager,
    grid::Map,
    tower::debuffs::SpeedUpPoint,
    ui::constants::GREEN,
};

#[derive(Debug, Clone, Copy)]
pub enum EnemyVariant {
    Weak,
    Normal,
    Fast,
    Strong,
    Boss,
    StrongFast,
    UltraBoss,
}
impl EnemyVariant {
    pub fn points(&self) -> u32 {
        match self {
            Self::Weak => 1,
            Self::Normal => 2,
            Self::Fast => 4,
            Self::Strong => 10,
            Self::Boss => 20,
            Self::StrongFast => 15,
            Self::UltraBoss => 50,
        }
    }
}

#[derive(Debug, Component)]
pub struct Enemy {
    pub variant: EnemyVariant,
    pub max_health: f32,
    pub current_health: f32,
    pub healthbar: Option<Entity>,
    pub move_speed: f32,
    pub path_target: usize,
    pub current_grid_pos: (i8, i8),
    pub distance_travelled: f32,
}
impl Enemy {
    pub fn new(
        variant: EnemyVariant,
        grid_pos: (i8, i8),
        health_multiplier: f32,
        speed_multiplier: f32,
    ) -> Self {
        match variant {
            EnemyVariant::Weak => Self {
                variant,
                max_health: 2.0 * health_multiplier,
                current_health: 2.0 * health_multiplier,
                healthbar: None,
                move_speed: 15.0 * speed_multiplier,
                path_target: 0,
                current_grid_pos: grid_pos,
                distance_travelled: 0.0,
            },
            EnemyVariant::Normal => Self {
                variant,
                max_health: 5.0 * health_multiplier,
                current_health: 5.0 * health_multiplier,
                healthbar: None,
                move_speed: 20.0 * speed_multiplier,
                path_target: 0,
                current_grid_pos: grid_pos,
                distance_travelled: 0.0,
            },
            EnemyVariant::Fast => Self {
                variant,
                max_health: 5.0 * health_multiplier,
                current_health: 5.0 * health_multiplier,
                healthbar: None,
                move_speed: 40.0 * speed_multiplier,
                path_target: 0,
                current_grid_pos: grid_pos,
                distance_travelled: 0.0,
            },
            EnemyVariant::Strong => Self {
                variant,
                max_health: 20.0 * health_multiplier,
                current_health: 20.0 * health_multiplier,
                healthbar: None,
                move_speed: 15.0 * speed_multiplier,
                path_target: 0,
                current_grid_pos: grid_pos,
                distance_travelled: 0.0,
            },
            EnemyVariant::Boss => Self {
                variant,
                max_health: 100.0 * health_multiplier,
                current_health: 100.0 * health_multiplier,
                healthbar: None,
                move_speed: 10.0 * speed_multiplier,
                path_target: 0,
                current_grid_pos: grid_pos,
                distance_travelled: 0.0,
            },
            EnemyVariant::StrongFast => Self {
                variant,
                max_health: 20.0 * health_multiplier,
                current_health: 20.0 * health_multiplier,
                healthbar: None,
                move_speed: 35.0 * speed_multiplier,
                path_target: 0,
                current_grid_pos: grid_pos,
                distance_travelled: 0.0,
            },
            EnemyVariant::UltraBoss => Self {
                variant,
                max_health: 250.0 * health_multiplier,
                current_health: 250.0 * health_multiplier,
                healthbar: None,
                move_speed: 10.0 * speed_multiplier,
                path_target: 0,
                current_grid_pos: grid_pos,
                distance_travelled: 0.0,
            },
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

pub fn enemy_movement(
    mut commands: Commands,
    mut map: ResMut<Map>,
    mut enemies: Query<(&mut Enemy, &mut Transform, Entity), Without<SpeedUpPoint>>,
    speed_up_points: Query<(&SpeedUpPoint, &Transform), Without<Enemy>>,
    mut game_manager: ResMut<GameManager>,
    time: Res<Time>,
    sound_channel: Res<AudioChannel<SoundChannel>>,
    audio_assets: Res<AudioAssets>,
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
                if game_manager.lives > 0 {
                    game_manager.lives -= 1;
                }
                map.enemies
                    .get_mut(&enemy.current_grid_pos)
                    .unwrap()
                    .retain(|e| *e != entity);
                commands.entity(entity).despawn_recursive();
                sound_channel.play(audio_assets.end.clone());
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
        if enemy.current_health < enemy.max_health && enemy.current_health > 0.0 {
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
                                color: Color::srgb(0.0, 0.0, 0.0),
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
                                            color: GREEN,
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
    mut game_manager: ResMut<GameManager>,
    sound_channel: Res<AudioChannel<SoundChannel>>,
    audio_assets: Res<AudioAssets>,
) {
    for (entity, enemy) in enemies.iter() {
        if enemy.current_health <= 0.0 {
            if let Some(entities) = map.enemies.get_mut(&enemy.current_grid_pos) {
                entities.retain(|e| *e != entity);
            }
            commands.entity(entity).despawn_recursive();
            game_manager.score += enemy.variant.points();
            sound_channel.play(audio_assets.kill.clone());
        }
    }
}
