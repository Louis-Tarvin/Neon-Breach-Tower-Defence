use bevy::prelude::*;
use rand::{distributions::Standard, prelude::Distribution, Rng};

use crate::grid::Map;

pub mod charge_shot;
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
            TowerType::ChargeShot => "Charge Shot",
            TowerType::Laser => "Laser",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            TowerType::ChargeShot => {
                "A tower that regularly shoots a projectile at the furthest enemy in range"
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

#[derive(Debug)]
pub enum Debuff {
    MoveSpeedUp(f32),
    ReduceNeighbourDamage(f32),
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
        }
    }
}
impl Distribution<Debuff> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Debuff {
        match rng.gen_range(0..=1) {
            0 => Debuff::MoveSpeedUp((rng.gen_range(5.0..=25.0) as f32).round()),
            1 => Debuff::ReduceNeighbourDamage((rng.gen_range(5.0..=25.0) as f32).round()),
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
}

#[derive(Debug)]
pub struct TowerPlaced {
    pub grid_pos: (i8, i8),
}

#[derive(Debug)]
pub struct AddDebuff {
    pub grid_pos: (i8, i8),
    pub debuff: Debuff,
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
                    _ => {}
                }
            }
        }
    }
}

pub fn debuff_event_handler(
    mut events: EventReader<AddDebuff>,
    mut query: Query<&mut Tower>,
    map: Res<Map>,
) {
    for event in events.iter() {
        if let Some(entity) = map.placements.get(&event.grid_pos) {
            let mut tower = query.get_mut(*entity).expect("Tower entity not found");
            match &event.debuff {
                Debuff::ReduceNeighbourDamage(percent) => tower.reduce_damage_by(*percent),
                _ => {}
            }
        }
    }
}
