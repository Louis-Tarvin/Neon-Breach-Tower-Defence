use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};
use bevy_asset_loader::prelude::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "fonts/roboto.ttf")]
    pub font: Handle<Font>,
    #[asset(path = "enemy.png")]
    pub enemy: Handle<Image>,
    #[asset(path = "tiles/empty.png")]
    pub empty_tile: Handle<Image>,
    #[asset(path = "tiles/buildable.png")]
    pub buildable_tile: Handle<Image>,
    #[asset(path = "tiles/chargeshot.png")]
    pub charge_shot: Handle<Image>,
    #[asset(path = "tiles/laser.png")]
    pub laser: Handle<Image>,
    #[asset(path = "bullet.png")]
    pub bullet: Handle<Image>,
}

#[derive(Component)]
pub struct LoadingNode;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..Default::default()
            },
            tonemapping: bevy::core_pipeline::tonemapping::Tonemapping::TonyMcMapface,
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
                    color: Color::BLACK,
                },
            ));
        });
}

pub fn cleanup(mut commands: Commands, query: Query<Entity, With<LoadingNode>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
