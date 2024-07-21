use bevy::prelude::*;

use crate::game::daycycle::{DayDuration, GameTime};

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Update, rotate_earth);
}

#[derive(Component)]
pub struct Earth;


fn rotate_earth(
    time: Res<GameTime>,
    mut q_earths: Query<&mut Transform, With<Earth>>,
    day_duration: Res<DayDuration>) 
{
    let Ok(mut earth_transform) = q_earths.get_single_mut() else {return;};
    let freq = 0.5 / day_duration.0;

    let rotate_speed = 2.0 * std::f32::consts::PI * freq;
    earth_transform.rotate_z(time.delta_seconds() * rotate_speed);
}
