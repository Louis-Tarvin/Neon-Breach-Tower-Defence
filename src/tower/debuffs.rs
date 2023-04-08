use std::time::Duration;

use bevy::prelude::*;
use rand::{distributions::Standard, prelude::Distribution, Rng};

use crate::{
    gameplay::{GameManager, WaveState},
    grid::Map,
    state::loading::GameAssets,
};

use super::{TargetMode, Tower, TowerType};

#[derive(Debug)]
pub enum Debuff {
    MoveSpeedUp(f32),
    ReduceNeighbourDamage(f32),
    ReduceRowDamage(f32),
    ReduceColumnDamage(f32),
    ReduceNeighbourRate(f32),
    ReduceRowRate(f32),
    ReduceColumnRate(f32),
    Overheat,
    RowOverheat,
    ColumnOverheat,
    TurretIncompatible,
    SniperIncompatible,
    LaserIncompatible,
    MissileIncompatible,
    TargetClosest,
    TargetRandom,
    Immune,
}
impl Debuff {
    pub fn description(&self) -> String {
        match self {
            Debuff::MoveSpeedUp(percent) => {
                format!("Enemies within 1 tile move {}% faster", percent)
            }
            Debuff::ReduceNeighbourDamage(percent) => {
                format!(
                    "Towers directly next to this one do {}% less damage",
                    percent
                )
            }
            Debuff::ReduceRowDamage(percent) => {
                format!("Towers in the same row do {}% less damage", percent)
            }
            Debuff::ReduceColumnDamage(percent) => {
                format!("Towers in the same column do {}% less damage", percent)
            }
            Debuff::ReduceNeighbourRate(percent) => {
                format!("Towers directly next to this one shoot {}% slower", percent)
            }
            Debuff::ReduceRowRate(percent) => {
                format!("Towers in the same row shoot {}% slower", percent)
            }
            Debuff::ReduceColumnRate(percent) => {
                format!("Towers in the same column shoot {}% slower", percent)
            }
            Debuff::Overheat => {
                "Has a chance to randomly overheat, rendering it ineffective for a few seconds"
                    .to_string()
            }
            Debuff::RowOverheat => {
                "All towers in the same row have a chance to randomly overheat, rendering them ineffective for a few seconds"
                    .to_string()
            }
            Debuff::ColumnOverheat => {
                "All towers in the same column have a chance to randomly overheat, rendering them ineffective for a few seconds"
                    .to_string()
            }
            Debuff::TurretIncompatible => {
                "Towers of type 'Turret' directly next to this one do half damage".to_string()
            }
            Debuff::SniperIncompatible => {
                "Towers of type 'Sniper' directly next to this one do half damage".to_string()
            }
            Debuff::LaserIncompatible => {
                "Towers of type 'Laser' directly next to this one do half damage".to_string()
            }
            Debuff::MissileIncompatible => {
                "Towers of type 'Missile Launcher' directly next to this one do half damage"
                    .to_string()
            }
            Debuff::TargetClosest => {
                "This tower will target the closest enemy in range instead of the one at the front"
                    .to_string()
            }
            Debuff::TargetRandom => "This tower will fire randomly at enemies in range".to_string(),
            Debuff::Immune => {
                "This tower is immune to the effects of neighbouring tiles".to_string()
            }
        }
    }
}
impl Distribution<Debuff> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Debuff {
        match rng.gen_range(0..=29) {
            0..=1 => Debuff::MoveSpeedUp((rng.gen_range(15.0..=45.0) as f32).round()),
            2..=3 => Debuff::ReduceNeighbourDamage((rng.gen_range(15.0..=40.0) as f32).round()),
            4..=5 => Debuff::ReduceNeighbourRate((rng.gen_range(10.0..=40.0) as f32).round()),
            6..=7 => Debuff::Overheat,
            8..=9 => Debuff::TurretIncompatible,
            10 => Debuff::SniperIncompatible,
            11 => Debuff::LaserIncompatible,
            12 => Debuff::MissileIncompatible,
            13..=14 => Debuff::TargetClosest,
            15..=16 => Debuff::TargetRandom,
            17..=18 => Debuff::RowOverheat,
            19..=20 => Debuff::ColumnOverheat,
            21..=22 => Debuff::ReduceRowDamage((rng.gen_range(10.0..=35.0) as f32).round()),
            23..=24 => Debuff::ReduceColumnDamage((rng.gen_range(10.0..=35.0) as f32).round()),
            25..=26 => Debuff::ReduceRowRate((rng.gen_range(10.0..=35.0) as f32).round()),
            27..=28 => Debuff::ReduceColumnRate((rng.gen_range(10.0..=35.0) as f32).round()),
            29 => Debuff::Immune,
            _ => unreachable!(),
        }
    }
}

#[derive(Component)]
pub struct SpeedUpPoint(pub f32);

#[derive(Debug)]
pub struct AddDebuff {
    pub grid_pos: (i8, i8),
    pub debuff: Debuff,
}

pub fn debuff_event_handler(
    mut commands: Commands,
    mut events: EventReader<AddDebuff>,
    mut query: Query<&mut Tower>,
    map: Res<Map>,
) {
    for event in events.iter() {
        if let Some(entity) = map.placements.get(&event.grid_pos) {
            let mut tower = query.get_mut(*entity).expect("Tower entity not found");
            if let Debuff::Immune = tower.debuff {
                continue;
            }
            match &event.debuff {
                Debuff::ReduceNeighbourDamage(percent) => tower.reduce_damage_by(*percent),
                Debuff::ReduceNeighbourRate(percent) => tower.reduce_rate_by(*percent),
                Debuff::MoveSpeedUp(percent) => {
                    commands
                        .spawn(SpeedUpPoint(*percent))
                        .insert(Transform::from_xyz(
                            event.grid_pos.0 as f32 * 32.0,
                            event.grid_pos.1 as f32 * 32.0,
                            0.0,
                        ));
                }
                Debuff::Overheat => {
                    commands
                        .entity(*entity)
                        .insert(Overheatable(Timer::from_seconds(20.0, TimerMode::Once)));
                }
                Debuff::TurretIncompatible => {
                    if let TowerType::ChargeShot = tower.variant {
                        tower.reduce_damage_by(50.0);
                    }
                }
                Debuff::SniperIncompatible => {
                    if let TowerType::Sniper = tower.variant {
                        tower.reduce_damage_by(50.0);
                    }
                }
                Debuff::LaserIncompatible => {
                    if let TowerType::Laser = tower.variant {
                        tower.reduce_damage_by(50.0);
                    }
                }
                Debuff::MissileIncompatible => {
                    if let TowerType::Missile = tower.variant {
                        tower.reduce_damage_by(50.0);
                    }
                }
                Debuff::TargetClosest => tower.target_mode = TargetMode::Closest,
                Debuff::TargetRandom => tower.target_mode = TargetMode::Random,
                _ => (),
            }
        }
    }
}

#[derive(Component)]
pub struct Overheatable(pub Timer);

#[derive(Component)]
pub struct OverheatIcon;

pub fn handle_overheat(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Tower, &mut Overheatable, &Children), Without<OverheatIcon>>,
    icons: Query<Entity, With<OverheatIcon>>,
    game_assets: Res<GameAssets>,
    game_manager: Res<GameManager>,
    time: Res<Time>,
) {
    if let WaveState::Waiting = game_manager.wave_state {
        return;
    }
    for (entity, mut tower, mut overheatable, children) in query.iter_mut() {
        overheatable.0.tick(time.delta());
        if overheatable.0.just_finished() {
            if tower.overheating {
                tower.overheating = false;
                overheatable
                    .0
                    .set_duration(Duration::from_secs_f32(rand::random::<f32>() * 60.0 + 10.0));
                overheatable.0.reset();
                for child in children.iter() {
                    if icons.get(*child).is_ok() {
                        commands.entity(*child).despawn_recursive();
                    }
                }
            } else {
                tower.overheating = true;
                overheatable.0.set_duration(Duration::from_secs_f32(8.0));
                overheatable.0.reset();
                commands.entity(entity).with_children(|parent| {
                    parent
                        .spawn(SpriteBundle {
                            texture: game_assets.overheat.clone(),
                            transform: Transform::from_xyz(0.0, 0.0, 4.0),
                            ..Default::default()
                        })
                        .insert(OverheatIcon);
                });
            }
        }
    }
}
