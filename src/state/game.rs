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
            .add_systems(Update, gameplay::gameloop.run_if(in_state(super::State::Game))) // gameplay::gameloop.in_set(OnUpdate(super::State::Game)))
            .add_systems(Update, gameplay::start_next_wave.run_if(in_state(super::State::Game)))
            .add_systems(Update, gameplay::game_over_check.run_if(in_state(super::State::Game)))
            .add_systems(Update,
                         (
                             enemies::enemy_movement.run_if(in_state(super::State::Game)),
                             enemies::update_enemy_grid_pos.run_if(in_state(super::State::Game)),
                         )
                             .chain(),
            )
            .add_systems(Update,
                         (
                             enemies::check_killed.run_if(in_state(super::State::Game)),
                             enemies::update_healthbar.run_if(in_state(super::State::Game)),
                             enemies::scale_healthbar.run_if(in_state(super::State::Game)),
                         )
                             .chain(),
            )
            .add_systems(Update, tower::handle_tower_placement.run_if(in_state(super::State::Game)))
            .add_systems(Update, tower::debuffs::debuff_event_handler.run_if(in_state(super::State::Game)))
            .add_systems(Update, tower::debuffs::handle_overheat.run_if(in_state(super::State::Game)))
            .add_systems(Update, tower::charge_shot::shoot.run_if(in_state(super::State::Game)))
            .add_systems(Update, tower::sniper::shoot.run_if(in_state(super::State::Game)))
            .add_systems(Update, tower::handle_projectiles.run_if(in_state(super::State::Game)))
            .add_systems(Update, tower::laser::shoot.run_if(in_state(super::State::Game)))
            .add_systems(Update, tower::jammer::rotate_dish.run_if(in_state(super::State::Game)))
            .add_systems(Update, tower::missile::spawn_missile.run_if(in_state(super::State::Game)))
            .add_systems(Update, tower::missile::handle_missile.run_if(in_state(super::State::Game)))
            .add_systems(Update, ui::update_selection_indicator.run_if(in_state(super::State::Game)))
            .add_systems(Update, inventory::draw_inventory.run_if(in_state(super::State::Game)))
            .add_systems(Update, inventory::handle_inventory_buttons.run_if(in_state(super::State::Game)))
            .add_systems(Update, inventory::create_ghost.run_if(in_state(super::State::Game)))
            .add_systems(Update, inventory::handle_ghost.run_if(in_state(super::State::Game)))
            .add_systems(Update, tower_options::handle_tower_options.run_if(in_state(super::State::Game)))
            .add_systems(Update, sidebar::draw_sidebar.run_if(in_state(super::State::Game)))
            .add_systems(Update, sidebar::handle_toggle_rotation_button.run_if(in_state(super::State::Game)))
            .add_systems(Update, statusbar::update_status_bar_text.run_if(in_state(super::State::Game)))
            .add_systems(Update, statusbar::update_score_text.run_if(in_state(super::State::Game)))
            .add_systems(Update, statusbar::update_lives_text.run_if(in_state(super::State::Game)))
            .add_systems(Update, statusbar::handle_normal_speed_button.run_if(in_state(super::State::Game)))
            .add_systems(Update, statusbar::handle_fast_speed_button.run_if(in_state(super::State::Game)))
            .add_systems(Update, input::grid_click_handler.run_if(in_state(super::State::Game)))
            .add_systems(Update, input::mouse_hover_handler.run_if(in_state(super::State::Game)))
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
