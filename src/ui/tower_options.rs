use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};

use crate::{
    audio::{AudioAssets, SoundChannel},
    tower::Tower,
};

use super::{
    constants::*,
    inventory::{draw_tower_card, Inventory},
    UiState, UiStateResource,
};

#[derive(Component)]
pub struct TowerOption {
    pub index: usize,
}

#[derive(Component)]
pub struct TowerOptionsRoot;

pub fn present_tower_options(mut commands: Commands, font: Handle<Font>, towers: &[Tower]) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
            ..Default::default()
        })
        .insert(TowerOptionsRoot)
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Auto, Val::Auto),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    background_color: CARD_BACKGROUND_COLOR.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Select a new tower",
                            TextStyle {
                                font: font.clone(),
                                font_size: 50.0,
                                color: TEXT_COLOR,
                            },
                        ),
                        ..Default::default()
                    });
                });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(400.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Row,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    (0..3).for_each(|i| {
                        draw_tower_card(parent, &towers[i], font.clone(), i, false);
                    });
                });
        });
}

pub fn handle_tower_options(
    mut commands: Commands,
    mut ui_state: ResMut<UiStateResource>,
    mut query: Query<(&TowerOption, &Interaction, &mut BackgroundColor), Changed<Interaction>>,
    root: Query<Entity, With<TowerOptionsRoot>>,
    mut inventory: ResMut<Inventory>,
    sound_channel: Res<AudioChannel<SoundChannel>>,
    audio_assets: Res<AudioAssets>,
) {
    for (tower_option, interaction, mut background_color) in query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                sound_channel.play(audio_assets.blip2.clone());
                if let UiState::PickingTower(ref mut options) = ui_state.state {
                    let tower = options.remove(tower_option.index);
                    inventory.towers.push(tower);
                }
                for entity in root.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                ui_state.state = UiState::Normal;
            }
            Interaction::Hovered => {
                background_color.0 = CARD_BACKGROUND_COLOR_HOVER;
                sound_channel.play(audio_assets.blip1.clone());
            }
            Interaction::None => {
                if let UiState::PlacingTower(i) = ui_state.state {
                    if i == tower_option.index {
                        background_color.0 = CARD_BACKGROUND_COLOR_HOVER;
                    } else {
                        background_color.0 = CARD_BACKGROUND_COLOR;
                    }
                } else {
                    background_color.0 = CARD_BACKGROUND_COLOR;
                }
            }
        }
    }
}
