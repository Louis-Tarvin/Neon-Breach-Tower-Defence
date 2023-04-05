use bevy::prelude::*;

use crate::{
    grid::Map,
    input::HoverPosition,
    inventory::Inventory,
    state::loading::GameAssets,
    tower::{
        laser::{spawn_laser_beam, Laser},
        Tower, TowerType,
    },
};

const CARD_BACKGROUND_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
const CARD_BACKGROUND_COLOR_HOVER: Color = Color::rgb(0.9, 0.9, 0.9);

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
pub struct UiData {
    pub hovered_pos: Option<(i8, i8)>,
    pub selected_pos: Option<(i8, i8)>,
    pub state: UiState,
}

#[derive(Component)]
pub struct InventoryTower {
    pub index: usize,
}

#[derive(Component)]
pub struct TowerOption {
    pub index: usize,
}

#[derive(Component)]
pub struct InventoryRoot;

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

fn draw_tower_card(
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
    match ui_data.state {
        UiState::PlacingTower(_) | UiState::PickingTower(_) => return,
        _ => (),
    }
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

pub fn handle_tower_options(
    mut commands: Commands,
    mut ui_data: ResMut<UiData>,
    mut query: Query<(&TowerOption, &Interaction, &mut BackgroundColor), Changed<Interaction>>,
    root: Query<Entity, With<TowerOptionsRoot>>,
    mut inventory: ResMut<Inventory>,
) {
    for (tower_option, interaction, mut background_color) in query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                if let UiState::PickingTower(ref mut options) = ui_data.state {
                    let tower = options.remove(tower_option.index);
                    inventory.towers.push(tower);
                }
                ui_data.state = UiState::Normal;
                for entity in root.iter() {
                    commands.entity(entity).despawn_recursive();
                }
            }
            Interaction::Hovered => {
                background_color.0 = CARD_BACKGROUND_COLOR_HOVER;
            }
            Interaction::None => {
                if let UiState::PlacingTower(i) = ui_data.state {
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

#[derive(Component)]
pub struct SidebarRoot;

#[derive(Component)]
pub struct RotationButton;

pub fn draw_sidebar(
    mut commands: Commands,
    query: Query<Entity, With<SidebarRoot>>,
    towers: Query<&Tower>,
    ui_data: Res<UiData>,
    map: Res<Map>,
    game_assets: Res<GameAssets>,
) {
    if ui_data.is_changed() {
        // Remove old sidebar
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        // Draw new sidebar
        commands
            .spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(250.0), Val::Auto),
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        right: Val::Px(0.0),
                        top: Val::Px(0.0),
                        ..Default::default()
                    },
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..Default::default()
                },
                background_color: CARD_BACKGROUND_COLOR.into(),
                ..Default::default()
            })
            .insert(SidebarRoot)
            .with_children(|parent| {
                if let Some(grid_pos) = ui_data.selected_pos {
                    if let Some(entity) = map.placements.get(&grid_pos) {
                        // Tower is selected
                        let tower = towers.get(*entity).expect("Tower entity not found");
                        // Display tower name
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
                                        format!("Selected: {}", tower.variant.name()),
                                        TextStyle {
                                            font: game_assets.font.clone(),
                                            font_size: 20.0,
                                            color: Color::BLACK,
                                        },
                                    ),
                                    ..Default::default()
                                });
                            });
                        // Display tower DPS
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
                                            font: game_assets.font.clone(),
                                            font_size: 20.0,
                                            color: Color::BLACK,
                                        },
                                    ),
                                    ..Default::default()
                                });
                            });
                        // Display tower debuff
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
                                            font: game_assets.font.clone(),
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
                        // If tower is a laser, show a button to toggle its rotation
                        if let TowerType::Laser = tower.variant {
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Px(50.0)),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent
                                        .spawn(ButtonBundle {
                                            style: Style {
                                                size: Size::new(Val::Percent(100.0), Val::Px(50.0)),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(RotationButton)
                                        .with_children(|parent| {
                                            parent.spawn(TextBundle {
                                                text: Text::from_section(
                                                    "Toggle rotation",
                                                    TextStyle {
                                                        font: game_assets.font.clone(),
                                                        font_size: 20.0,
                                                        color: Color::BLACK,
                                                    },
                                                ),
                                                ..Default::default()
                                            });
                                        });
                                });
                        }
                    }
                } else {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "No tower selected",
                            TextStyle {
                                font: game_assets.font.clone(),
                                font_size: 20.0,
                                color: Color::BLACK,
                            },
                        ),
                        ..Default::default()
                    });
                }
            });
    }
}

pub fn handle_toggle_rotation_button(
    mut commands: Commands,
    mut query: Query<
        (&Interaction, &mut BackgroundColor),
        (With<RotationButton>, Changed<Interaction>),
    >,
    mut lasers: Query<(Entity, &mut Transform, &mut Laser, &Children)>,
    ui_data: Res<UiData>,
    map: Res<Map>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    for (interaction, mut background_color) in query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                if let Some(grid_pos) = ui_data.selected_pos {
                    if let Some(entity) = map.placements.get(&grid_pos) {
                        let (entity, mut transform, mut laser, children) =
                            lasers.get_mut(*entity).unwrap();
                        // Change laser direction
                        transform.rotation *= Quat::from_rotation_z(-std::f32::consts::PI / 2.0);
                        laser.toggle_direction();
                        // Remove and redraw beam
                        for child in children.iter() {
                            commands.entity(*child).despawn_recursive();
                        }
                        commands.entity(entity).with_children(|parent| {
                            spawn_laser_beam(
                                parent,
                                grid_pos,
                                laser.direction,
                                meshes,
                                materials,
                                &map,
                            )
                        });
                    }
                }
                break;
            }
            Interaction::Hovered => {
                background_color.0 = Color::rgb(0.8, 0.8, 0.8);
            }
            Interaction::None => {
                background_color.0 = Color::rgb(0.9, 0.9, 0.9);
            }
        }
    }
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
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(TowerOptionsRoot)
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(400.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Select a tower",
                            TextStyle {
                                font: font.clone(),
                                font_size: 30.0,
                                color: Color::BLACK,
                            },
                        ),
                        ..Default::default()
                    });
                });
            (0..3).for_each(|i| {
                draw_tower_card(parent, &towers[i], font.clone(), i, false);
            });
        });
}
