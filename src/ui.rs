use bevy::prelude::*;

use crate::{inventory::Inventory, state::loading::GameAssets, tower::Tower};

#[derive(Default, Debug, PartialEq, Eq)]
pub enum UiState {
    #[default]
    Normal,
    PlacingTower(usize),
}

#[derive(Resource, Debug, Default)]
pub struct UiData {
    pub hovered_pos: Option<(i8, i8)>,
    pub state: UiState,
}

#[derive(Component)]
pub struct InventoryTower {
    pub index: usize,
}

#[derive(Component)]
pub struct InventoryRoot;

const CARD_BACKGROUND_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
const CARD_BACKGROUND_COLOR_HOVER: Color = Color::rgb(0.9, 0.9, 0.9);

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
                    draw_tower_card(parent, tower, game_assets.font.clone(), i);
                }
            });
    }
}

fn draw_tower_card(parent: &mut ChildBuilder, tower: &Tower, font: Handle<Font>, index: usize) {
    parent
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(200.0), Val::Px(300.0)),
                margin: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            background_color: CARD_BACKGROUND_COLOR.into(),
            ..Default::default()
        })
        .insert(InventoryTower { index })
        .with_children(|parent| {
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
                                color: Color::BLACK,
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
                    background_color: Color::rgb(0.6, 0.6, 0.6).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            tower.variant.description(),
                            TextStyle {
                                font: font.clone(),
                                font_size: 19.0,
                                color: Color::BLACK,
                            },
                        ),
                        style: Style {
                            max_size: Size::new(Val::Px(200.0), Val::Px(60.0)),
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
                            format!("DPS: {}", tower.damage * tower.rate),
                            TextStyle {
                                font: font.clone(),
                                font_size: 17.0,
                                color: Color::BLACK,
                            },
                        ),
                        ..Default::default()
                    });
                });
            // Debuff description
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
                            format!("Side effect: {}", tower.debuff.description()),
                            TextStyle {
                                font,
                                font_size: 20.0,
                                color: Color::RED,
                            },
                        ),
                        style: Style {
                            max_size: Size::new(Val::Px(200.0), Val::Px(50.0)),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
        });
}

pub fn handle_inventory_buttons(
    mut ui_data: ResMut<UiData>,
    mut query: Query<(&InventoryTower, &Interaction, &mut BackgroundColor), Changed<Interaction>>,
) {
    for (inventory_tower, interaction, mut background_color) in query.iter_mut() {
        match interaction {
            Interaction::Clicked => match ui_data.state {
                UiState::PlacingTower(i) => {
                    if i == inventory_tower.index {
                        ui_data.state = UiState::Normal;
                    } else {
                        ui_data.state = UiState::PlacingTower(inventory_tower.index);
                    }
                }
                _ => {
                    ui_data.state = UiState::PlacingTower(inventory_tower.index);
                }
            },
            Interaction::Hovered => {
                background_color.0 = CARD_BACKGROUND_COLOR_HOVER;
            }
            Interaction::None => {
                if let UiState::PlacingTower(i) = ui_data.state {
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
