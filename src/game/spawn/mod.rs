//! Handles spawning of entities. Here, we are using
//! [observers](https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.Observer.html)
//! for this, but you could also use `Events<E>` or `Commands`.

use bevy::prelude::*;
use level::SpawnLevel;

use super::resources::{CarbonDioxide, Oxygen};

pub mod level;
pub mod player;
pub mod spawn_commands;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((level::plugin, player::plugin));

    app.observe(setup_resource);
}



fn setup_resource(
    _: Trigger<SpawnLevel>,
    mut commands: Commands,
) {
    commands.insert_resource(Oxygen::new(50.0, 100.0));
    commands.insert_resource(CarbonDioxide::new(50.0, 100.0));
}