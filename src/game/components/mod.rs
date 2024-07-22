pub mod earth;
pub mod fire;
pub mod pc;
pub mod flowup_text;

pub(crate) fn plugin(app: &mut bevy::prelude::App) {
    app.add_plugins((
        earth::plugin, 
        fire::plugin, 
        pc::plugin,
        flowup_text::plugin
    ));
}
