use bevy::prelude::*;

pub mod charge_shot;

#[derive(Debug)]
pub enum TowerType {
    ChargeShot,
    // Laser,
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
