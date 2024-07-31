use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{grid::Map, state::loading::GameAssets};

use super::{debuffs::SpeedUpPoint, RangeIndicator, Tower, TowerPlaced};

#[derive(Component, Debug)]
pub struct RotatingDish;

pub fn spawn_jammer(
    tower: Tower,
    mut commands: Commands,
    grid_pos: (i8, i8),
    game_assets: Res<GameAssets>,
    mut event_writer: EventWriter<TowerPlaced>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut map: ResMut<Map>,
) {
    let entity = commands
        .spawn(SpriteBundle {
            texture: game_assets.pivot.clone(),
            transform: Transform::from_translation(Vec3::new(
                grid_pos.0 as f32 * 32.0,
                grid_pos.1 as f32 * 32.0,
                1.0,
            )),
            ..Default::default()
        })
        .insert(tower)
        .with_children(|parent| {
            // Circle used to show the range of the tower
            parent
                .spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(Circle::new(1.5 * 32.0)).into(),
                    material: materials.add(ColorMaterial::from(Color::rgba(0.8, 0.4, 0.4, 0.2))),
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)),
                    visibility: Visibility::Hidden,
                    ..Default::default()
                })
                .insert(RangeIndicator);
            parent
                .spawn(SpriteBundle {
                    texture: game_assets.dish.clone(),
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                    ..Default::default()
                })
                .insert(RotatingDish);
        })
        .id();
    map.place_tower(grid_pos, entity).unwrap();
    event_writer.send(TowerPlaced { grid_pos });
    commands
        .spawn(SpeedUpPoint(-40.0))
        .insert(Transform::from_translation(Vec3::new(
            grid_pos.0 as f32 * 32.0,
            grid_pos.1 as f32 * 32.0,
            1.0,
        )));
}

pub fn rotate_dish(mut query: Query<&mut Transform, With<RotatingDish>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_z(time.delta_seconds() * 0.5);
    }
}
