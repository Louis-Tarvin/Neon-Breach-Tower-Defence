use bevy::prelude::*;
use rand::{distributions::Standard, prelude::Distribution, Rng};

use crate::grid::Map;

use super::Tower;

#[derive(Debug)]
pub enum Debuff {
    MoveSpeedUp(f32),
    ReduceNeighbourDamage(f32),
    ReduceNeighbourRate(f32),
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
            Debuff::ReduceNeighbourRate(percent) => {
                format!("Towers directly next to this one shoot {}% slower", percent)
            }
            Debuff::Immune => {
                "This tower is immune to the effects of neighbouring tiles".to_string()
            }
        }
    }
}
impl Distribution<Debuff> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Debuff {
        match rng.gen_range(0..=3) {
            0 => Debuff::MoveSpeedUp((rng.gen_range(5.0..=25.0) as f32).round()),
            1 => Debuff::ReduceNeighbourDamage((rng.gen_range(5.0..=25.0) as f32).round()),
            2 => Debuff::ReduceNeighbourRate((rng.gen_range(5.0..=25.0) as f32).round()),
            3 => Debuff::Immune,
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
                _ => (),
            }
        }
    }
}
