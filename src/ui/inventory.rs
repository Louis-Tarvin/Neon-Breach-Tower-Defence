use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};

use crate::{
    audio::{AudioAssets, SoundChannel},
    state::loading::GameAssets,
    tower::{Tower, TowerType},
};

use super::{constants::*, tower_options::TowerOption, UiState, UiStateResource};

#[derive(Resource, Default, Debug)]
pub struct Inventory {
    pub towers: Vec<Tower>,
}

#[derive(Component)]
pub struct InventoryRoot;

#[derive(Component)]
pub struct InventoryTower {
    pub index: usize,
}

pub fn draw_inventory(
    mut commands: Commands,
    inventory: Res<Inventory>,
    query: Query<Entity, With<InventoryRoot>>,
    game_assets: Res<GameAssets>,
) {
    if inventory.is_changed() {
        // Remove old inventory
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        // Draw new inventory
        commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(300.0),

                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(InventoryRoot)
            .with_children(|parent| {
                for (i, tower) in inventory.towers.iter().enumerate() {
                    draw_tower_card(parent, tower, game_assets.font.clone(), i, true);
                }
            });
    }
}

pub fn draw_tower_card(
    parent: &mut ChildBuilder,
    tower: &Tower,
    font: Handle<Font>,
    index: usize,
    is_inventory: bool,
) {
    let mut card = parent.spawn(ButtonBundle {
        style: Style {
            width: Val::Px(200.0),
            height: Val::Px(300.0),

            margin: UiRect::all(Val::Px(10.0)),
            padding: UiRect::all(Val::Px(5.0)),
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        background_color: CARD_BACKGROUND_COLOR.into(),
        ..Default::default()
    });
    if is_inventory {
        card.insert(InventoryTower { index });
    } else {
        card.insert(TowerOption { index });
    }
    card.with_children(|parent| {
        // Tower name
        parent
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(50.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        tower.variant.name(),
                        TextStyle {
                            font: font.clone(),
                            font_size: 20.0,
                            color: CARD_TEXT_COLOR,
                        },
                    ),
                    ..Default::default()
                });
            });
        // Tower description
        parent
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(60.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        tower.variant.description(),
                        TextStyle {
                            font: font.clone(),
                            font_size: 19.0,
                            color: CARD_TEXT_COLOR,
                        },
                    ),
                    style: Style {
                        max_width: Val::Px(190.0),
                        max_height: Val::Px(60.0),
                        ..Default::default()
                    },
                    ..Default::default()
                });
            });
        // Tower stats
        parent
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(30.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        format!("DPS: {:.2}", tower.damage * tower.rate),
                        TextStyle {
                            font: font.clone(),
                            font_size: 17.0,
                            color: CARD_TEXT_COLOR,
                        },
                    ),
                    ..Default::default()
                });
            });
        // Debuff description
        parent
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(100.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        format!("Side effect: {}", tower.debuff.description()),
                        TextStyle {
                            font,
                            font_size: 20.0,
                            color: RED,
                        },
                    ),
                    style: Style {
                        max_width: Val::Px(190.0),
                        max_height: Val::Px(120.0),
                        ..Default::default()
                    },
                    ..Default::default()
                });
            });
    });
}

pub fn handle_inventory_buttons(
    mut ui_state: ResMut<UiStateResource>,
    mut query: Query<(&InventoryTower, &Interaction, &mut BackgroundColor), Changed<Interaction>>,
    sound_channel: Res<AudioChannel<SoundChannel>>,
    audio_assets: Res<AudioAssets>,
) {
    if let UiState::PickingTower(_) = ui_state.state {
        return;
    }
    for (inventory_tower, interaction, mut background_color) in query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                sound_channel.play(audio_assets.blip2.clone());
                match ui_state.state {
                    UiState::PlacingTower(i) => {
                        if i == inventory_tower.index {
                            ui_state.state = UiState::Normal;
                        } else {
                            ui_state.state = UiState::PlacingTower(inventory_tower.index);
                        }
                    }
                    _ => {
                        ui_state.state = UiState::PlacingTower(inventory_tower.index);
                    }
                }
            }
            Interaction::Hovered => {
                background_color.0 = CARD_BACKGROUND_COLOR_HOVER;
                match ui_state.state {
                    UiState::PlacingTower(_) => {}
                    _ => {
                        sound_channel.play(audio_assets.blip1.clone());
                    }
                }
            }
            Interaction::None => {
                if let UiState::PlacingTower(i) = ui_state.state {
                    if i == inventory_tower.index {
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

#[derive(Component)]
pub struct Ghost;

pub fn create_ghost(
    mut commands: Commands,
    query: Query<Entity, With<Ghost>>,
    ui_state: Res<UiStateResource>,
    inventory: Res<Inventory>,
    game_assets: Res<GameAssets>,
) {
    if !ui_state.is_changed() {
        return;
    }
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    if let UiState::PlacingTower(i) = ui_state.state {
        if let Some(tower) = inventory.towers.get(i) {
            for entity in query.iter() {
                commands.entity(entity).despawn_recursive();
            }
            commands
                .spawn(SpriteBundle {
                    texture: match tower.variant {
                        TowerType::ChargeShot => game_assets.pivot.clone(),
                        TowerType::Laser => game_assets.laser.clone(),
                        TowerType::Missile => game_assets.silo.clone(),
                        TowerType::Sniper => game_assets.pivot.clone(),
                        TowerType::Jammer => game_assets.pivot.clone(),
                    },
                    transform: Transform::from_xyz(0.0, 0.0, -1.0),
                    sprite: Sprite {
                        color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Ghost);
        }
    }
}

pub fn handle_ghost(
    mut query: Query<&mut Transform, With<Ghost>>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    for mut transform in query.iter_mut() {
        if let Ok(window) = windows.get_single() {
            let (camera, camera_transform) = camera.get_single().unwrap();
            if let Some(mouse_position) = window.cursor_position() {
                if let Some(world_position) =
                    camera.viewport_to_world(camera_transform, mouse_position)
                {
                    let x = (world_position.origin.x / 32.0).round() * 32.0;
                    let y = (world_position.origin.y / 32.0).round() * 32.0;
                    transform.translation = Vec3::new(x, y, 6.0);
                }
            }
        }
    }
}
