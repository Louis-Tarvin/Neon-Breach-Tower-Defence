use bevy::prelude::*;

use crate::{
    enemies, gameplay, grid, input,
    tower::{self, debuffs::SpeedUpPoint},
    ui::{self, inventory, sidebar, statusbar, tower_options},
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<tower::TowerPlaced>()
            .add_event::<tower::debuffs::AddDebuff>()
            .add_system(setup.in_schedule(OnEnter(super::State::Game)))
            .add_system(grid::load_map.in_schedule(OnEnter(super::State::Game)))
            .add_system(statusbar::draw_status_bar.in_schedule(OnEnter(super::State::Game)))
            .add_system(gameplay::gameloop.in_set(OnUpdate(super::State::Game)))
            .add_system(gameplay::start_next_wave.in_set(OnUpdate(super::State::Game)))
            .add_system(gameplay::game_over_check.in_set(OnUpdate(super::State::Game)))
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
            .add_system(tower::debuffs::handle_overheat.in_set(OnUpdate(super::State::Game)))
            .add_system(tower::charge_shot::shoot.in_set(OnUpdate(super::State::Game)))
            .add_system(tower::sniper::shoot.in_set(OnUpdate(super::State::Game)))
            .add_system(tower::handle_projectiles.in_set(OnUpdate(super::State::Game)))
            .add_system(tower::laser::shoot.in_set(OnUpdate(super::State::Game)))
            .add_system(tower::jammer::rotate_dish.in_set(OnUpdate(super::State::Game)))
            .add_system(tower::missile::spawn_missile.in_set(OnUpdate(super::State::Game)))
            .add_system(tower::missile::handle_missile.in_set(OnUpdate(super::State::Game)))
            .add_system(inventory::give_random_tower.in_set(OnUpdate(super::State::Game)))
            .add_system(ui::update_selection_indicator.in_set(OnUpdate(super::State::Game)))
            .add_system(inventory::draw_inventory.in_set(OnUpdate(super::State::Game)))
            .add_system(inventory::handle_inventory_buttons.in_set(OnUpdate(super::State::Game)))
            .add_system(tower_options::handle_tower_options.in_set(OnUpdate(super::State::Game)))
            .add_system(sidebar::draw_sidebar.in_set(OnUpdate(super::State::Game)))
            .add_system(sidebar::handle_toggle_rotation_button.in_set(OnUpdate(super::State::Game)))
            .add_system(statusbar::update_status_bar_text.in_set(OnUpdate(super::State::Game)))
            .add_system(statusbar::update_score_text.in_set(OnUpdate(super::State::Game)))
            .add_system(statusbar::update_lives_text.in_set(OnUpdate(super::State::Game)))
            .add_system(statusbar::handle_normal_speed_button.in_set(OnUpdate(super::State::Game)))
            .add_system(statusbar::handle_fast_speed_button.in_set(OnUpdate(super::State::Game)))
            .add_system(input::grid_click_handler.in_set(OnUpdate(super::State::Game)))
            .add_system(input::mouse_hover_handler.in_set(OnUpdate(super::State::Game)))
            .add_system(cleanup.in_schedule(OnExit(super::State::Game)));
    }
}

fn setup(
    mut commands: Commands,
    mut cameras: Query<(&mut OrthographicProjection, &mut Transform)>,
) {
    // Insert resources
    commands.insert_resource(gameplay::GameManager::new());
    commands.insert_resource(ui::UiData::default());
    commands.insert_resource(ui::UiStateResource::default());
    commands.insert_resource(input::HoverPosition::default());
    commands.insert_resource(inventory::Inventory::default());
    commands.insert_resource(ui::statusbar::GameSpeed(false));
    // Position the camera
    for (mut projection, mut transform) in cameras.iter_mut() {
        projection.scale = 0.4;
        transform.translation.x = 76.0;
        transform.translation.y = 76.0;
    }
}

fn cleanup(
    mut commands: Commands,
    mut sprites: Query<Entity, With<Sprite>>,
    mut nodes: Query<Entity, With<Node>>,
    mut speed_up_points: Query<Entity, With<SpeedUpPoint>>,
) {
    for entity in sprites
        .iter_mut()
        .chain(nodes.iter_mut())
        .chain(speed_up_points.iter_mut())
    {
        commands.entity(entity).despawn_recursive();
    }
}
