use bevy::prelude::*;

use super::{
    daycycle::GameTime,
    resources::{CarbonDioxide, Oxygen, OxygenGenerator},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update_oxygen_and_co2);
}

fn update_oxygen_and_co2(
    oxygen_generator: ResMut<OxygenGenerator>,
    mut oxygen: ResMut<Oxygen>,
    mut co2: ResMut<CarbonDioxide>,
    gametime: Res<GameTime>,
) {
    let delta_seconds = gametime.delta_seconds();
    let delta_o = oxygen_generator.oxygen_generation_rate - oxygen.consumption_rate;
    let timed_delta_o = delta_o * delta_seconds;
    oxygen.amount += timed_delta_o;

    let delta_co2 = co2.generation_rate - oxygen_generator.co2_consumption_rate;
    let timed_delta_co2 = delta_co2 * delta_seconds;
    co2.amount += timed_delta_co2;
}
