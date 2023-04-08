use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};
use bevy_asset_loader::prelude::AssetCollection;

use crate::ui::constants::TEXT_COLOR;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "fonts/roboto.ttf")]
    pub font: Handle<Font>,
    #[asset(path = "titlecard.png")]
    pub titlecard: Handle<Image>,
    #[asset(path = "enemy1.png")]
    pub enemy1: Handle<Image>,
    #[asset(path = "enemy2.png")]
    pub enemy2: Handle<Image>,
    #[asset(path = "enemy3.png")]
    pub enemy3: Handle<Image>,
    #[asset(path = "enemy4.png")]
    pub enemy4: Handle<Image>,
    #[asset(path = "enemy5.png")]
    pub enemy5: Handle<Image>,
    #[asset(path = "enemy6.png")]
    pub enemy6: Handle<Image>,
    #[asset(path = "enemy7.png")]
    pub enemy7: Handle<Image>,
    #[asset(path = "tiles/tile_select.png")]
    pub tile_select: Handle<Image>,
    #[asset(path = "tiles/empty.png")]
    pub empty_tile: Handle<Image>,
    #[asset(path = "tiles/buildable.png")]
    pub buildable_tile: Handle<Image>,
    #[asset(path = "tiles/pivot.png")]
    pub pivot: Handle<Image>,
    #[asset(path = "tiles/laser.png")]
    pub laser: Handle<Image>,
    #[asset(path = "tiles/missile_silo.png")]
    pub silo: Handle<Image>,
    #[asset(path = "tiles/turret.png")]
    pub turret: Handle<Image>,
    #[asset(path = "tiles/sniper.png")]
    pub sniper: Handle<Image>,
    #[asset(path = "tiles/dish.png")]
    pub dish: Handle<Image>,
    #[asset(path = "bullet.png")]
    pub bullet: Handle<Image>,
    #[asset(path = "missile.png")]
    pub missile: Handle<Image>,
    #[asset(path = "tiles/overheat.png")]
    pub overheat: Handle<Image>,
}

#[derive(Component)]
pub struct LoadingNode;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: !cfg!(target_arch = "wasm32"),
                ..Default::default()
            },
            tonemapping: if cfg!(target_arch = "wasm32") {
                bevy::core_pipeline::tonemapping::Tonemapping::default()
            } else {
                bevy::core_pipeline::tonemapping::Tonemapping::TonyMcMapface
            },
            ..Default::default()
        },
        BloomSettings::default(),
    ));
    commands
        .spawn(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                padding: UiRect::all(Val::Px(20.)),
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(LoadingNode)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Loading...",
                TextStyle {
                    font: asset_server.load("fonts/roboto.ttf"),
                    font_size: 40.0,
                    color: TEXT_COLOR,
                },
            ));
        });
}

pub fn cleanup(mut commands: Commands, query: Query<Entity, With<LoadingNode>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
