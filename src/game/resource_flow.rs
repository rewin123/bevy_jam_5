#![allow(unused)]

use bevy::prelude::*;

use super::{
    components::fire::InFire,
    daycycle::{DeathCause, GameOver, GameTime, TimeSpeed},
    difficult::{BREATH_RATE, FIRE_RATE, HUNGRY_RATE, THIRST_RATE, TOILET_K},
    resources::{
        CarbonDioxide, Food, FoodGeneration, GameResource, Generate, Hungry, Oxygen,
        OxygenRecycling, Pee, Thirst, Toilet, Water,
    },
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            update_oxygen_and_co2,
            fire_oxigen,
            update_thirst,
            update_hungry,
            update_toilet,
        ),
    );
    // app.add_systems(PostUpdate, (bad_air_death, too_many_oxigen_death));
}

fn update_hungry(mut hungry: EventWriter<Generate<Hungry>>) {
    hungry.send(Generate::new(HUNGRY_RATE));
}

fn update_oxygen_and_co2(
    oxygen_recycling: ResMut<OxygenRecycling>,
    mut oxygen: EventWriter<Generate<Oxygen>>,
    mut co2: EventWriter<Generate<CarbonDioxide>>,
    gametime: Res<GameTime>,
) {
    let recycling = oxygen_recycling.working;

    // Oxygen
    let oxygen_generation = if recycling {
        oxygen_recycling.oxygen_generation_rate
    } else {
        0.0
    };

    oxygen.send(Generate::new(oxygen_generation));
    co2.send(Generate::new(-oxygen_generation));

    //breate
    oxygen.send(Generate::new(-BREATH_RATE));
    co2.send(Generate::new(BREATH_RATE));
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

fn fire_oxigen(
    mut oxigen: EventWriter<Generate<Oxygen>>,
    mut co2: EventWriter<Generate<CarbonDioxide>>,
    gametime: Res<GameTime>,
    q_in_fire: Query<Entity, With<InFire>>,
) {
    let count = q_in_fire.iter().count();
    if count > 0 {
        let consuming = count as f32 * FIRE_RATE;

        oxigen.send(Generate::new(-consuming));
        co2.send(Generate::new(consuming));
    }
}

fn update_thirst(mut thirst: EventWriter<Generate<Thirst>>) {
    thirst.send(Generate::new(THIRST_RATE));
}

fn update_toilet(mut toilet: EventWriter<Generate<Toilet>>, pee: Res<Pee>) {
    toilet.send(Generate::new(pee.amount() * TOILET_K));
}
