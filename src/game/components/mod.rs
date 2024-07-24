use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub mod earth;
pub mod fire;
pub mod flowup_text;
pub mod metal_trash_pile;
pub mod oxygen_recycler;
pub mod pc;
pub mod toilet;
pub mod water_dispenser;

pub(crate) fn plugin(app: &mut bevy::prelude::App) {
    app.add_plugins((
        earth::plugin,
        fire::plugin,
        pc::plugin,
        flowup_text::plugin,
        oxygen_recycler::plugin,
        metal_trash_pile::plugin,
        toilet::plugin,
        water_dispenser::plugin,
    ));

    app.add_plugins(WorldInspectorPlugin::new());
}
