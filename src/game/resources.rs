use bevy::prelude::*;
use bevy_quill::Cx;

use crate::screen::Screen;

use super::{
    daycycle::{GameTime, PlayerDied, PlayerState, TimeSpeed}, difficult::OXYGEN_REGENRATE_SPEED, ui::components::resource_slider::ResourceSlider
};

pub(crate) fn plugin(app: &mut App) {
    app.init_resource::<AllResourcesGetter>();
    app.init_resource::<OxygenRecycling>();
    app.init_resource::<FoodGeneration>();
    app.init_resource::<Electricity>();
    app.init_resource::<Temperature>();
    app.init_resource::<Hungry>();
    app.init_resource::<Toilet>();

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
        GameResourcePlugin::<Thirst>::default(),
        GameResourcePlugin::<Hungry>::default(),
        GameResourcePlugin::<Toilet>::default(),
    ));

    #[cfg(feature = "dev")]
    app.add_plugins(dev::plugin);
}

macro_rules! impl_limitless_resource {
    ($name:ident) => {
        #[derive(Resource, Default, Clone, Copy)]
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

            fn resource_threshold(&self) -> ResourceThreshold {
                ResourceThreshold::Limitless
            }

            fn warning_thresholds(&self) -> (Option<f32>, Option<f32>) {
                (None, None)
            }

            fn label(&self) -> String {
                stringify!($name).to_string()
            }

            #[doc = "Decreases the amount by the given amount until 0."]
            fn decrease(&mut self, decrease_amount: f32) {
                self.set_amount((self.amount - decrease_amount).max(0.0))
            }
            #[doc = "Increases the amount byt the given amount. Doesn't have a max value"]
            fn increase(&mut self, increase_amount: f32) {
                self.set_amount((self.amount + increase_amount).max(0.0))
            }

            fn death_reason(&self, _is_deficiency: bool) -> Option<String> {
                None
            }
        }
    };
}

macro_rules! simple_game_resource {
    ($name:ident, $initial_amount:literal, $limit:literal, $min:expr, $max:expr, $resource_type:expr, $excess_reason:expr) => {
        simple_game_resource!(
            $name,
            $initial_amount,
            $limit,
            $min,
            $max,
            $resource_type,
            "",
            $excess_reason
        );
    };
    ($name:ident, $initial_amount:literal, $limit:literal, $min:expr, $max:expr, $resource_type:expr, $deficiency_reason:expr, $excess_reason:expr) => {
        #[derive(Resource, Clone, Copy)]
        pub struct $name {
            amount: f32,
            limit: f32,
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    amount: $initial_amount,
                    limit: $limit,
                }
            }
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

            fn warning_thresholds(&self) -> (Option<f32>, Option<f32>) {
                ($min, $max)
            }

            fn resource_threshold(&self) -> ResourceThreshold {
                $resource_type
            }

            fn label(&self) -> String {
                stringify!($name).to_string()
            }

            fn decrease(&mut self, decrease_amount: f32) {
                self.set_amount((self.amount - decrease_amount).clamp(0.0, self.limit))
            }

            fn increase(&mut self, increase_amount: f32) {
                self.set_amount((self.amount + increase_amount).clamp(0.0, self.limit))
            }

            fn death_reason(&self, is_deficiency: bool) -> Option<String> {
                if is_deficiency {
                    Some($deficiency_reason.to_string())
                } else {
                    Some($excess_reason.to_string())
                }
            }
        }
    };
}

// Now let's redefine our resources with optional deficiency reasons
simple_game_resource!(
    Water,
    90.0,
    100.0,
    Some(10.0),
    Some(90.0),
    ResourceThreshold::Waste,
    "You couldn't pay your water bill and dried up like your dreams of early mortgage repayment.",
    "You drowned in debt and water simultaneously. At least your mortgage is now waterproof!"
);

simple_game_resource!(
    Food,
    5.0,
    100.0,
    None,
    None,
    ResourceThreshold::Limitless,
    "You ate your last instant noodle. Now you're bankrupt and in heaven, where mortgage is just a myth.",
    "You burst from overeating. Too bad your mortgage didn't burst with you."
);

simple_game_resource!(
    Oxygen,
    50.0,
    100.0,
    Some(20.0),
    Some(80.0),
    ResourceThreshold::HealthyRange,
    "You suffocated trying to save on oxygen tanks to pay the mortgage. To breathe or to pay - that is the question!",
    "You exploded from excess oxygen. Your mortgage also bubbled up, but alas, didn't pop."
);

simple_game_resource!(
    Hydrogen,
    50.0,
    100.0,
    None,
    None,
    ResourceThreshold::HealthyRange,
    "Your hydrogen engine stalled. Now you're drifting in space, like your mortgage in a sea of debt.",
    "Boom! You turned into a small sun. The mortgage bank is already billing your relatives for light pollution."
);

simple_game_resource!(
    Pee,
    0.0,
    100.0,
    None,
    Some(90.0),
    ResourceThreshold::Limitless,
    ""
);

simple_game_resource!(
    Toilet,
    0.0,
    100.0,
    None,
    Some(90.0),
    ResourceThreshold::Waste,
    "You drowned in your own urine, trying to save on sewage to pay the mortgage. The golden shower turned into a golden cage!"
);

simple_game_resource!(
    Thirst,
    10.0,
    100.0,
    None,
    Some(80.0),
    ResourceThreshold::Waste,
    "You died of thirst. Your last thought was about the mortgage, not water.",
    "You died of thirst, refusing to drink anything but elite champagne. Your mortgage remained unpaid, just like your thirst."
);

simple_game_resource!(
    BadWater,
    0.0,
    100.0,
    None,
    Some(90.0),
    ResourceThreshold::Waste,
    "You drowned in an ocean of poor quality water. Your mortgage broker is already selling tickets for tours to the new toxic lake."
);

simple_game_resource!(
    CarbonDioxide,
    0.0,
    100.0,
    None,
    None,
    ResourceThreshold::Waste,
    "You suffocated in carbon dioxide. Your last breath was used to inflate a balloon saying 'For Sale: Almost Paid Off Mortgage'."
);

simple_game_resource!(
    Hungry,
    50.0,
    100.0,
    None,
    Some(80.0),
    ResourceThreshold::Waste,
    "You died of starvation. Your last thought was about the mortgage, not food."
);




impl_limitless_resource!(MetalTrash);
impl_limitless_resource!(Metal);


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
            oxygen_generation_rate: OXYGEN_REGENRATE_SPEED,
            // While on, consumes a bit more than is generated
            co2_consumption_rate: OXYGEN_REGENRATE_SPEED,
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

impl<T: GameResource + Clone + Default> Plugin for GameResourcePlugin<T> {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameResInfo::<T>::new());
        app.init_resource::<T>();
        app.add_event::<Generate<T>>();
        app.add_systems(PostUpdate, collect_generations::<T>);
        app.add_systems(PostUpdate, check_death_conditions::<T>);

        app.world_mut()
            .resource_mut::<AllResourcesGetter>()
            .res_plugin
            .push(Box::new(|cx| {
                let val = cx.use_resource::<T>();
                let info = cx.use_resource::<GameResInfo<T>>();
                let (min_threshold, max_threshold) = val.warning_thresholds();
                ResourceSlider {
                    limit: val.limit().unwrap_or(100.0),
                    amount: val.amount(),
                    label: format!("{} {:+.0}", val.label(), info.generation_rate),
                    style: bevy_mod_stylebuilder::StyleHandle::default(),
                    min_threshold,
                    max_threshold,
                    resource_threshold: val.resource_threshold(),
                }
            }));
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ResourceThreshold {
    /// Is good if the value is between the threshold values
    HealthyRange,
    /// Is good if there is at least a min
    Necessity,
    /// Is good if there is less than the max
    Waste,
    /// No need for thresholds
    Limitless,
}

pub trait GameResource: Resource {
    fn amount(&self) -> f32;
    fn set_amount(&mut self, amount: f32);
    fn limit(&self) -> Option<f32>;
    fn warning_thresholds(&self) -> (Option<f32>, Option<f32>);
    fn resource_threshold(&self) -> ResourceThreshold;
    fn label(&self) -> String;
    fn decrease(&mut self, decreate_amount: f32);
    fn increase(&mut self, increase_amount: f32);

    fn death_reason(&self, is_deficiency: bool) -> Option<String>;
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

fn check_death_conditions<T: GameResource + Clone>(
    resource: ResMut<T>,
    mut time_speed: ResMut<TimeSpeed>,
    mut death: EventWriter<PlayerDied>,
    mut next_state: ResMut<NextState<PlayerState>>,
    screen: Res<State<Screen>>,
) {
    if *time_speed == TimeSpeed::Pause {
        return;
    };

    if *screen != Screen::Playing {
        return;
    }

    let amount = resource.amount();
    let (player_died, is_deficiency): (bool, bool) =
        match (resource.resource_threshold(), resource.limit()) {
            (ResourceThreshold::Necessity, _) => (amount <= 0.0, true),
            (ResourceThreshold::Waste, Some(limit)) => (amount >= limit, false),
            (ResourceThreshold::HealthyRange, Some(limit)) if amount >= limit => (true, false),
            (ResourceThreshold::HealthyRange, _) if (amount <= 0.0) => (true, true),
            _ => (false, false),
        };
    // info!("is dead {} {}", player_died, is_deficiency);
    if player_died {
        info!("Die by {}. Amount: {}", resource.label(), amount);
        let death_reason: String = resource
            .death_reason(is_deficiency)
            .unwrap_or("Capitalism Won".to_string());

        death.send(PlayerDied(death_reason));
        *time_speed = TimeSpeed::Pause;
        // This state is unnecesarry now
        // But we can still use it for music I guess
        next_state.set(PlayerState::Dead);
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
    resource.increase(info.generation_rate * time.delta_seconds());
}
