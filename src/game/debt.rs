use bevy::prelude::*;

use super::{
    daycycle::{GameOver, GameTime, PlayerState},
    difficult::{money_k, second_money_k, SECOND_INCREASE_LEVEL},
    ui::components::debt::{Plot, PlotPoint},
};

#[allow(dead_code)]
pub struct DebtPlugin;

pub(crate) fn plugin(app: &mut App) {
    app.init_resource::<Debt>();
    app.add_systems(PostUpdate, increase_debt);
    app.add_systems(PreUpdate, win_on_zero_debt);
    app.add_systems(PreUpdate, update_plot);

    app.insert_resource(DebtPlot {
        timer: Timer::from_seconds(2.0, TimerMode::Repeating),
    });

    #[cfg(feature = "dev")]
    app.add_plugins(dev::plugin);
}

#[derive(Resource)]
pub struct Debt {
    pub amount: f32,
    pub second_rate: f32,

    pub last_updated: i32,

    pub second_increased: bool,
}

#[derive(Resource)]
pub struct DebtPlot {
    timer: Timer,
}

impl Debt {
    pub fn increase(&mut self) {
        self.amount += self.second_rate * self.amount;
    }
    pub fn reset(&mut self) {

        let second_rate = money_k();

        self.amount = 12500.0;
        self.second_rate = second_rate as f32;
        self.last_updated = 0;
        self.second_increased = false;
    }
}

impl Default for Debt {
    fn default() -> Self {

        let real_rate = money_k();

        Self {
            amount: 12500.0,
            second_rate: real_rate,
            last_updated: 0,
            second_increased: false,
        }
    }
}

fn increase_debt(time: Res<GameTime>, mut debt: ResMut<Debt>) {

    if debt.amount < (SECOND_INCREASE_LEVEL - 1000.0) && !debt.second_increased {
        debt.second_increased = true;
        debt.second_rate = second_money_k();
    }

    if time.elapsed_seconds() - debt.last_updated as f32 > 1.0 {
        debt.increase();
        debt.last_updated = time.elapsed_seconds() as i32;
    }
}

fn win_on_zero_debt(
    debt: Res<Debt>,
    mut player_state: ResMut<NextState<PlayerState>>,
    mut game_over: EventWriter<GameOver>,
) {
    if debt.amount <= 0.0 {
        game_over.send(GameOver::won(
            "You Paid your debt. Now you live in space. Alone, but what a cool view".to_string(),
        ));
        player_state.set(PlayerState::Won);
    }
}

fn update_plot(
    time: Res<GameTime>,
    mut debt_plot: ResMut<DebtPlot>,
    debt: ResMut<Debt>,
    mut plot: ResMut<Plot>,
) {
    if debt_plot.timer.tick(time.delta()).finished() {
        plot.points
            .push(PlotPoint::new(time.elapsed_seconds(), debt.amount));

        if plot.points.len() > 30 {
            plot.points.remove(0);
        }
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
