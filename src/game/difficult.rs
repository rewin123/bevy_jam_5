//! This file contains all major constans for game difficulty calculated from idea that distance between events must have known mean time distance

pub const EVENT_DISTANCE: f32 = 2.0; //some action nead to do every 2.5 second
pub const EVENT_DURATION: f32 = 2.0; //duration for each action (approximatly)

/// How many non work events per game loop
pub const REQUIREMENTS_EVENTS: f32 =
        1.0 // Oxygen on
        + 1.0 // Oxygen off

        + 1.0 // Hydroponic refill
        + 1.0 // Hydroponic gather food

        + 1.0 // BAD WATER RECYCLE

        + EATS_PER_LOOP // Eat
        + DRINK_PER_LOOP // Drink
        + TOILET_PER_LOOP // Toilet

        + FIRE_PER_CYCLE // Fire
;

pub const EVENTS_IN_LOOP: f32 = REQUIREMENTS_EVENTS; // Oxygen on/Oxygen off/Hydroponic refill/Water recycle/Food eating/Toilet/Fire/Work (Work is x2 to any another event)

pub const EVENT_LOOP_DURATION: f32 = EVENTS_IN_LOOP * (EVENT_DISTANCE + EVENT_DURATION);

pub const RES_LIMIT: f32 = 100.0;

/// Oxygen
/// We must spend 60 percent of all oxygen during EVENT_LOOP_DURATION seconds
/// (RES_LIMIT * 0.6 - EVENT_LOOP_DURATION * BREATH_RATE + HYDROPONIC_UP_TIME * HYDROPONIC_OXYGEN_RATE) must be zero
pub const BREATH_RATE: f32 = 1.0;
pub const FIRE_DURATION: f32 = 3.0; // How many fire will be alive before it will be destroyed by player
pub const FIRE_PER_CYCLE: f32 = 2.0;
pub const FIRE_MEAN_PERIOD: f32 = EVENT_LOOP_DURATION / FIRE_PER_CYCLE;

pub const FIRE_RATE: f32 = (RES_LIMIT * 0.6 - EVENT_LOOP_DURATION * BREATH_RATE
    + HYDROPONIC_UP_TIME * HYDROPONIC_OXYGEN_RATE)
    / FIRE_DURATION
    / FIRE_PER_CYCLE;

pub const OXYGEN_REGENRATE_SPEED: f32 = RES_LIMIT * 0.6 / OXYGEN_REGENARATE_TIME;
pub const OXYGEN_REGENARATE_TIME: f32 = 5.0;

/// Hydroponic
/// We will harvest food once per EVENT_LOOP_DURATION
pub const HYDROPONIC_OXYGEN_RATE: f32 = 0.5;
pub const HYDROPONIC_UP_TIME: f32 = EVENT_LOOP_DURATION - EVENT_DURATION * 2.0;
pub const HYDROPONIC_TIME_TO_FOOD: f32 = EVENT_LOOP_DURATION;
pub const HYDROPONIC_WATER_MAX: f32 = 10.0;
pub const HYDROPONIC_WATER_RATE: f32 = HYDROPONIC_WATER_MAX / EVENT_LOOP_DURATION;
pub const HYDROPONIC_FOOD_PER_HARVEST: f32 = RACION_SIZE * EATS_PER_LOOP;
pub const WATER_TO_FOOD_RATIO: f32 =
    (HYDROPONIC_WATER_RATE * HYDROPONIC_TIME_TO_FOOD * 0.5) / HYDROPONIC_FOOD_PER_HARVEST;

/// Eating
pub const EATS_PER_LOOP: f32 = 2.0;
pub const HUNGRY_TIMEOUT: f32 = EVENT_LOOP_DURATION / EATS_PER_LOOP;
pub const HUNGRY_RATE: f32 = RES_LIMIT / HUNGRY_TIMEOUT;
pub const RACION_SIZE: f32 = 1.0;

/// Thirst
pub const DRINK_PER_LOOP: f32 = 3.0;
pub const THIRST_TIMEOUT: f32 = EVENT_LOOP_DURATION / DRINK_PER_LOOP;
pub const THIRST_RATE: f32 = RES_LIMIT / THIRST_TIMEOUT;
pub const DRINK_AMOUNT: f32 = 10.0;

/// Toilet
pub const PEE_RATE: f32 = (WATER_TO_FOOD_RATIO * RACION_SIZE * EATS_PER_LOOP
    + DRINK_AMOUNT * DRINK_PER_LOOP)
    / EVENT_LOOP_DURATION;
pub const TOILET_PER_LOOP: f32 = 1.0;
pub const TOILET_TIMEOUT: f32 = EVENT_LOOP_DURATION / TOILET_PER_LOOP;
pub const TOILET_K: f32 = RES_LIMIT / PEE_RATE / (TOILET_TIMEOUT * TOILET_TIMEOUT);
