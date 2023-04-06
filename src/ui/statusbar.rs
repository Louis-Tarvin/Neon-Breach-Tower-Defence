use bevy::prelude::*;

use crate::{
    gameplay::{GameManager, WaveState},
    state::loading::GameAssets,
};

use super::{constants::*, UiState, UiStateResource};

#[derive(Component)]
pub struct StatusBarText;

pub fn draw_status_bar(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(50.0)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    ..Default::default()
                },
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: CARD_BACKGROUND_COLOR.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    text: Text::from_section(
                        "Status Bar",
                        TextStyle {
                            font: game_assets.font.clone(),
                            font_size: 30.0,
                            color: TEXT_COLOR,
                        },
                    ),
                    ..Default::default()
                })
                .insert(StatusBarText);
        });
}

pub fn update_status_bar_text(
    mut query: Query<&mut Text, With<StatusBarText>>,
    ui_state: Res<UiStateResource>,
    game_manager: Res<GameManager>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        match game_manager.wave_state {
            WaveState::Spawning(_) | WaveState::Finished => {
                text.sections[0].value = format!("Wave {}", game_manager.current_wave);
            }
            WaveState::Waiting => match ui_state.state {
                UiState::Normal => {
                    text.sections[0].value =
                        "Once you have placed your towers, press SPACE to start the next wave."
                            .to_string();
                }
                UiState::PlacingTower(_) => {
                    text.sections[0].value =
                        "Click on an unoccupied tile to place a tower. Click the card again to cancel."
                            .to_string();
                }
                UiState::PickingTower(_) => {
                    text.sections[0].value = "Choose a new tower".to_string();
                }
            },
        }
    }
}
