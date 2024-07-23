use bevy::prelude::*;

use crate::game::daycycle::GameTime;

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(bevy_mod_billboard::prelude::BillboardPlugin);
    app.add_systems(Update, flowing_up);
}

#[derive(Component)]
pub struct FlowUpText {
    pub lifetime: f32,
}

fn flowing_up(
    mut commands: Commands,
    time: Res<GameTime>,
    mut q_text: Query<(Entity, &mut Transform, &mut FlowUpText)>,
) {
    for (e, mut transform, mut text) in q_text.iter_mut() {
        text.lifetime -= time.delta_seconds();
        transform.translation.y += 3.0 * time.delta_seconds();

        if text.lifetime < 0.0 {
            commands.entity(e).despawn_recursive();
        }
    }
}
