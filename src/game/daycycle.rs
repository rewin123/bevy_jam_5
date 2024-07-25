#![allow(unused)]

use std::time::Duration;

use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};

use super::resources::GameResource;

pub type GameTime = Time<GameTimeContext>;

#[allow(dead_code)]
pub struct GameTimePlugin;

pub(crate) fn plugin(app: &mut App) {
    app.insert_resource(DayDuration(30.0));
    app.insert_resource(DayPassed(0));
    app.insert_resource(TimeSpeed::Pause);
    app.insert_resource(DayState::Day);
    app.insert_resource(GameTime::default());

    app.init_state::<PlayerState>();
    app.add_event::<NightStart>();
    app.add_event::<DayStart>();

    // app.add_systems(PreUpdate, stop_game_on_death);
    app.add_systems(PreUpdate, (time_speed, update_time).chain());
    app.add_systems(PreUpdate, day_events);
    app.add_systems(Update, change_time_speed);

    #[cfg(feature = "dev")]
    app.add_plugins(dev::plugin);
}

#[derive(Resource, Debug, PartialEq, Eq)]
pub enum TimeSpeed {
    Pause,
    Normal,
    Fast,
    Fast2,
    Fast3,
}

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerState {
    #[default]
    Alive,
    Dead,
    Won,
}

// Day duration in seconds
#[derive(Resource)]
pub struct DayDuration(pub f32);

#[derive(Resource)]
pub struct DayPassed(u32);

#[derive(Resource, Reflect, PartialEq, Eq)]
pub enum DayState {
    Night,
    Day,
}

pub enum DeathCause {
    Suffocated,
    TooManyOxigen,
}

#[derive(Event)]
pub struct PlayerDied<T: GameResource>(pub T);

#[derive(Event)]
pub struct NightStart;

#[derive(Event)]
pub struct DayStart;

fn day_events(
    day_duration: Res<DayDuration>,
    mut day_pased: ResMut<DayPassed>,
    gametime: Res<GameTime>,
    mut day_state: ResMut<DayState>,
    //events
    mut night_start: EventWriter<NightStart>,
    mut day_start: EventWriter<DayStart>,
) {
    let day = gametime.elapsed_seconds() / day_duration.0;

    let day_count = day as u32;
    if day_count != day_pased.0 {
        day_pased.0 = day_count;
        day_start.send(DayStart);
    }

    let day_time = day - day_count as f32;
    if day_time > 0.3 && day_time < 0.7 && *day_state != DayState::Day {
        *day_state = DayState::Day;
        night_start.send(NightStart);
    } else if *day_state != DayState::Night {
        *day_state = DayState::Night;
        night_start.send(NightStart);
    }
}

fn time_speed(mut time: ResMut<GameTime>, time_speed: ResMut<TimeSpeed>) {
    match *time_speed {
        TimeSpeed::Pause => time.context_mut().set_relative_speed(0.0),
        TimeSpeed::Normal => time.context_mut().set_relative_speed(1.0),
        TimeSpeed::Fast => time.context_mut().set_relative_speed(2.0),
        TimeSpeed::Fast2 => time.context_mut().set_relative_speed(3.0),
        TimeSpeed::Fast3 => time.context_mut().set_relative_speed(4.0),
    }
}

fn update_time(mut gametime: ResMut<GameTime>, real_time: Res<Time>) {
    let delta = real_time.delta_seconds() * gametime.context().relative_speed;

    gametime.advance_by(Duration::from_secs_f32(delta));
}

fn change_time_speed(
    mut keyboard_input: EventReader<KeyboardInput>,
    mut time_speed: ResMut<TimeSpeed>,
) {
    for key in keyboard_input.read() {
        if key.state != ButtonState::Pressed {
            continue;
        }
        if key.key_code == KeyCode::BracketRight {
            *time_speed = match *time_speed {
                TimeSpeed::Pause => TimeSpeed::Normal,
                TimeSpeed::Normal => TimeSpeed::Fast,
                TimeSpeed::Fast => TimeSpeed::Fast2,
                TimeSpeed::Fast2 => TimeSpeed::Fast3,
                TimeSpeed::Fast3 => TimeSpeed::Pause,
            }
        } else if key.key_code == KeyCode::BracketLeft {
            *time_speed = match *time_speed {
                TimeSpeed::Pause => TimeSpeed::Fast3,
                TimeSpeed::Normal => TimeSpeed::Pause,
                TimeSpeed::Fast => TimeSpeed::Normal,
                TimeSpeed::Fast2 => TimeSpeed::Fast,
                TimeSpeed::Fast3 => TimeSpeed::Fast2,
            }
        }
    }

    keyboard_input.clear();
}

// fn stop_game_on_death(
//     mut death_events: EventReader<PlayerDied>,
//     mut time_speed: ResMut<TimeSpeed>,
//     mut next_state: ResMut<NextState<PlayerState>>,
// ) {
//     for _ in death_events.read() {
//         *time_speed = TimeSpeed::Pause;
//         next_state.set(PlayerState::Dead);
//     }
// }
//
#[derive(Default)]
pub struct GameTimeContext {
    relative_speed: f32,
}

impl GameTimeContext {
    pub fn set_relative_speed(&mut self, speed: f32) {
        self.relative_speed = speed;
    }
}

#[cfg(feature = "dev")]
mod dev {
    use super::*;
    use crate::dev_tools::*;

    pub(crate) fn plugin(app: &mut App) {
        app.add_systems(Update, show_gametime);
    }

    fn show_gametime(
        mut debug_planer: ResMut<DebugPanel>,
        day_passed: Res<DayPassed>,
        gametime: Res<GameTime>,
        time_speed: Res<TimeSpeed>,
    ) {
        debug_planer.add("Day passed", format!("Day: {}", day_passed.0));
        debug_planer.add(
            "Gametime",
            format!("Time: {:.1}", gametime.elapsed_seconds()),
        );

        debug_planer.add("Time speed", format!("Time speed: {:?}", *time_speed));
    }
}
