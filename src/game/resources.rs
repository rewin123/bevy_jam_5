use bevy::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.init_resource::<Water>();
    app.init_resource::<BadWater>();
    app.init_resource::<Oxygen>();
    app.init_resource::<OxygenInAir>();
    app.init_resource::<Pee>();
    app.init_resource::<Food>();
    app.init_resource::<Hydrogen>();
    app.init_resource::<Electricity>();
    app.init_resource::<CarbonInAir>();
    app.init_resource::<MetalTrash>();
    app.init_resource::<Metal>();

    #[cfg(feature = "dev")]
    app.add_plugins(dev::plugin);
}

#[derive(Resource, Default)]
pub struct Metal(pub f32);

#[derive(Resource, Default)]
pub struct MetalTrash(pub f32);

#[derive(Resource, Default)]
pub struct Water(pub f32);

#[derive(Resource, Default)]
pub struct BadWater(pub f32);

/// Oxygen in ship
#[derive(Resource, Default)]
pub struct Oxygen(pub f32);


/// Oxygen in ship air
#[derive(Resource, Default)]
pub struct OxygenInAir(pub f32); //in percent

/// How many carbon is in the air. If its too many, then you will die
#[derive(Resource, Default)]
pub struct CarbonInAir(pub f32);

#[derive(Resource, Default)]
pub struct Pee(pub f32);

#[derive(Resource, Default)]
pub struct Food(pub f32);

#[derive(Resource, Default)]
pub struct Hydrogen(pub f32);

#[derive(Resource, Default)]
pub struct Electricity {
    pub total: f32, //how many electricity can be used. If used > total, then all electricity generators will be shut off
    pub used : f32 //how many electricity was used in last tick
}


#[cfg(feature = "dev")]
mod dev {
    use bevy::prelude::*;
    use crate::dev_tools::DebugPanel;

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
        oxygen_in_air: Res<OxygenInAir>,
        pee: Res<Pee>,
        food: Res<Food>,
        hydrogen: Res<Hydrogen>,
        electricity: Res<Electricity>,
        carbon_in_air: Res<CarbonInAir>,
    ) {

        debug_panel.add("Metal", format!("Metal: {}", metal.0));
        debug_panel.add("Metal trash", format!("Metal trash: {}", metal_trash.0));
        debug_panel.add("Water", format!("Water: {}", water.0));
        debug_panel.add("Bad water", format!("Bad water: {}", bad_water.0));
        debug_panel.add("Oxygen", format!("Oxygen: {}", oxygen.0));
        debug_panel.add("Oxygen in air", format!("Oxygen in air: {}", oxygen_in_air.0));
        debug_panel.add("Pee", format!("Pee: {}", pee.0));
        debug_panel.add("Food", format!("Food: {}", food.0));
        debug_panel.add("Hydrogen", format!("Hydrogen: {}", hydrogen.0));
        debug_panel.add("Electricity", format!("Electricity: {}", electricity.total - electricity.used));
        debug_panel.add("Carbon in air", format!("Carbon in air: {}", carbon_in_air.0));
    }
}