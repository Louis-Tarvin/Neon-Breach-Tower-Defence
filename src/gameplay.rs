use std::time::Duration;

use bevy::prelude::*;

use crate::{
    enemies::{Enemy, EnemyVariant},
    grid::Map,
    state::loading::GameAssets,
    tower::Tower,
    ui::{tower_options::present_tower_options, UiState, UiStateResource},
};

pub struct WaveSegment {
    pub enemy_type: EnemyVariant,
    pub count: usize,
    pub spawn_rate: f32,
}

pub struct Wave {
    pub segments: Vec<WaveSegment>,
}

pub enum WaveState {
    /// Waiting for the next wave to start.
    Waiting,
    /// The wave is currently spawning enemies. Contains the number of enemies spawned so far.
    Spawning(u16),
    /// The wave is finished, but there are still enemies alive.
    Finished,
}

#[derive(Resource)]
pub struct GameManager {
    pub current_wave: usize,
    pub waves: Vec<Wave>,
    pub wave_state: WaveState,
    pub spawn_timer: Timer,
}
impl GameManager {
    pub fn new() -> Self {
        Self {
            current_wave: 0,
            waves: vec![
                Wave { segments: vec![] },
                Wave {
                    segments: vec![WaveSegment {
                        enemy_type: EnemyVariant::Normal,
                        count: 10,
                        spawn_rate: 0.5,
                    }],
                },
                Wave {
                    segments: vec![
                        WaveSegment {
                            enemy_type: EnemyVariant::Normal,
                            count: 10,
                            spawn_rate: 1.0,
                        },
                        WaveSegment {
                            enemy_type: EnemyVariant::Fast,
                            count: 5,
                            spawn_rate: 0.5,
                        },
                    ],
                },
                Wave {
                    segments: vec![
                        WaveSegment {
                            enemy_type: EnemyVariant::Normal,
                            count: 10,
                            spawn_rate: 0.5,
                        },
                        WaveSegment {
                            enemy_type: EnemyVariant::Fast,
                            count: 10,
                            spawn_rate: 0.5,
                        },
                        WaveSegment {
                            enemy_type: EnemyVariant::Strong,
                            count: 10,
                            spawn_rate: 0.5,
                        },
                    ],
                },
                Wave {
                    segments: vec![WaveSegment {
                        enemy_type: EnemyVariant::Boss,
                        count: 1,
                        spawn_rate: 0.5,
                    }],
                },
            ],
            wave_state: WaveState::Waiting,
            spawn_timer: Timer::from_seconds(0.1, TimerMode::Once),
        }
    }
}

pub fn gameloop(
    mut commands: Commands,
    mut game_manager: ResMut<GameManager>,
    enemies: Query<&Enemy>,
    map: Res<Map>,
    time: Res<Time>,
    mut ui_state: ResMut<UiStateResource>,
    game_assets: Res<GameAssets>,
) {
    match game_manager.wave_state {
        WaveState::Spawning(num) => {
            game_manager.spawn_timer.tick(time.delta());
            if game_manager.spawn_timer.finished() {
                let mut finished = true;
                let wave = &game_manager.waves[game_manager.current_wave];
                let mut count = 0;
                for segment in wave.segments.iter() {
                    if count + segment.count > num.into() {
                        let spawn_pos = Map::grid_to_world_pos((
                            map.start_pos.0 as f32,
                            map.start_pos.1 as f32,
                        ));
                        println!("Spawning enemy of type {:?}", segment.enemy_type);
                        commands
                            .spawn(SpriteBundle {
                                texture: game_assets.enemy.clone(),
                                transform: Transform::from_translation(Vec3::new(
                                    spawn_pos.x,
                                    spawn_pos.y,
                                    1.0,
                                )),
                                ..Default::default()
                            })
                            .insert(Enemy::new_variant(segment.enemy_type, map.start_pos));
                        let cooldown = 1.0 / segment.spawn_rate;
                        game_manager
                            .spawn_timer
                            .set_duration(Duration::from_secs_f32(cooldown));
                        game_manager.spawn_timer.reset();
                        game_manager.wave_state = WaveState::Spawning(num + 1);
                        finished = false;
                        break;
                    } else {
                        count += segment.count;
                    }
                }
                if finished {
                    game_manager.wave_state = WaveState::Finished;
                }
            }
        }
        WaveState::Finished => {
            if enemies.iter().count() == 0 {
                println!("Wave {} finished", game_manager.current_wave);
                game_manager.current_wave += 1;
                game_manager.wave_state = WaveState::Waiting;
                let mut options = Vec::new();
                for _ in 0..3 {
                    options.push(Tower::new_random());
                }
                present_tower_options(commands, game_assets.font.clone(), &options);
                ui_state.state = UiState::PickingTower(options);
            }
        }
        _ => {}
    }
}

pub fn start_next_wave(input: Res<Input<KeyCode>>, mut game_manager: ResMut<GameManager>) {
    if input.just_pressed(KeyCode::Space) && game_manager.current_wave < game_manager.waves.len() {
        if let WaveState::Waiting = game_manager.wave_state {
            game_manager.wave_state = WaveState::Spawning(0);
        }
    }
}
