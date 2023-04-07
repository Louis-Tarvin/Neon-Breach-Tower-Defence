use bevy::prelude::*;
use rand::{distributions::Standard, prelude::Distribution, Rng};

use crate::{enemies::Enemy, grid::Map, ui::UiData};

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
        match rng.gen_range(0..=4) {
            0 => TowerType::ChargeShot,
            1 => TowerType::Laser,
            2 => TowerType::Sniper,
            3 => TowerType::Jammer,
            4 => TowerType::Missile,
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
}
impl Tower {
    pub fn new(damage: f32, rate: f32, variant: TowerType, debuff: Debuff) -> Self {
        Self {
            damage,
            rate,
            variant,
            debuff,
        }
    }

    pub fn new_random() -> Self {
        let mut rng = rand::thread_rng();
        let variant: TowerType = rng.gen();
        match variant {
            TowerType::ChargeShot => Self::new(
                (rng.gen_range(0.7..=1.3) * 100.0_f32).round() / 100.0,
                1.0,
                variant,
                rand::random(),
            ),
            TowerType::Laser => Self::new(0.2, 4.0, variant, rand::random()),
            TowerType::Sniper => Self::new(4.0, 0.3, variant, rand::random()),
            TowerType::Jammer => Self::new(0.0, 0.0, variant, rand::random()),
            TowerType::Missile => Self::new(10.0, 0.15, variant, rand::random()),
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
    mut ui_data: ResMut<UiData>,
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

        ui_data.selected_pos = Some(event.grid_pos);
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
