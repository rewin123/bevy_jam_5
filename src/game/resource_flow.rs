use bevy::prelude::*;

use super::{
    daycycle::{DeathCause, GameTime, PlayerDied, TimeSpeed},
    resources::{CarbonDioxide, Food, FoodGeneration, Oxygen, OxygenRecycling},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (update_oxygen_and_co2, update_food));
    app.add_systems(PostUpdate, bad_air_death);
}

fn update_oxygen_and_co2(
    oxygen_recycling: ResMut<OxygenRecycling>,
    mut oxygen: ResMut<Oxygen>,
    mut co2: ResMut<CarbonDioxide>,
    gametime: Res<GameTime>,
) {
    let recycling = oxygen_recycling.working;

    // Oxygen
    let oxygen_generation = if recycling {
        oxygen_recycling.oxygen_generation_rate
    } else {
        0.0
    };
    oxygen.amount = calculate_new_amount(
        oxygen.amount,
        oxygen_generation,
        oxygen.consumption_rate,
        gametime.delta_seconds(),
        oxygen.limit,
    );

    // Carbond Dioxide
    let co2_consumption = if recycling {
        oxygen_recycling.co2_consumption_rate
    } else {
        0.0
    };
    co2.amount = calculate_new_amount(
        co2.amount,
        co2.generation_rate,
        co2_consumption,
        gametime.delta_seconds(),
        co2.limit,
    );
}

fn bad_air_death(
    oxygen: Res<Oxygen>,
    co2: Res<CarbonDioxide>,
    mut death: EventWriter<PlayerDied>,
    time_speed: Res<TimeSpeed>,
) {
    if *time_speed != TimeSpeed::Pause && (oxygen.amount <= 0.0 || co2.amount >= co2.limit) {
        death.send(PlayerDied(DeathCause::Suffocated));
        info!("No more air, o2: {}, co2 {}", oxygen.amount, co2.amount);
    }
}

fn update_food(
    mut food: ResMut<Food>,
    food_generation: Res<FoodGeneration>,
    gametime: Res<GameTime>,
) {
    food.amount = calculate_new_amount(
        food.amount,
        food_generation.generation_rate,
        0.0,
        gametime.delta_seconds(),
        food.limit,
    );
}

fn calculate_new_amount(
    amount: f32,
    generation: f32,
    consumption: f32,
    time_delta: f32,
    limit: f32,
) -> f32 {
    let delta = generation - consumption;
    let timed_delta = delta * time_delta;
    (amount + timed_delta).clamp(0.0, limit)
}
