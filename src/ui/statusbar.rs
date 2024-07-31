use bevy::prelude::*;

use crate::{
    gameplay::{GameManager, WaveState},
    state::loading::GameAssets,
};

use super::{constants::*, UiState, UiStateResource};

#[derive(Resource)]
pub struct GameSpeed(pub bool);

#[derive(Component)]
pub struct NormalSpeedButton;
#[derive(Component)]
pub struct FastSpeedButton;
#[derive(Component)]
pub struct StatusBarScore;
#[derive(Component)]
pub struct StatusBarLives;
#[derive(Component)]
pub struct StatusBarText;

pub fn draw_status_bar(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(50.0),
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: CARD_BACKGROUND_COLOR.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Auto,
                        height: Val::Px(50.0),
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                width: Val::Px(50.0),
                                height: Val::Px(50.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                padding: UiRect::all(Val::Px(10.0)),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text::from_section(
                                    ">",
                                    TextStyle {
                                        font: game_assets.font.clone(),
                                        font_size: 30.0,
                                        color: TEXT_COLOR,
                                    },
                                ),
                                ..Default::default()
                            });
                        })
                        .insert(NormalSpeedButton);
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                width: Val::Px(50.0),
                                height: Val::Px(50.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                padding: UiRect::all(Val::Px(10.0)),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text::from_section(
                                    ">>",
                                    TextStyle {
                                        font: game_assets.font.clone(),
                                        font_size: 30.0,
                                        color: TEXT_COLOR,
                                    },
                                ),
                                ..Default::default()
                            });
                        })
                        .insert(FastSpeedButton);
                });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Auto,
                        height: Val::Px(50.0),
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(TextBundle {
                            text: Text::from_section(
                                "Score: 0",
                                TextStyle {
                                    font: game_assets.font.clone(),
                                    font_size: 30.0,
                                    color: TEXT_COLOR,
                                },
                            ),
                            ..Default::default()
                        })
                        .insert(StatusBarScore);
                });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Auto,
                        height: Val::Px(50.0),
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(TextBundle {
                            text: Text::from_section(
                                "Lives: 10",
                                TextStyle {
                                    font: game_assets.font.clone(),
                                    font_size: 30.0,
                                    color: TEXT_COLOR,
                                },
                            ),
                            ..Default::default()
                        })
                        .insert(StatusBarLives);
                });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(50.0),
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..Default::default()
                    },
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
        });
}

pub fn handle_normal_speed_button(
    mut query: Query<(&Interaction, &mut BackgroundColor), With<NormalSpeedButton>>,
    mut game_speed: ResMut<GameSpeed>,
    mut time: ResMut<Time>,
) {
    for (interaction, mut background_color) in query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                game_speed.0 = false;
                time.set_relative_speed(1.0);
                background_color.0 = BUTTON_BACKGROUND_COLOR_ACTIVE;
            }
            Interaction::Hovered => {
                background_color.0 = BUTTON_BACKGROUND_COLOR_HOVER;
            }
            Interaction::None => {
                background_color.0 = if game_speed.0 {
                    BUTTON_BACKGROUND_COLOR
                } else {
                    BUTTON_BACKGROUND_COLOR_ACTIVE
                };
            }
        }
    }
}

pub fn handle_fast_speed_button(
    mut query: Query<(&Interaction, &mut BackgroundColor), With<FastSpeedButton>>,
    mut game_speed: ResMut<GameSpeed>,
    mut time: ResMut<Time>,
) {
    for (interaction, mut background_color) in query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                game_speed.0 = true;
                time.set_relative_speed(3.0);
                background_color.0 = BUTTON_BACKGROUND_COLOR_ACTIVE;
            }
            Interaction::Hovered => {
                background_color.0 = BUTTON_BACKGROUND_COLOR_HOVER;
            }
            Interaction::None => {
                background_color.0 = if game_speed.0 {
                    BUTTON_BACKGROUND_COLOR_ACTIVE
                } else {
                    BUTTON_BACKGROUND_COLOR
                };
            }
        }
    }
}

pub fn update_status_bar_text(
    mut query: Query<&mut Text, With<StatusBarText>>,
    ui_state: Res<UiStateResource>,
    game_manager: Res<GameManager>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        match game_manager.wave_state {
            WaveState::Spawning(_) | WaveState::Finished => {
                text.sections[0].value = format!("Wave: {}", game_manager.current_wave);
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

pub fn update_score_text(
    mut query: Query<&mut Text, With<StatusBarScore>>,
    game_manager: Res<GameManager>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[0].value = format!("Score: {}", game_manager.score);
    }
}

pub fn update_lives_text(
    mut query: Query<&mut Text, With<StatusBarLives>>,
    game_manager: Res<GameManager>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[0].value = format!("Lives: {}", game_manager.lives);
    }
}
