use bevy::prelude::*;

pub struct DebtPlugin;

pub(crate) fn plugin(app: &mut App) {
    app.init_resource::<Debt>();
}

#[derive(Resource)]
pub struct Debt {
    pub amount: f32,
    pub rate: f32,
}

impl Debt {
    pub fn increase(&mut self) {
        self.amount += self.rate * self.amount;
    }
}

impl Default for Debt {
    fn default() -> Self {
        Debt {
            amount: 13000.0,
            rate: 0.05,
        }
    }
}