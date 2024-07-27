use bevy::prelude::*;
use node_tree::styling::Styling;
use node_tree::tree::{IntoNodeTree, NodeTree};
use node_tree::{div, InsertNodumEntity};

use crate::game::daycycle::GameTime;
use crate::game::debt::Debt;

use super::{hex2color, BACKGROUND_COLOR, BORDER_COLOR, FONT_PATH};

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Update, spawn_debt_ui);
    app.insert_resource(Plot { points: vec![] });

    app.add_systems(Startup, |mut cmds: Commands| {
        cmds.spawn(DebtMarker);
    });
}

#[derive(Resource, PartialEq, Clone, Debug)]
pub struct Plot {
    pub points: Vec<PlotPoint>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct PlotPoint {
    pub x: f32,
    pub y: f32,
}

impl PlotPoint {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Component)]
struct DebtMarker;

fn spawn_debt_ui(
    mut commands: Commands,
    plot: ResMut<Plot>,
    debt: Res<Debt>,
    time: Res<GameTime>,
    q_ui: Query<Entity, With<DebtMarker>>,
    asset_server: Res<AssetServer>,
) {
    let dept_width = 250.0;
    let debt_height = 150.0;

    let plot_shift_left = 10.0;
    let plot_shift = 20.0;

    let mut plot = plot.clone();
    plot.points
        .push(PlotPoint::new(time.elapsed_seconds(), debt.amount));

    let mut parent = div()
        .with_top(Val::Px(0.0))
        .with_right(Val::Px(0.0))
        .with_height(Val::Px(debt_height))
        .with_width(Val::Px(dept_width))
        .with_border(UiRect::all(Val::Px(1.0)))
        .with_background_color(hex2color(BACKGROUND_COLOR))
        .with_border_color(hex2color(BORDER_COLOR))
        .with_position_type(PositionType::Absolute);

    let plot_width = dept_width - plot_shift_left * 2.0;
    let plot_height = debt_height - plot_shift * 3.0;
    let inner_plot = draw_plot_inner(&plot, plot_width, plot_height);

    let style = TextStyle {
        font: asset_server.load(FONT_PATH),
        font_size: 14.0,
        ..default()
    };

    parent = parent.with_child(
        TextBundle::from_section("Your \"happy\" mortgage debt", style.clone())
            .into_node_tree()
            .with_top(Val::Px(0.0))
            .with_width(Val::Px(plot_width))
            .with_height(Val::Px(19.0))
            .with_position_type(PositionType::Absolute)
            .with_align_self(AlignSelf::Center),
    );

    //Draw debt

    parent = parent.with_child(
        TextBundle::from_section(
            format!(
                "Debt: {:.0} (+{:.0}/s)",
                debt.amount,
                debt.amount * debt.second_rate
            ),
            TextStyle {
                font: asset_server.load(FONT_PATH),
                font_size: 24.0,
                color: hex2color("#8c4a4a").lighter(0.4),
            },
        )
        .into_node_tree()
        .with_bottom(Val::Px(0.0))
        .with_left(Val::Px(plot_shift_left))
        .with_width(Val::Px(plot_width))
        .with_height(Val::Px(30.0))
        .with_position_type(PositionType::Absolute),
    );

    parent = parent.with_child(
        inner_plot
            .with_left(Val::Px(plot_shift_left))
            .with_top(Val::Px(plot_shift))
            .with_position_type(PositionType::Absolute),
    );

    let target = q_ui.single();
    commands.add(InsertNodumEntity {
        entity: target,
        nodum: parent,
    });
}

fn draw_plot_inner(plot: &Plot, width: f32, hieght: f32) -> NodeTree {
    let mut display = NodeBundle::default()
        .into_node_tree()
        .with_width(Val::Px(width))
        .with_height(Val::Px(hieght))
        .with_border_color(hex2color(BORDER_COLOR))
        .with_border(UiRect::all(Val::Px(1.0)));

    if plot.points.len() < 2 {
        return display;
    }

    let min_x = plot.points.iter().map(|p| p.x).reduce(f32::min).unwrap();
    let max_x = plot.points.iter().map(|p| p.x).reduce(f32::max).unwrap();
    let min_y = plot.points.iter().map(|p| p.y).reduce(f32::min).unwrap();
    let max_y = plot.points.iter().map(|p| p.y).reduce(f32::max).unwrap() + 0.000001;

    // mid x line
    display = display.with_child(
        div()
            .with_width(Val::Px(width))
            .with_height(Val::Px(1.0))
            .with_top(Val::Px(hieght / 2.0))
            .with_position_type(PositionType::Absolute)
            .with_background_color(hex2color(BORDER_COLOR)),
    );

    // mid y line
    display = display.with_child(
        div()
            .with_width(Val::Px(1.0))
            .with_height(Val::Px(hieght))
            .with_left(Val::Px(width / 2.0))
            .with_position_type(PositionType::Absolute)
            .with_background_color(hex2color(BORDER_COLOR)),
    );

    let bar_width = width / plot.points.len().max(30) as f32;

    for i in 0..plot.points.len() {
        let x = plot.points[i].x;
        let y = plot.points[i].y;

        let bar_end = y / max_y;

        let bar_x = bar_width * i as f32;

        let color = hex2color("#8c4a4a");
        let border_color = color.darker(0.2);

        display = display.with_child(
            div()
                .with_width(Val::Px(bar_width.max(0.0)))
                .with_height(Val::Px(bar_end.max(0.0) * hieght))
                .with_left(Val::Px(bar_x))
                .with_top(Val::Px((1.0 - bar_end).max(0.0) * hieght))
                .with_border(UiRect::all(Val::Px(1.0)))
                .with_border_color(border_color)
                .with_background_color(color)
                .with_position_type(PositionType::Absolute),
        );
    }

    display
}
