use bevy::prelude::*;
use rand::{distributions::Standard, prelude::Distribution, Rng};

use crate::grid::Map;

use self::debuffs::{AddDebuff, Debuff};

pub mod charge_shot;
pub mod debuffs;
pub mod laser;

#[derive(Debug)]
pub enum TowerType {
    ChargeShot,
    Laser,
    // Sniper,
    // Rocket,
}
impl TowerType {
    pub fn name(&self) -> &'static str {
        match self {
            TowerType::ChargeShot => "Turret",
            TowerType::Laser => "Laser",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            TowerType::ChargeShot => {
                "A tower that regularly shoots a projectile at the first enemy in range"
            }
            TowerType::Laser => "A tower that shoots a continuous beam. Pierces enemies",
        }
    }
}
impl Distribution<TowerType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TowerType {
        match rng.gen_range(0..=1) {
            0 => TowerType::ChargeShot,
            1 => TowerType::Laser,
            // 2 => TowerType::Sniper,
            // 3 => TowerType::Rocket,
            _ => unreachable!(),
        }
    }
}

#[derive(Component, Debug)]
pub struct Tower {
    pub damage: f32,
    pub rate: f32,
    pub variant: TowerType,
    pub debuff: Debuff,
    pub timer: Timer,
}
impl Tower {
    pub fn new(damage: f32, rate: f32, variant: TowerType, debuff: Debuff) -> Self {
        Self {
            damage,
            rate,
            variant,
            debuff,
            timer: Timer::from_seconds(1.0 / rate, TimerMode::Repeating),
        }
    }

    pub fn new_random() -> Self {
        let variant: TowerType = rand::random();
        match variant {
            TowerType::ChargeShot => Self::new(1.0, 1.0, variant, rand::random()),
            TowerType::Laser => Self::new(0.2, 4.0, variant, rand::random()),
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

#[derive(Debug)]
pub struct TowerPlaced {
    pub grid_pos: (i8, i8),
}

pub fn handle_tower_placement(
    mut events: EventReader<TowerPlaced>,
    mut debuff_events: EventWriter<AddDebuff>,
    query: Query<&Tower>,
    map: Res<Map>,
) {
    for event in events.iter() {
        let tower = query
            .get(
                *map.placements
                    .get(&event.grid_pos)
                    .expect("TowerPlaced event with invalid position"),
            )
            .expect("Tower entity not found");

        let (x, y) = event.grid_pos;
        // Apply debuff to self
        if let Debuff::MoveSpeedUp(percent) = &tower.debuff {
            debuff_events.send(AddDebuff {
                grid_pos: (x, y),
                debuff: Debuff::MoveSpeedUp(*percent),
            });
        }
        // Apply debuff to neighbours
        match &tower.debuff {
            Debuff::ReduceNeighbourDamage(percent) => {
                for &(dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    debuff_events.send(AddDebuff {
                        grid_pos: (x + dx, y + dy),
                        debuff: Debuff::ReduceNeighbourDamage(*percent),
                    });
                }
            }
            Debuff::ReduceNeighbourRate(percent) => {
                for &(dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    debuff_events.send(AddDebuff {
                        grid_pos: (x + dx, y + dy),
                        debuff: Debuff::ReduceNeighbourRate(*percent),
                    });
                }
            }
            _ => {}
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
    }
}
