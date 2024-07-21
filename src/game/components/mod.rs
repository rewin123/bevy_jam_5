pub mod earth;

pub(crate) fn plugin(app: &mut bevy::prelude::App) {
    app.add_plugins((earth::plugin,));
}