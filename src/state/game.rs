use bevy::prelude::*;

use crate::{enemies, grid, input, tower};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(super::State::Game)))
            .add_system(grid::load_map.in_schedule(OnEnter(super::State::Game)))
            .add_system(enemies::spawn_enemies.in_set(OnUpdate(super::State::Game)))
            .add_system(enemies::update_enemy_grid_pos.in_set(OnUpdate(super::State::Game)))
            .add_system(enemies::enemy_movement.in_set(OnUpdate(super::State::Game)))
            .add_system(enemies::check_killed.in_set(OnUpdate(super::State::Game)))
            .add_system(tower::charge_shot::shoot.in_set(OnUpdate(super::State::Game)))
            .add_system(tower::charge_shot::handle_projectiles.in_set(OnUpdate(super::State::Game)))
            .add_system(input::grid_click_handler.in_set(OnUpdate(super::State::Game)));
    }
}

fn setup(mut cameras: Query<(&mut OrthographicProjection, &mut Transform)>) {
    // Position the camera
    for (mut projection, mut transform) in cameras.iter_mut() {
        projection.scale = 0.5;
        transform.translation.x += 96.0;
        transform.translation.y += 96.0;
    }
}
