use bevy::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.init_resource::<Water>();
    app.init_resource::<BadWater>();
    app.init_resource::<Oxygen>();
    app.init_resource::<OxygenGenerator>();
    app.init_resource::<Pee>();
    app.init_resource::<Food>();
    app.init_resource::<Hydrogen>();
    app.init_resource::<Electricity>();
    app.init_resource::<CarbonDioxide>();
    app.init_resource::<MetalTrash>();
    app.init_resource::<Metal>();
    app.init_resource::<Temperature>();

    #[cfg(feature = "dev")]
    app.add_plugins(dev::plugin);
}

#[derive(Resource, Default)]
pub struct Metal {
    pub amount: f32,
}

#[derive(Resource, Default)]
pub struct MetalTrash {
    pub amount: f32,
}

#[derive(Resource)]
pub struct Water {
    pub amount: f32,
    pub limit: f32,
}

impl Default for Water {
    fn default() -> Self {
        Self {
            amount: 50.0,
            limit: 100.0,
        }
    }
}

#[derive(Resource, Default)]
pub struct BadWater {
    pub amount: f32,
    pub limit: f32,
}

/// Oxygen in ship
#[derive(Resource)]
pub struct Oxygen {
    pub amount: f32,
    pub limit: f32,
    pub consumption_rate: f32,
}

impl Default for Oxygen {
    fn default() -> Self {
        Self {
            amount: 50.0,
            limit: 100.0,
            consumption_rate: 2.0,
        }
    }
}

/// Oxygen in ship air
#[derive(Resource)]
pub struct OxygenGenerator {
    pub oxygen_generation_rate: f32,
    pub co2_consumption_rate: f32,
}

impl Default for OxygenGenerator {
    fn default() -> Self {
        Self {
            oxygen_generation_rate: 1.0,
            co2_consumption_rate: 1.0,
        }
    }
}

/// How many carbon is in the air. If its too many, then you will die
#[derive(Resource)]
pub struct CarbonDioxide {
    pub amount: f32,
    pub limit: f32,
    pub generation_rate: f32,
}

impl Default for CarbonDioxide {
    fn default() -> Self {
        Self {
            amount: 0.0,
            limit: 100.0,
            generation_rate: 1.0,
        }
    }
}

#[derive(Resource, Default)]
pub struct Pee {
    pub amount: f32,
    pub limit: f32,
}

#[derive(Resource, Default)]
pub struct Food {
    pub amount: f32,
    pub limit: f32,
}

#[derive(Resource, Default)]
pub struct Hydrogen {
    pub amount: f32,
    pub limit: f32,
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
        oxygen_generator: Res<OxygenGenerator>,
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
            "Oxygen Generation",
            format!(
                "Oxygen Generation (o / co2): {}/{}",
                oxygen_generator.oxygen_generation_rate, oxygen_generator.co2_consumption_rate,
            ),
        );
        debug_panel.add("Pee", format!("Pee: {}/{}", pee.amount, pee.limit));
        debug_panel.add("Food", format!("Food: {}/{}", food.amount, food.limit));
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
                carbon_in_air.amount, carbon_in_air.limit
            ),
        );
        debug_panel.add(
            "Temperature",
            format!("Temperature: {} ℃", temperature.amount),
        );
    }
}
