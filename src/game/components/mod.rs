pub mod earth;
pub mod fire;
pub mod pc;

pub(crate) fn plugin(app: &mut bevy::prelude::App) {
    app.add_plugins((earth::plugin, fire::plugin, pc::plugin));
}
