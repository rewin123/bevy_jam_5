use crate::game::spawn::level::SpawnLevel;
use bevy::prelude::*;

use super::*;

pub(crate) fn plugin(app: &mut App) {
    app.observe(spawn_map);
}

fn spawn_map(_: Trigger<SpawnLevel>,
     mut commands: Commands,
     asset_server: Res<AssetServer>
) {
    let ship_scene = "models/my_ship.glb#Scene0";

    commands.spawn(SceneBundle {
        scene: asset_server.load(ship_scene),
        transform: Transform::from_translation(Vec3::new(3.5, -0.2, 3.5)),
        ..default()
    });

    // let mut map = ShipMap::new(10, 10);

    // for x in 0..map.width() {
    //     for y in 0..map.height() {
    //         map.set(x, y, Tile::Floor);
    //     }
    // }

    // //fill edges with wall
    // for x in 0..map.width() {
    //     map.set(x, 0, Tile::Wall);
    //     map.set(x, map.height() - 1, Tile::Wall);
    // }
    // for y in 0..map.height() {
    //     map.set(0, y, Tile::Wall);
    //     map.set(map.width() - 1, y, Tile::Wall);
    // }

    // commands.spawn((SpatialBundle::default(), map));
}
