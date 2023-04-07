use bevy::prelude::*;

use crate::{
    grid::Map,
    state::loading::GameAssets,
    tower::{
        laser::{spawn_laser_beam, Laser},
        Tower, TowerType,
    },
};

use super::{constants::*, UiData};

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
                        left: Val::Px(0.0),
                        top: Val::Px(50.0),
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
                                            color: TEXT_COLOR,
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
                                        format!("DPS: {:.2}", tower.damage * tower.rate),
                                        TextStyle {
                                            font: game_assets.font.clone(),
                                            font_size: 20.0,
                                            color: TEXT_COLOR,
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
                                            color: RED,
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
                                                        color: BUTTON_TEXT_COLOR,
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
                                color: CARD_TEXT_COLOR,
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
                background_color.0 = BUTTON_BACKGROUND_COLOR_HOVER;
            }
            Interaction::None => {
                background_color.0 = BUTTON_BACKGROUND_COLOR;
            }
        }
    }
}
