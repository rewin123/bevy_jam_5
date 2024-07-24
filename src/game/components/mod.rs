pub mod earth;
pub mod fire;
pub mod flowup_text;
pub mod metal_trash_pile;
pub mod oxygen_recycler;
pub mod pc;

pub(crate) fn plugin(app: &mut bevy::prelude::App) {
    app.add_plugins((
        earth::plugin,
        fire::plugin,
        pc::plugin,
        flowup_text::plugin,
        oxygen_recycler::plugin,
        metal_trash_pile::plugin,
    ));
}
