//! Game mechanics and content.

use bevy::prelude::*;

mod animation;
pub mod assets;
pub mod audio;
mod daycycle;
mod debt;
mod map;
mod movement;
pub mod spawn;
pub mod ui;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(node_tree::NodumTreePlugin);

    app.add_plugins((
        animation::plugin,
        audio::plugin,
        assets::plugin,
        movement::plugin,
        spawn::plugin,
        ui::plugin,
    ));

    app.add_plugins((daycycle::plugin, debt::plugin, map::plugin));
}
