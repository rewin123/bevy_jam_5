//! Handle player input and translate it into movement.
//! Note that the approach used here is simple for demonstration purposes.
//! If you want to move the player in a smoother way,
//! consider using a [fixed timestep](https://github.com/bevyengine/bevy/blob/latest/examples/movement/physics_in_fixed_timestep.rs).

use bevy::{prelude::*, window::PrimaryWindow};

use crate::AppSet;

use super::daycycle::GameTime;

pub(super) fn plugin(app: &mut App) {
    // Record directional input as movement controls.
    app.register_type::<MovementController>();
    // app.add_systems(
    //     Update,
    //     record_movement_controller.in_set(AppSet::RecordInput),
    // );

    // Apply movement based on controls.
    app.register_type::<(Movement, WrapWithinWindow)>();
    app.add_systems(
        Update,
        (apply_movement, wrap_within_window)
            .chain()
            .in_set(AppSet::Update),
    );
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MovementController(pub Vec2);

fn record_movement_controller(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController>,
) {
    // Collect directional input.
    let mut intent = Vec2::ZERO;
    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        intent.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        intent.y -= 1.0;
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        intent.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        intent.x += 1.0;
    }

    // Normalize so that diagonal movement has the same speed as
    // horizontal and vertical movement.
    let intent = intent.normalize_or_zero();

    // Apply movement intent to controllers.
    for mut controller in &mut controller_query {
        controller.0 = intent;
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Movement {
    pub speed: f32,
}

fn apply_movement(
    time: Res<GameTime>,
    mut movement_query: Query<(&MovementController, &Movement, &mut Transform)>,
    q_camera: Query<&Transform, (With<Camera>, Without<Movement>)>,
) {
    let Ok(cam_transform) = q_camera.get_single() else {
        return;
    };

    for (controller, movement, mut transform) in &mut movement_query {
        let velocity = movement.speed * controller.0;

        let cam_frw = cam_transform.forward();
        let up = Vec3::new(cam_frw.x, 0.0, cam_frw.z).normalize_or_zero();

        let cam_right = cam_transform.right();
        let right = Vec3::new(cam_right.x, 0.0, cam_right.z).normalize_or_zero();

        let velocity = right * velocity.x + up * velocity.y;

        transform.translation += velocity * time.delta_seconds();
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct WrapWithinWindow;

fn wrap_within_window(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut wrap_query: Query<&mut Transform, With<WrapWithinWindow>>,
) {
    let size = window_query.single().size() + 256.0;
    let half_size = size / 2.0;
    for mut transform in &mut wrap_query {
        let position = transform.translation.xy();
        let wrapped = (position + half_size).rem_euclid(size) - half_size;
        transform.translation = wrapped.extend(transform.translation.z);
    }
}
