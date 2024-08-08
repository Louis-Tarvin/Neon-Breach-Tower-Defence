use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};

use crate::{
    audio::{DrumsChannel, VolumeSettings},
    enemies::{Enemy, EnemyVariant},
    grid::Map,
    state::{loading::GameAssets, results::Scores, State},
    tower::{debuffs::Debuff, Tower, TowerType},
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
    pub lives: u16,
    pub score: u32,
    pub health_multiplier: f32,
    pub speed_multiplier: f32,
}
impl GameManager {
    pub fn new() -> Self {
        Self {
            current_wave: 0,
            waves: vec![
                Wave { segments: vec![] },
                Wave {
                    segments: vec![WaveSegment {
                        enemy_type: EnemyVariant::Weak,
                        count: 5,
                        spawn_rate: 0.5,
                    }],
                },
                Wave {
                    segments: vec![
                        WaveSegment {
                            enemy_type: EnemyVariant::Weak,
                            count: 5,
                            spawn_rate: 1.0,
                        },
                        WaveSegment {
                            enemy_type: EnemyVariant::Normal,
                            count: 3,
                            spawn_rate: 0.5,
                        },
                    ],
                },
                Wave {
                    segments: vec![WaveSegment {
                        enemy_type: EnemyVariant::Normal,
                        count: 10,
                        spawn_rate: 0.75,
                    }],
                },
                Wave {
                    segments: vec![
                        WaveSegment {
                            enemy_type: EnemyVariant::Normal,
                            count: 8,
                            spawn_rate: 1.0,
                        },
                        WaveSegment {
                            enemy_type: EnemyVariant::Fast,
                            count: 3,
                            spawn_rate: 0.5,
                        },
                    ],
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
                        WaveSegment {
                            enemy_type: EnemyVariant::Normal,
                            count: 5,
                            spawn_rate: 1.5,
                        },
                    ],
                },
                Wave {
                    segments: vec![WaveSegment {
                        enemy_type: EnemyVariant::Fast,
                        count: 15,
                        spawn_rate: 0.8,
                    }],
                },
                Wave {
                    segments: vec![
                        WaveSegment {
                            enemy_type: EnemyVariant::Normal,
                            count: 15,
                            spawn_rate: 1.2,
                        },
                        WaveSegment {
                            enemy_type: EnemyVariant::Strong,
                            count: 1,
                            spawn_rate: 0.7,
                        },
                    ],
                },
                Wave {
                    segments: vec![
                        WaveSegment {
                            enemy_type: EnemyVariant::Normal,
                            count: 5,
                            spawn_rate: 0.7,
                        },
                        WaveSegment {
                            enemy_type: EnemyVariant::Strong,
                            count: 5,
                            spawn_rate: 0.2,
                        },
                    ],
                },
                Wave {
                    segments: vec![WaveSegment {
                        enemy_type: EnemyVariant::Strong,
                        count: 8,
                        spawn_rate: 0.4,
                    }],
                },
                Wave {
                    segments: vec![
                        WaveSegment {
                            enemy_type: EnemyVariant::Strong,
                            count: 1,
                            spawn_rate: 1.0,
                        },
                        WaveSegment {
                            enemy_type: EnemyVariant::Fast,
                            count: 5,
                            spawn_rate: 1.5,
                        },
                        WaveSegment {
                            enemy_type: EnemyVariant::Strong,
                            count: 1,
                            spawn_rate: 1.0,
                        },
                        WaveSegment {
                            enemy_type: EnemyVariant::Fast,
                            count: 5,
                            spawn_rate: 1.5,
                        },
                    ],
                },
                Wave {
                    segments: vec![WaveSegment {
                        enemy_type: EnemyVariant::Boss,
                        count: 1,
                        spawn_rate: 0.7,
                    }],
                },
                Wave {
                    segments: vec![
                        WaveSegment {
                            enemy_type: EnemyVariant::Normal,
                            count: 5,
                            spawn_rate: 0.7,
                        },
                        WaveSegment {
                            enemy_type: EnemyVariant::Strong,
                            count: 5,
                            spawn_rate: 0.2,
                        },
                    ],
                },
                Wave {
                    segments: vec![
                        WaveSegment {
                            enemy_type: EnemyVariant::Normal,
                            count: 10,
                            spawn_rate: 1.5,
                        },
                        WaveSegment {
                            enemy_type: EnemyVariant::Boss,
                            count: 3,
                            spawn_rate: 0.2,
                        },
                    ],
                },
                Wave {
                    segments: vec![
                        WaveSegment {
                            enemy_type: EnemyVariant::Fast,
                            count: 20,
                            spawn_rate: 2.5,
                        },
                        WaveSegment {
                            enemy_type: EnemyVariant::StrongFast,
                            count: 1,
                            spawn_rate: 0.5,
                        },
                    ],
                },
                Wave {
                    segments: vec![
                        WaveSegment {
                            enemy_type: EnemyVariant::StrongFast,
                            count: 7,
                            spawn_rate: 1.5,
                        },
                        WaveSegment {
                            enemy_type: EnemyVariant::Boss,
                            count: 3,
                            spawn_rate: 0.2,
                        },
                    ],
                },
                Wave {
                    segments: vec![
                        WaveSegment {
                            enemy_type: EnemyVariant::StrongFast,
                            count: 12,
                            spawn_rate: 1.5,
                        },
                        WaveSegment {
                            enemy_type: EnemyVariant::Boss,
                            count: 5,
                            spawn_rate: 0.2,
                        },
                    ],
                },
                Wave {
                    segments: vec![WaveSegment {
                        enemy_type: EnemyVariant::Boss,
                        count: 10,
                        spawn_rate: 0.4,
                    }],
                },
                Wave {
                    segments: vec![WaveSegment {
                        enemy_type: EnemyVariant::StrongFast,
                        count: 15,
                        spawn_rate: 2.5,
                    }],
                },
                Wave {
                    segments: vec![WaveSegment {
                        enemy_type: EnemyVariant::Normal,
                        count: 10,
                        spawn_rate: 1.0,
                    }],
                },
                Wave {
                    segments: vec![WaveSegment {
                        enemy_type: EnemyVariant::UltraBoss,
                        count: 1,
                        spawn_rate: 1.5,
                    }],
                },
                Wave {
                    segments: vec![
                        WaveSegment {
                            enemy_type: EnemyVariant::Boss,
                            count: 5,
                            spawn_rate: 0.5,
                        },
                        WaveSegment {
                            enemy_type: EnemyVariant::StrongFast,
                            count: 10,
                            spawn_rate: 1.2,
                        },
                    ],
                },
                Wave {
                    segments: vec![WaveSegment {
                        enemy_type: EnemyVariant::Strong,
                        count: 20,
                        spawn_rate: 2.5,
                    }],
                },
                Wave {
                    segments: vec![
                        WaveSegment {
                            enemy_type: EnemyVariant::Boss,
                            count: 5,
                            spawn_rate: 0.5,
                        },
                        WaveSegment {
                            enemy_type: EnemyVariant::UltraBoss,
                            count: 2,
                            spawn_rate: 0.2,
                        },
                    ],
                },
                Wave {
                    segments: vec![WaveSegment {
                        enemy_type: EnemyVariant::UltraBoss,
                        count: 5,
                        spawn_rate: 0.3,
                    }],
                },
            ],

            wave_state: WaveState::Waiting,
            spawn_timer: Timer::from_seconds(0.1, TimerMode::Once),
            lives: 15,
            score: 0,
            health_multiplier: 1.0,
            speed_multiplier: 1.0,
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
    drums_channel: Res<AudioChannel<DrumsChannel>>,
) {
    match game_manager.wave_state {
        WaveState::Spawning(num) => {
            game_manager.spawn_timer.tick(time.delta());
            if game_manager.spawn_timer.finished() {
                let mut finished = true;
                let wave_index = if game_manager.current_wave < game_manager.waves.len() {
                    game_manager.current_wave
                } else {
                    // if we've run out of waves, alternate between the last 4
                    (game_manager.current_wave % 4) + game_manager.waves.len() - 4
                };
                let wave = &game_manager.waves[wave_index];
                let mut count = 0;
                for segment in wave.segments.iter() {
                    if count + segment.count > num.into() {
                        let spawn_pos = Map::grid_to_world_pos((
                            map.start_pos.0 as f32,
                            map.start_pos.1 as f32,
                        ));
                        commands
                            .spawn(SpriteBundle {
                                texture: match segment.enemy_type {
                                    EnemyVariant::Weak => game_assets.enemy1.clone(),
                                    EnemyVariant::Normal => game_assets.enemy2.clone(),
                                    EnemyVariant::Fast => game_assets.enemy3.clone(),
                                    EnemyVariant::Strong => game_assets.enemy4.clone(),
                                    EnemyVariant::Boss => game_assets.enemy5.clone(),
                                    EnemyVariant::StrongFast => game_assets.enemy6.clone(),
                                    EnemyVariant::UltraBoss => game_assets.enemy7.clone(),
                                },
                                transform: Transform::from_translation(Vec3::new(
                                    spawn_pos.x,
                                    spawn_pos.y,
                                    1.0,
                                )),
                                ..Default::default()
                            })
                            .insert(Enemy::new(
                                segment.enemy_type,
                                map.start_pos,
                                game_manager.health_multiplier,
                                game_manager.speed_multiplier,
                            ));
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
                drums_channel.set_volume(0.0);
                game_manager.current_wave += 1;
                game_manager.wave_state = WaveState::Waiting;
                let mut options = Vec::new();
                if game_manager.current_wave == 1 {
                    // Fix the first set of tower options
                    options.push(Tower::new(
                        0.5,
                        1.8,
                        TowerType::ChargeShot,
                        Debuff::LaserIncompatible,
                    ));
                    options.push(Tower::new(
                        0.17,
                        4.0,
                        TowerType::Laser,
                        Debuff::ReduceColumnDamage(10.0),
                    ));
                    options.push(Tower::new(
                        3.0,
                        0.3,
                        TowerType::Sniper,
                        Debuff::ReduceNeighbourRate(15.0),
                    ));
                } else {
                    for _ in 0..3 {
                        options.push(Tower::new_random());
                    }
                }
                present_tower_options(commands, game_assets.font.clone(), &options);
                ui_state.state = UiState::PickingTower(options);
            }
        }
        _ => {}
    }
}

pub fn start_next_wave(
    input: Res<ButtonInput<KeyCode>>,
    mut game_manager: ResMut<GameManager>,
    drums_channel: Res<AudioChannel<DrumsChannel>>,
    volume_settings: Res<VolumeSettings>,
) {
    if (input.just_pressed(KeyCode::Space)) || game_manager.current_wave == 0 {
        if let WaveState::Waiting = game_manager.wave_state {
            drums_channel.set_volume(volume_settings.music_vol * 1.5);
            game_manager.wave_state = WaveState::Spawning(0);
            if game_manager.current_wave >= game_manager.waves.len() {
                // If we're in endless mode, gradually increase the difficulty
                game_manager.health_multiplier += 0.1;
                game_manager.speed_multiplier += 0.1;
            }
        }
    }
}

pub fn game_over_check(
    game_manager: Res<GameManager>,
    mut next_state: ResMut<NextState<State>>,
    mut scores: ResMut<Scores>,
) {
    if game_manager.lives == 0 {
        scores.last_score = game_manager.score;
        scores.last_wave = game_manager.current_wave as u32;
        if game_manager.score > scores.high_score {
            scores.high_score = game_manager.score;
        }
        next_state.set(State::Results);
    }
}
