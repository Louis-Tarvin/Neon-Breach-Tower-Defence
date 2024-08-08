use bevy::prelude::*;
use bevy_jornet::{JornetPlugin, Leaderboard, Score};
use bevy_kira_audio::{AudioChannel, AudioControl};

use crate::{
    audio::{AudioAssets, DrumsChannel, MusicChannel, SoundChannel},
    ui::constants::*,
};

use super::loading::GameAssets;
use super::State;

pub struct ResultsPlugin;

impl Plugin for ResultsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Scores::default());
        if let Some(id) = option_env!("JORNET_ID") {
            if let Some(key) = option_env!("JORNET_KEY") {
                app.add_plugins(JornetPlugin::with_leaderboard(id, key))
                    .add_systems(OnEnter(State::Results), save_score)
                    .add_systems(Update, draw_leaderboard.run_if(in_state(State::Results)))
                    .add_systems(
                        Update,
                        handle_main_menu_button.run_if(in_state(State::Results)),
                    )
                    .add_systems(Update, refresh_after_timer.run_if(in_state(State::Results)))
                    .add_systems(OnExit(State::Results), cleanup);
            }
        }
    }
}

#[derive(Resource, Debug, Default)]
pub struct Scores {
    pub high_score: u32,
    pub last_score: u32,
    pub last_wave: u32,
}

pub fn create_player(mut leaderboard: ResMut<Leaderboard>) {
    leaderboard.create_player(None);
}

fn save_score(mut commands: Commands, leaderboard: Res<Leaderboard>, scores: Res<Scores>) {
    leaderboard.send_score_with_meta(
        scores.last_score as f32,
        &format!("Wave {}", scores.last_wave),
    );
    leaderboard.refresh_leaderboard();
    commands.insert_resource(RefreshTimer(Timer::from_seconds(3.0, TimerMode::Once)));
}

#[derive(Component)]
pub struct LeaderboardRoot;
#[derive(Component)]
pub struct MainMenuButton;

fn draw_leaderboard(
    mut commands: Commands,
    leaderboard: Res<Leaderboard>,
    query: Query<Entity, With<LeaderboardRoot>>,
    game_assets: Res<GameAssets>,
    score: Res<Scores>,
) {
    if !leaderboard.is_changed() {
        return;
    }
    if let Some(entity) = query.iter().next() {
        commands.entity(entity).despawn_recursive();
    }
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            background_color: BACKGROUND_COLOR.into(),
            ..Default::default()
        })
        .insert(LeaderboardRoot)
        .with_children(|parent| {
            // Title
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(70.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Leaderboard",
                            TextStyle {
                                font: game_assets.font.clone(),
                                font_size: 50.0,
                                color: TEXT_COLOR,
                            },
                        ),
                        ..Default::default()
                    });
                });
            // Main body
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(800.),
                        height: Val::Auto,
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    let mut scores = leaderboard.get_leaderboard();
                    let count = scores.len();
                    scores.sort_unstable_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
                    if let Some(player) = leaderboard.get_player() {
                        if let Some(rank) = scores.iter().position(|s| {
                            s.player == player.name && s.score == score.last_score as f32
                        }) {
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        height: Val::Px(70.0),

                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..Default::default()
                                    },
                                    background_color: CARD_BACKGROUND_COLOR_HOVER.into(),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(TextBundle {
                                        text: Text::from_section(
                                            format!("You are ranked {} out of {}", rank + 1, count),
                                            TextStyle {
                                                font: game_assets.font.clone(),
                                                font_size: 30.0,
                                                color: TEXT_COLOR,
                                            },
                                        ),
                                        ..Default::default()
                                    });
                                });
                        }
                    }
                    scores.truncate(10);
                    for score in scores {
                        draw_leaderboard_row(parent, game_assets.font.clone(), score);
                    }
                });
            // Display player name
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(50.0),

                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    let name = match leaderboard.get_player() {
                        Some(player) => player.name.clone(),
                        None => "Error".to_string(),
                    };
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            format!("You are: {}", name),
                            TextStyle {
                                font: game_assets.font.clone(),
                                font_size: 30.0,
                                color: TEXT_COLOR,
                            },
                        ),
                        ..Default::default()
                    });
                });
            // Display player score
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            format!(
                                "Your score: {} (High score: {})",
                                score.last_score, score.high_score
                            ),
                            TextStyle {
                                font: game_assets.font.clone(),
                                font_size: 30.0,
                                color: TEXT_COLOR,
                            },
                        ),
                        ..Default::default()
                    });
                });
            // Main menu button
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(MainMenuButton)
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Main Menu",
                            TextStyle {
                                font: game_assets.font.clone(),
                                font_size: 30.0,
                                color: TEXT_COLOR,
                            },
                        ),
                        ..Default::default()
                    });
                });
        });
}

fn draw_leaderboard_row(parent: &mut ChildBuilder, font: Handle<Font>, score: Score) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(40.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(33.3),
                        height: Val::Px(40.0),

                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            score.player,
                            TextStyle {
                                font: font.clone(),
                                font_size: 25.0,
                                color: TEXT_COLOR,
                            },
                        ),
                        ..Default::default()
                    });
                });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(33.3),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            format!("{}", score.score),
                            TextStyle {
                                font: font.clone(),
                                font_size: 25.0,
                                color: TEXT_COLOR,
                            },
                        ),
                        ..Default::default()
                    });
                });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(33.3),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            score.meta.unwrap_or("".to_string()),
                            TextStyle {
                                font: font.clone(),
                                font_size: 25.0,
                                color: TEXT_COLOR,
                            },
                        ),
                        ..Default::default()
                    });
                });
        });
}

#[derive(Resource)]
pub struct RefreshTimer(pub Timer);

fn refresh_after_timer(
    leaderboard: Res<Leaderboard>,
    mut timer: ResMut<RefreshTimer>,
    time: Res<Time>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        leaderboard.refresh_leaderboard();
    }
}

fn handle_main_menu_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<MainMenuButton>),
    >,
    mut next_state: ResMut<NextState<State>>,
    sound_channel: Res<AudioChannel<SoundChannel>>,
    audio_assets: Res<AudioAssets>,
) {
    for (interaction, mut background_color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                next_state.set(State::MainMenu);
                sound_channel.play(audio_assets.blip2.clone());
            }
            Interaction::Hovered => {
                background_color.0 = BUTTON_BACKGROUND_COLOR_HOVER;
                sound_channel.play(audio_assets.blip1.clone());
            }
            Interaction::None => {
                background_color.0 = BUTTON_BACKGROUND_COLOR;
            }
        }
    }
}

fn cleanup(
    mut commands: Commands,
    query: Query<Entity, With<LeaderboardRoot>>,
    music_channel: Res<AudioChannel<MusicChannel>>,
    drums_channel: Res<AudioChannel<DrumsChannel>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    music_channel.stop();
    drums_channel.stop();
}
