use bevy::prelude::*;

use super::daycycle::GameTime;

#[allow(dead_code)]
pub struct DebtPlugin;

pub(crate) fn plugin(app: &mut App) {
    app.init_resource::<Debt>();
    app.add_systems(PostUpdate, increase_debt);

    #[cfg(feature = "dev")]
    app.add_plugins(dev::plugin);
}

#[derive(Resource)]
pub struct Debt {
    pub amount: f32,
    #[allow(dead_code)]
    pub day_rate: f32,
    pub second_rate: f32,

    pub last_updated: i32,
}

impl Debt {
    pub fn increase(&mut self) {
        self.amount += self.second_rate * self.amount;
    }
}

impl Default for Debt {
    fn default() -> Self {
        let day_rate = 0.05;
        let day_duration = 30.0;

        let second_rate = (1.0f64 + day_rate).powf(1.0 / day_duration) - 1.0;

        Self {
            amount: 13000.0,
            day_rate: 0.05,
            second_rate: second_rate as f32,
            last_updated: 0,
        }
    }
}

fn increase_debt(time: Res<GameTime>, mut debt: ResMut<Debt>) {
    if time.elapsed_seconds() - debt.last_updated as f32 > 1.0 {
        debt.increase();
        debt.last_updated = time.elapsed_seconds() as i32;
    }
}

#[cfg(feature = "dev")]
mod dev {
    use crate::dev_tools::DebugPanel;
    use bevy::prelude::*;

    use super::*;

    pub(crate) fn plugin(app: &mut App) {
        app.add_systems(Update, show_debt);
    }

    fn show_debt(mut debug_planer: ResMut<DebugPanel>, debt: Res<Debt>) {
        debug_planer.add("Debt", format!("Debt: {:.1}", debt.amount));
    }
}
