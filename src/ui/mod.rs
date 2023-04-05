use bevy::prelude::*;

use crate::{input::HoverPosition, state::loading::GameAssets, tower::Tower};

pub mod constants;
pub mod inventory;
pub mod sidebar;
pub mod tower_options;

#[derive(Default, Debug)]
pub enum UiState {
    #[default]
    Normal,
    PlacingTower(usize),
    PickingTower(Vec<Tower>),
}
impl PartialEq for UiState {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (UiState::Normal, UiState::Normal)
                | (UiState::PlacingTower(_), UiState::PlacingTower(_))
                | (UiState::PickingTower(_), UiState::PickingTower(_))
        )
    }
}

#[derive(Resource, Debug, Default)]
pub struct UiStateResource {
    pub state: UiState,
}

#[derive(Resource, Debug, Default)]
pub struct UiData {
    pub hovered_pos: Option<(i8, i8)>,
    pub selected_pos: Option<(i8, i8)>,
}

#[derive(Component)]
pub struct SelectionIndicator;

pub fn update_selection_indicator(
    mut commands: Commands,
    hovered_pos: Res<HoverPosition>,
    mut query: Query<&mut Transform, With<SelectionIndicator>>,
    game_assets: Res<GameAssets>,
) {
    if hovered_pos.is_changed() {
        if let Some((x, y)) = hovered_pos.0 {
            if let Ok(mut transform) = query.get_single_mut() {
                transform.translation = Vec3::new(x as f32 * 32.0, y as f32 * 32.0, 4.0);
            } else {
                commands
                    .spawn(SpriteBundle {
                        texture: game_assets.tile_select.clone(),
                        transform: Transform::from_translation(Vec3::new(x as f32, y as f32, 4.0)),
                        ..Default::default()
                    })
                    .insert(SelectionIndicator);
            }
        }
    }
}
