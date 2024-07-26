//! Game mechanics and content.

use bevy::prelude::*;
use bevy_registry_export::ExportRegistryPlugin;

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
mod kitchen_work;
mod map;
pub mod metal_trash;
mod movement;
mod pc_work;
pub mod render;
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

    app.add_plugins(ExportRegistryPlugin::default());

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
        metal_trash::plugin,
        components::plugin,
        resources::plugin,
        pc_work::plugin,
        kitchen_work::plugin,
        sequence::plugin,
        resource_flow::plugin,
        bilboard_state::plugin,
    ));

    app.add_plugins(sprite_material::SpriteMaterialPlugin);

    app.add_plugins((render::plugin,));
}
