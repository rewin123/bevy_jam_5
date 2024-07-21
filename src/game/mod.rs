//! Game mechanics and content.

use bevy::prelude::*;

mod animation;
pub mod assets;
pub mod audio;
mod daycycle;
mod debt;
mod highlight;
mod map;
mod movement;
mod selectable;
pub mod spawn;
pub mod ui;
pub mod components;
pub mod resources;

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

    app.add_plugins((
        daycycle::plugin,
        debt::plugin,
        map::plugin,
        selectable::plugin,
        highlight::plugin,
        components::plugin,
        resources::plugin,
    ));
}
