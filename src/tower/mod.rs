use bevy::prelude::*;

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

#[derive(Debug)]
pub enum Debuff {
    // MoveSpeedUp(f32),
    ReduceNeighbourDamage(f32),
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
            }
        }
    }
}
