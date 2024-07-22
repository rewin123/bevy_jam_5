//! Game mechanics and content.

use bevy::prelude::*;

mod animation;
pub mod assets;
pub mod audio;
pub mod auto_anim;
mod character;
pub mod components;
mod daycycle;
mod debt;
mod highlight;
mod map;
mod movement;
pub mod resources;
mod selectable;
pub mod spawn;
pub mod sprite_material;
pub mod ui;
mod pc_work;
mod sequence;

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
        character::plugin,
        debt::plugin,
        map::plugin,
        selectable::plugin,
        highlight::plugin,
        components::plugin,
        resources::plugin,
        sprite_material::SpriteMaterialPlugin,
        pc_work::plugin,
        sequence::plugin,
    ));
}
