use std::time::Duration;

use bevy::{prelude::*, ui::update};

pub type GameTime = Time<GameTimeContext>;

pub struct GameTimePlugin;

pub(crate) fn plugin(app: &mut App) {
    app.insert_resource(DayDuration(30.0));
    app.insert_resource(GameTimeOfDay(0));
    app.insert_resource(TimeSpeed::Pause);
    app.insert_resource(DayState::Day);
    app.insert_resource(GameTime::default());

    app.add_event::<NightStart>();
    app.add_event::<DayStart>();



    app.add_systems(PreUpdate, (time_speed, update_time).chain());
    app.add_systems(PreUpdate, day_events);
}


#[derive(Resource)]
pub enum TimeSpeed {
    Pause,
    Normal,
    Fast,
    Fast2,
    Fast3
}


// Day duration in seconds
#[derive(Resource)]
pub struct DayDuration(f32);

#[derive(Resource)]
pub struct GameTimeOfDay(u32);

#[derive(Resource, Reflect, PartialEq)]
pub enum DayState {
    Night,
    Day,
}

#[derive(Event)]
pub struct NightStart;

#[derive(Event)]
pub struct DayStart;

fn day_events(
    mut commands: Commands, 
    day_duration: Res<DayDuration>,
    mut day_pased: ResMut<GameTimeOfDay>, 
    gametime: Res<GameTime>,
    mut day_state: ResMut<DayState>,
//events
    mut night_start: EventWriter<NightStart>,
    mut day_start: EventWriter<DayStart>,
) 
{
    let day = gametime.elapsed_seconds() / day_duration.0;

    let day_count = day as u32;
    if day_count != day_pased.0 {
        day_pased.0 = day_count;
        day_start.send(DayStart);
    }

    let day_time = day - day_count as f32;
    if day_time > 0.3 && day_time < 0.7 {
        if *day_state != DayState::Day {
            *day_state = DayState::Day;
            night_start.send(NightStart);
        }
    } else {
        if *day_state != DayState::Night {
            *day_state = DayState::Night;
            night_start.send(NightStart);
        }
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


#[derive(Default)]
pub struct GameTimeContext {
    relative_speed: f32
}

impl GameTimeContext {
    pub fn set_relative_speed(&mut self, speed: f32) {
        self.relative_speed = speed;
    }
}