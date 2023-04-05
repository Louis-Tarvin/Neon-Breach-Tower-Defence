use bevy::prelude::*;

use crate::{
    enemies, gameplay, grid, input, tower,
    ui::{self, inventory, sidebar, tower_options},
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<tower::TowerPlaced>()
            .add_event::<tower::debuffs::AddDebuff>()
            .insert_resource(gameplay::GameManager::new())
            .insert_resource(ui::UiData::default())
            .insert_resource(ui::UiStateResource::default())
            .insert_resource(input::HoverPosition::default())
            .insert_resource(inventory::Inventory::default())
            .add_system(setup.in_schedule(OnEnter(super::State::Game)))
            .add_system(grid::load_map.in_schedule(OnEnter(super::State::Game)))
            .add_system(gameplay::gameloop.in_set(OnUpdate(super::State::Game)))
            .add_system(enemies::spawn_enemies.in_set(OnUpdate(super::State::Game)))
            .add_systems(
                (
                    enemies::enemy_movement.in_set(OnUpdate(super::State::Game)),
                    enemies::update_enemy_grid_pos.in_set(OnUpdate(super::State::Game)),
                )
                    .chain(),
            )
            .add_systems(
                (
                    enemies::check_killed.in_set(OnUpdate(super::State::Game)),
                    enemies::update_healthbar.in_set(OnUpdate(super::State::Game)),
                    enemies::scale_healthbar.in_set(OnUpdate(super::State::Game)),
                )
                    .chain(),
            )
            .add_system(tower::handle_tower_placement.in_set(OnUpdate(super::State::Game)))
            .add_system(tower::debuffs::debuff_event_handler.in_set(OnUpdate(super::State::Game)))
            .add_system(tower::charge_shot::shoot.in_set(OnUpdate(super::State::Game)))
            .add_system(tower::charge_shot::handle_projectiles.in_set(OnUpdate(super::State::Game)))
            .add_system(tower::laser::shoot.in_set(OnUpdate(super::State::Game)))
            .add_system(inventory::give_random_tower.in_set(OnUpdate(super::State::Game)))
            .add_system(ui::update_selection_indicator.in_set(OnUpdate(super::State::Game)))
            .add_system(inventory::draw_inventory.in_set(OnUpdate(super::State::Game)))
            .add_system(inventory::handle_inventory_buttons.in_set(OnUpdate(super::State::Game)))
            .add_system(tower_options::handle_tower_options.in_set(OnUpdate(super::State::Game)))
            .add_system(sidebar::draw_sidebar.in_set(OnUpdate(super::State::Game)))
            .add_system(sidebar::handle_toggle_rotation_button.in_set(OnUpdate(super::State::Game)))
            .add_system(input::grid_click_handler.in_set(OnUpdate(super::State::Game)))
            .add_system(input::mouse_hover_handler.in_set(OnUpdate(super::State::Game)));
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
