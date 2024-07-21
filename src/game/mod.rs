//! Game mechanics and content.

use bevy::prelude::*;

mod animation;
pub mod assets;
pub mod audio;
mod movement;
pub mod spawn;
mod debt;
mod daycycle;

pub(super) fn plugin(app: &mut App) {

    app.add_plugins(node_tree::NodumTreePlugin);

    app.add_plugins((
        animation::plugin,
        audio::plugin,
        assets::plugin,
        movement::plugin,
        spawn::plugin,
    ));


    app.add_plugins((
        daycycle::plugin,
        debt::plugin
    ));
}
