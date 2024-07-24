use std::ops::RangeInclusive;

use bevy::prelude::*;
use bevy_quill::Cx;

use super::{daycycle::GameTime, ui::components::resource_slider::ResourceSlider};

pub(crate) fn plugin(app: &mut App) {
    app.init_resource::<AllResourcesGetter>();
    app.init_resource::<OxygenRecycling>();
    app.init_resource::<FoodGeneration>();
    app.init_resource::<Electricity>();
    app.init_resource::<Temperature>();

    app.add_plugins((
        GameResourcePlugin::<Oxygen>::default(),
        GameResourcePlugin::<CarbonDioxide>::default(),
        GameResourcePlugin::<Water>::default(),
        GameResourcePlugin::<BadWater>::default(),
        GameResourcePlugin::<Pee>::default(),
        GameResourcePlugin::<Food>::default(),
        GameResourcePlugin::<Hydrogen>::default(),
        GameResourcePlugin::<Metal>::default(),
        GameResourcePlugin::<MetalTrash>::default(),
    ));

    #[cfg(feature = "dev")]
    app.add_plugins(dev::plugin);
}

#[derive(Resource, Default)]
pub struct AllResourcesGetter {
    pub res_plugin: Vec<Box<dyn Fn(&Cx) -> ResourceSlider + Send + Sync>>,
}

/// Resource for OxygenRecycling configuration
#[derive(Resource)]
pub struct OxygenRecycling {
    pub oxygen_generation_rate: f32,
    pub co2_consumption_rate: f32,
    pub working: bool,
}

impl Default for OxygenRecycling {
    fn default() -> Self {
        Self {
            oxygen_generation_rate: 5.5,
            // While on, consumes a bit more than is generated
            co2_consumption_rate: 5.5,
            working: true,
        }
    }
}

#[derive(Resource)]
pub struct FoodGeneration {
    pub generation_rate: f32,
}

impl Default for FoodGeneration {
    fn default() -> Self {
        Self {
            generation_rate: 1.0,
        }
    }
}

#[derive(Resource, Default)]
pub struct Electricity {
    pub total: f32, //how many electricity can be used. If used > total, then all electricity generators will be shut off
    pub used: f32,  //how many electricity was used in last tick
}

#[derive(Resource, Default)]
pub struct Temperature {
    pub amount: f32,
}

#[cfg(feature = "dev")]
mod dev {
    use crate::dev_tools::DebugPanel;
    use bevy::prelude::*;

    use super::*;

    pub fn plugin(app: &mut App) {
        app.add_systems(Update, debug_panel);
    }

    fn debug_panel(
        mut debug_panel: ResMut<DebugPanel>,
        metal: Res<Metal>,
        metal_trash: Res<MetalTrash>,
        water: Res<Water>,
        bad_water: Res<BadWater>,
        oxygen: Res<Oxygen>,
        oxygen_recycler: Res<OxygenRecycling>,
        pee: Res<Pee>,
        food: Res<Food>,
        hydrogen: Res<Hydrogen>,
        electricity: Res<Electricity>,
        carbon_in_air: Res<CarbonDioxide>,
        temperature: Res<Temperature>,
    ) {
        debug_panel.add("Metal", format!("Metal: {}", metal.amount));
        debug_panel.add(
            "Metal trash",
            format!("Metal trash: {}", metal_trash.amount),
        );
        debug_panel.add("Water", format!("Water: {}/{}", water.amount, water.limit));
        debug_panel.add(
            "Bad water",
            format!("Bad water: {}/{}", bad_water.amount, bad_water.limit),
        );
        debug_panel.add(
            "Oxygen",
            format!("Oxygen: {}/{}", oxygen.amount as i32, oxygen.limit),
        );
        debug_panel.add(
            "Oxygen Recycling",
            format!(
                "Oxygen Recycling (o / co2): {}/{}",
                oxygen_recycler.oxygen_generation_rate, oxygen_recycler.co2_consumption_rate,
            ),
        );
        debug_panel.add("Pee", format!("Pee: {}/{}", pee.amount, pee.limit));
        debug_panel.add(
            "Food",
            format!("Food: {}/{}", food.amount as i32, food.limit),
        );
        debug_panel.add(
            "Hydrogen",
            format!("Hydrogen: {}/{}", hydrogen.amount, hydrogen.limit),
        );
        debug_panel.add(
            "Electricity",
            format!("Electricity: {}/{}", electricity.total, electricity.used),
        );
        debug_panel.add(
            "Carbon in air",
            format!(
                "Carbon in air: {}/{}",
                carbon_in_air.amount as i32, carbon_in_air.limit
            ),
        );
        debug_panel.add(
            "Temperature",
            format!("Temperature: {} ℃", temperature.amount),
        );
    }
}

pub struct GameResourcePlugin<T: GameResource> {
    _type: std::marker::PhantomData<T>,
}

impl<T: GameResource> Default for GameResourcePlugin<T> {
    fn default() -> Self {
        Self {
            _type: std::marker::PhantomData,
        }
    }
}

impl<T: GameResource + Default> Plugin for GameResourcePlugin<T> {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameResInfo::<T>::new());
        app.init_resource::<T>();
        app.add_event::<Generate<T>>();
        app.add_systems(PostUpdate, collect_generations::<T>);

        app.world_mut()
            .resource_mut::<AllResourcesGetter>()
            .res_plugin
            .push(Box::new(|cx| {
                let val = cx.use_resource::<T>();
                let info = cx.use_resource::<GameResInfo<T>>();
                ResourceSlider {
                    limit: val.limit().unwrap_or(100.0),
                    amount: val.amount(),
                    label: format!("{} {:+.0}", val.label(), info.generation_rate),
                    style: bevy_mod_stylebuilder::StyleHandle::default(),
                }
            }));
    }
}

pub trait GameResource: Resource {
    fn amount(&self) -> f32;
    fn set_amount(&mut self, amount: f32);
    fn limit(&self) -> Option<f32>;
    #[allow(dead_code)]
    fn healthly_range(&self) -> Option<RangeInclusive<f32>>;
    fn label(&self) -> String;
}
/// Generation for resource in dval/sec manner
/// Example
/// For increasing water to 1 per second:
/// Generate::<Water>::new(1.0)
#[derive(Event)]
pub struct Generate<T: Resource> {
    pub amount: f32,
    _type: std::marker::PhantomData<T>,
}

impl<T: Resource> Generate<T> {
    pub const fn new(amount: f32) -> Self {
        Self {
            amount,
            _type: std::marker::PhantomData,
        }
    }
}

#[derive(Resource)]
pub struct GameResInfo<T: GameResource> {
    pub generation_rate: f32,
    phantom: std::marker::PhantomData<T>,
}

impl<T: GameResource> GameResInfo<T> {
    pub const fn new() -> Self {
        Self {
            generation_rate: 0.0,
            phantom: std::marker::PhantomData,
        }
    }
}

fn collect_generations<T: GameResource>(
    mut ev_gens: EventReader<Generate<T>>,
    mut info: ResMut<GameResInfo<T>>,
    mut resource: ResMut<T>,
    time: Res<GameTime>,
) {
    info.generation_rate = 0.0;
    for gen in ev_gens.read() {
        info.generation_rate += gen.amount;
    }
    ev_gens.clear();
    let amount = resource.amount() + info.generation_rate * time.delta_seconds();
    resource.set_amount(amount);
}

macro_rules! impl_limitless_resource {
    ($name:ident) => {
        #[derive(Resource, Default)]
        pub struct $name {
            pub amount: f32,
        }

        impl $name {
            pub const fn new(amount: f32) -> Self {
                Self { amount }
            }
        }

        impl GameResource for $name {
            fn amount(&self) -> f32 {
                self.amount
            }

            fn set_amount(&mut self, amount: f32) {
                self.amount = amount;
            }

            fn limit(&self) -> Option<f32> {
                None
            }

            fn healthly_range(&self) -> Option<RangeInclusive<f32>> {
                None
            }

            fn label(&self) -> String {
                stringify!($name).to_string()
            }
        }
    };
}

impl_limitless_resource!(MetalTrash);
impl_limitless_resource!(Metal);

macro_rules! simple_game_resource {
    ($name:ident, $limit:literal, $helthly_min:literal, $helthly_max:literal) => {
        #[derive(Resource, Default)]
        pub struct $name {
            amount: f32,
            limit: f32,
        }

        impl $name {
            pub const fn new(amount: f32, limit: f32) -> Self {
                Self { amount, limit }
            }
        }

        impl GameResource for $name {
            fn amount(&self) -> f32 {
                self.amount
            }

            fn set_amount(&mut self, amount: f32) {
                self.amount = amount;
            }

            fn limit(&self) -> Option<f32> {
                Some(self.limit)
            }

            fn healthly_range(&self) -> Option<RangeInclusive<f32>> {
                Some($helthly_min..=$helthly_max)
            }

            fn label(&self) -> String {
                stringify!($name).to_string()
            }
        }
    };
}

simple_game_resource!(Water, 100.0, 10.0, 90.0);
simple_game_resource!(Food, 100.0, 10.0, 90.0);
simple_game_resource!(Pee, 100.0, 10.0, 90.0);
simple_game_resource!(BadWater, 100.0, 10.0, 90.0);
simple_game_resource!(Hydrogen, 100.0, 10.0, 90.0);
simple_game_resource!(CarbonDioxide, 100.0, 10.0, 90.0);
simple_game_resource!(Oxygen, 100.0, 10.0, 90.0);
