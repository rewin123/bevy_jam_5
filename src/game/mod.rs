//! Game mechanics and content.

use bevy::prelude::*;

mod animation;
pub mod assets;
pub mod audio;
pub mod auto_anim;
pub mod bilboard_state;
pub mod character;
pub mod components;
mod daycycle;
mod debt;
pub mod device_state;
mod highlight;
mod map;
mod metal_trash;
mod movement;
mod pc_work;
mod resource_flow;
pub mod resources;
mod selectable;
mod sequence;
pub mod spawn;
pub mod sprite_material;
mod trouble_planner;
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

    app.add_plugins((
        daycycle::plugin,
        trouble_planner::plugin,
        character::plugin,
        debt::plugin,
        map::plugin,
        selectable::plugin,
        highlight::plugin,
        components::plugin,
        resources::plugin,
        sprite_material::SpriteMaterialPlugin,
        pc_work::plugin,
        metal_trash::plugin,
        sequence::plugin,
        resource_flow::plugin,
        bilboard_state::plugin,
    ));
}
