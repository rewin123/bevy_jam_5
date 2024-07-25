#![allow(unused)]

use bevy::prelude::*;

use super::{
    components::fire::InFire,
    daycycle::{DeathCause, GameTime, PlayerDied, TimeSpeed},
    resources::{
        CarbonDioxide, Food, FoodGeneration, GameResource, Generate, Oxygen, OxygenRecycling, Pee,
        Thirst, Water,
    },
};

pub(super) fn plugin(app: &mut App) {
    
    
    
    app.add_systems(
        Update,
        (
            update_oxygen_and_co2,
            update_food,
            fire_oxigen,
            update_thirst,
        ),
    );
    // app.add_systems(PostUpdate, (bad_air_death, too_many_oxigen_death));
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
    oxygen.send(Generate::new(-1.0));
    co2.send(Generate::new(1.0));
}

// fn bad_air_death(
//     oxygen: Res<Oxygen>,
//     co2: Res<CarbonDioxide>,
//     mut death: EventWriter<PlayerDied>,
//     time_speed: Res<TimeSpeed>,
// ) {
//     if *time_speed != TimeSpeed::Pause
//         && (oxygen.amount() <= 0.0 || co2.amount() >= co2.limit().unwrap_or_default())
//     {
//         death.send(PlayerDied(DeathCause::Suffocated));
//         info!("No more air, o2: {}, co2 {}", oxygen.amount(), co2.amount());
//     }
// }
//
// fn too_many_oxigen_death(
//     oxygen: Res<Oxygen>,
//     mut death: EventWriter<PlayerDied>,
//     time_speed: Res<TimeSpeed>,
// ) {
//     if *time_speed != TimeSpeed::Pause && oxygen.amount() >= oxygen.limit().unwrap_or_default() {
//         death.send(PlayerDied(DeathCause::TooManyOxigen));
//     }
// }
//
fn update_food(
    mut food: EventWriter<Generate<Food>>,
    food_generation: Res<FoodGeneration>,
    gametime: Res<GameTime>,
) {
    food.send(Generate::new(food_generation.generation_rate));
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
        let consuming = count as f32 * 3.0;

        oxigen.send(Generate::new(-consuming));
        co2.send(Generate::new(consuming));
    }
}

fn update_thirst(mut thirst: EventWriter<Generate<Thirst>>) {
    thirst.send(Generate::new(2.0));
}
