use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};
use rand::{distributions::Standard, prelude::Distribution, Rng};

use crate::{
    audio::{AudioAssets, SoundChannel},
    enemies::Enemy,
    grid::Map,
    ui::UiData,
};

use self::debuffs::{AddDebuff, Debuff};

pub mod charge_shot;
pub mod debuffs;
pub mod jammer;
pub mod laser;
pub mod missile;
pub mod sniper;

#[derive(Debug)]
pub enum TowerType {
    ChargeShot,
    Laser,
    Sniper,
    Jammer,
    Missile,
}
impl TowerType {
    pub fn name(&self) -> &'static str {
        match self {
            TowerType::ChargeShot => "Turret",
            TowerType::Laser => "Laser",
            TowerType::Sniper => "Sniper",
            TowerType::Jammer => "Signal Jammer",
            TowerType::Missile => "Missile Launcher",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            TowerType::ChargeShot => {
                "A tower that regularly shoots a projectile at the first enemy in range"
            }
            TowerType::Laser => "A tower that shoots a continuous beam. Pierces enemies",
            TowerType::Sniper => "A long range tower with high damage but slow rate of fire",
            TowerType::Jammer => "Slows down enemies within 1 tile",
            TowerType::Missile => {
                "Launches a high damage missile at the strongest enemy. Infinite range"
            }
        }
    }
}
impl Distribution<TowerType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TowerType {
        match rng.gen_range(0..=8) {
            0..=2 => TowerType::ChargeShot,
            3..=4 => TowerType::Laser,
            5..=6 => TowerType::Sniper,
            7 => TowerType::Jammer,
            8 => TowerType::Missile,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum TargetMode {
    First,
    Closest,
    Random,
}

#[derive(Component, Debug)]
pub struct Tower {
    pub damage: f32,
    pub rate: f32,
    pub variant: TowerType,
    pub debuff: Debuff,
    pub overheating: bool,
    pub target_mode: TargetMode,
}
impl Tower {
    pub fn new(damage: f32, rate: f32, variant: TowerType, debuff: Debuff) -> Self {
        Self {
            damage,
            rate,
            variant,
            debuff,
            overheating: false,
            target_mode: TargetMode::First,
        }
    }

    pub fn new_random() -> Self {
        let mut rng = rand::thread_rng();
        let variant: TowerType = rng.gen();
        match variant {
            TowerType::ChargeShot => Self::new(
                (rng.gen_range(0.3..=0.5) * 100.0_f32).round() / 100.0,
                1.8,
                variant,
                rand::random(),
            ),
            TowerType::Laser => loop {
                let debuff: Debuff = rand::random();
                match debuff {
                    Debuff::TargetClosest | Debuff::TargetRandom => {
                        // These debuffs are not compatible with laser
                        continue;
                    }
                    _ => break Self::new(0.16, 4.0, variant, debuff),
                }
            },
            TowerType::Sniper => Self::new(3.0, 0.3, variant, rand::random()),
            TowerType::Jammer => loop {
                let debuff: Debuff = rand::random();
                match debuff {
                    Debuff::TargetClosest
                    | Debuff::TargetRandom
                    | Debuff::MoveSpeedUp(_)
                    | Debuff::Overheat => {
                        // These debuffs are not compatible with jammer
                        continue;
                    }
                    _ => break Self::new(0.0, 0.0, variant, debuff),
                }
            },
            TowerType::Missile => loop {
                let debuff: Debuff = rand::random();
                match debuff {
                    Debuff::TargetClosest | Debuff::TargetRandom => {
                        // These debuffs are not compatible with missile launcher
                        continue;
                    }
                    _ => break Self::new(8.0, 0.135, variant, debuff),
                }
            },
        }
    }

    pub fn reduce_damage_by(&mut self, percent: f32) {
        let reduction = self.damage * (percent / 100.0);
        self.damage -= reduction;
    }

    pub fn reduce_rate_by(&mut self, percent: f32) {
        let reduction = self.rate * (percent / 100.0);
        self.rate -= reduction;
    }
}

#[derive(Debug, Event)]
pub struct TowerPlaced {
    pub grid_pos: (i8, i8),
}

pub fn handle_tower_placement(
    mut events: EventReader<TowerPlaced>,
    mut debuff_events: EventWriter<AddDebuff>,
    mut ui_data: ResMut<UiData>,
    query: Query<&Tower>,
    map: Res<Map>,
    sound_channel: Res<AudioChannel<SoundChannel>>,
    audio_assets: Res<AudioAssets>,
) {
    for event in events.iter() {
        sound_channel.play(audio_assets.place.clone());
        let tower = query
            .get(
                *map.placements
                    .get(&event.grid_pos)
                    .expect("TowerPlaced event with invalid position"),
            )
            .expect("Tower entity not found");

        ui_data.selected_pos = Some(event.grid_pos);
        let (x, y) = event.grid_pos;
        // Apply debuff to self
        match &tower.debuff {
            Debuff::MoveSpeedUp(percent) => {
                debuff_events.send(AddDebuff {
                    grid_pos: (x, y),
                    debuff: Debuff::MoveSpeedUp(*percent),
                });
            }
            Debuff::Overheat => {
                debuff_events.send(AddDebuff {
                    grid_pos: (x, y),
                    debuff: Debuff::Overheat,
                });
            }
            Debuff::TargetClosest => {
                debuff_events.send(AddDebuff {
                    grid_pos: (x, y),
                    debuff: Debuff::TargetClosest,
                });
            }
            Debuff::TargetRandom => {
                debuff_events.send(AddDebuff {
                    grid_pos: (x, y),
                    debuff: Debuff::TargetRandom,
                });
            }
            _ => (),
        }
        // Apply debuff to neighbours
        for &(dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            match &tower.debuff {
                Debuff::ReduceNeighbourDamage(percent) => {
                    debuff_events.send(AddDebuff {
                        grid_pos: (x + dx, y + dy),
                        debuff: Debuff::ReduceNeighbourDamage(*percent),
                    });
                }
                Debuff::ReduceNeighbourRate(percent) => {
                    debuff_events.send(AddDebuff {
                        grid_pos: (x + dx, y + dy),
                        debuff: Debuff::ReduceNeighbourRate(*percent),
                    });
                }
                Debuff::TurretIncompatible => {
                    debuff_events.send(AddDebuff {
                        grid_pos: (x + dx, y + dy),
                        debuff: Debuff::TurretIncompatible,
                    });
                }
                Debuff::SniperIncompatible => {
                    debuff_events.send(AddDebuff {
                        grid_pos: (x + dx, y + dy),
                        debuff: Debuff::SniperIncompatible,
                    });
                }
                Debuff::LaserIncompatible => {
                    debuff_events.send(AddDebuff {
                        grid_pos: (x + dx, y + dy),
                        debuff: Debuff::LaserIncompatible,
                    });
                }
                Debuff::MissileIncompatible => {
                    debuff_events.send(AddDebuff {
                        grid_pos: (x + dx, y + dy),
                        debuff: Debuff::MissileIncompatible,
                    });
                }
                _ => {}
            }
        }
        // Apply debuff to row
        for x2 in 0..=map.width {
            if x2 == x as u8 {
                // Skip self
                continue;
            }
            match &tower.debuff {
                Debuff::RowOverheat => {
                    debuff_events.send(AddDebuff {
                        grid_pos: (x2.try_into().unwrap(), y),
                        debuff: Debuff::Overheat,
                    });
                }
                Debuff::ReduceRowDamage(percent) => {
                    debuff_events.send(AddDebuff {
                        grid_pos: (x2.try_into().unwrap(), y),
                        debuff: Debuff::ReduceNeighbourDamage(*percent),
                    });
                }
                Debuff::ReduceRowRate(percent) => {
                    debuff_events.send(AddDebuff {
                        grid_pos: (x2.try_into().unwrap(), y),
                        debuff: Debuff::ReduceNeighbourRate(*percent),
                    });
                }
                _ => (),
            }
        }
        // Apply debuff to column
        for y2 in 0..=map.height {
            if y2 == y as u8 {
                // Skip self
                continue;
            }
            match &tower.debuff {
                Debuff::ColumnOverheat => {
                    debuff_events.send(AddDebuff {
                        grid_pos: (x, y2.try_into().unwrap()),
                        debuff: Debuff::Overheat,
                    });
                }
                Debuff::ReduceColumnDamage(percent) => {
                    debuff_events.send(AddDebuff {
                        grid_pos: (x, y2.try_into().unwrap()),
                        debuff: Debuff::ReduceNeighbourDamage(*percent),
                    });
                }
                Debuff::ReduceColumnRate(percent) => {
                    debuff_events.send(AddDebuff {
                        grid_pos: (x, y2.try_into().unwrap()),
                        debuff: Debuff::ReduceNeighbourRate(*percent),
                    });
                }
                _ => (),
            }
        }
        // Apply neighbour debuff to self
        for &(dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if let Some(entity) = map.placements.get(&(x + dx, y + dy)) {
                let neighbour_tower = query.get(*entity).expect("Tower entity not found");
                match &neighbour_tower.debuff {
                    Debuff::ReduceNeighbourDamage(percent) => {
                        debuff_events.send(AddDebuff {
                            grid_pos: (x, y),
                            debuff: Debuff::ReduceNeighbourDamage(*percent),
                        });
                    }
                    Debuff::ReduceNeighbourRate(percent) => {
                        debuff_events.send(AddDebuff {
                            grid_pos: (x, y),
                            debuff: Debuff::ReduceNeighbourRate(*percent),
                        });
                    }
                    _ => {}
                }
            }
        }
        // Apply row debuffs to self
        for x2 in 0..=map.width {
            if x2 == x as u8 {
                // Skip self
                continue;
            }
            if let Some(entity) = map.placements.get(&(x2.try_into().unwrap(), y)) {
                let neighbour_tower = query.get(*entity).expect("Tower entity not found");
                match &neighbour_tower.debuff {
                    Debuff::RowOverheat => {
                        debuff_events.send(AddDebuff {
                            grid_pos: (x, y),
                            debuff: Debuff::Overheat,
                        });
                    }
                    Debuff::ReduceRowDamage(percent) => {
                        debuff_events.send(AddDebuff {
                            grid_pos: (x, y),
                            debuff: Debuff::ReduceNeighbourDamage(*percent),
                        });
                    }
                    Debuff::ReduceRowRate(percent) => {
                        debuff_events.send(AddDebuff {
                            grid_pos: (x, y),
                            debuff: Debuff::ReduceNeighbourRate(*percent),
                        });
                    }
                    _ => (),
                }
            }
        }
        // Apply column debuffs to self
        for y2 in 0..=map.height {
            if y2 == y as u8 {
                // Skip self
                continue;
            }
            if let Some(entity) = map.placements.get(&(x, y2.try_into().unwrap())) {
                let neighbour_tower = query.get(*entity).expect("Tower entity not found");
                match &neighbour_tower.debuff {
                    Debuff::ColumnOverheat => {
                        debuff_events.send(AddDebuff {
                            grid_pos: (x, y),
                            debuff: Debuff::Overheat,
                        });
                    }
                    Debuff::ReduceColumnDamage(percent) => {
                        debuff_events.send(AddDebuff {
                            grid_pos: (x, y),
                            debuff: Debuff::ReduceNeighbourDamage(*percent),
                        });
                    }
                    Debuff::ReduceColumnRate(percent) => {
                        debuff_events.send(AddDebuff {
                            grid_pos: (x, y),
                            debuff: Debuff::ReduceNeighbourRate(*percent),
                        });
                    }
                    _ => (),
                }
            }
        }
    }
}

#[derive(Component, Debug)]
pub struct Projectile {
    pub damage: f32,
    pub speed: f32,
    pub target: Entity,
}

#[derive(Component)]
pub struct RangeIndicator;

#[derive(Component)]
pub struct RotatingTurret;

pub fn handle_projectiles(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Projectile, &mut Transform), Without<Enemy>>,
    mut enemies: Query<(&mut Enemy, &Transform), Without<Projectile>>,
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
