use bevy::prelude::*;

use crate::tower::Tower;

#[derive(Resource, Default, Debug)]
pub struct Inventory {
    pub towers: Vec<Tower>,
}

pub fn give_random_tower(mut inventory: ResMut<Inventory>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Return) {
        inventory.towers.push(Tower::new_random());
    }
}
