use bevy::prelude::*;

use crate::{state::loading::GameAssets, tower::Tower};

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

pub fn give_random_tower(mut inventory: ResMut<Inventory>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Return) {
        inventory.towers.push(Tower::new_random());
    }
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
                    size: Size::new(Val::Percent(100.0), Val::Px(300.0)),
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(0.0),
                        bottom: Val::Px(0.0),
                        ..Default::default()
                    },
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
            size: Size::new(Val::Px(200.0), Val::Px(300.0)),
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
                    size: Size::new(Val::Percent(100.0), Val::Px(50.0)),
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
                    size: Size::new(Val::Percent(100.0), Val::Px(60.0)),
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
                        max_size: Size::new(Val::Px(190.0), Val::Px(60.0)),
                        ..Default::default()
                    },
                    ..Default::default()
                });
            });
        // Tower stats
        parent
            .spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Px(30.0)),
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
                    size: Size::new(Val::Percent(100.0), Val::Px(100.0)),
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
                        max_size: Size::new(Val::Px(190.0), Val::Px(100.0)),
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
) {
    if let UiState::PickingTower(_) = ui_state.state {
        return;
    }
    for (inventory_tower, interaction, mut background_color) in query.iter_mut() {
        match interaction {
            Interaction::Clicked => match ui_state.state {
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
            },
            Interaction::Hovered => {
                background_color.0 = CARD_BACKGROUND_COLOR_HOVER;
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
