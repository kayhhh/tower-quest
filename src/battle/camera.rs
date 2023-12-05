use bevy::{prelude::*, render::primitives::Frustum};

use super::units::{ai::Dead, UnitSprite};

#[derive(Component, Default)]
pub struct TargetBounds {
    pub min: Vec2,
    pub max: Vec2,
}

/// Calculates the bounds of the camera target
/// Keeps all units in view
pub fn calc_bounds(
    mut commands: Commands,
    mut camera: Query<(Entity, &Transform), With<Camera>>,
    mut units: Query<&Transform, (With<UnitSprite>, Without<Dead>)>,
) {
    let camera = camera.single_mut();

    let mut min = Vec2::new(f32::MAX, f32::MAX);
    let mut max = Vec2::new(f32::MIN, f32::MIN);

    for unit in units.iter_mut() {
        // Relative to camera
        let pos = unit.translation.truncate() - camera.1.translation.truncate();

        min = min.min(pos);
        max = max.max(pos);
    }

    // If there are no units, don't do anything
    if min == Vec2::new(f32::MAX, f32::MAX) {
        return;
    }

    commands.entity(camera.0).insert(TargetBounds { min, max });
}

#[derive(Component)]
pub struct CameraTarget(Transform);

#[derive(Component, Default)]
pub struct CameraVelocity(Vec3);

const MOVE_SPEED: f32 = 10.0;
const PADDING: f32 = 100.0;
const ZOOM_SPEED: f32 = 0.1;

pub fn set_camera_velocity(
    time: Res<Time>,
    mut camera: Query<(&Frustum, &TargetBounds, &Transform, &mut CameraVelocity), With<Camera>>,
) {
    for (frustum, bounds, transform, mut velocity) in camera.iter_mut() {
        let current = transform.translation.truncate();

        // Find a target transform + zoom that covers the target bounds
        let left = frustum.half_spaces[0].d() + bounds.min.x;
        let right = frustum.half_spaces[1].d() - bounds.max.x;
        let bottom = frustum.half_spaces[2].d() + bounds.min.y;
        let top = frustum.half_spaces[3].d() - bounds.max.y;

        // info!(
        //     "left: {}, right: {}, bottom: {}, top: {}",
        //     left, right, bottom, top
        // );

        // If we have extra space on every side, zoom in
        // Otherwise, zoom out
        let min = left.min(right).min(bottom).min(top) - PADDING;
        let speed = ZOOM_SPEED * min.abs() / PADDING;

        if min > 0.0 {
            velocity.0.z -= time.delta_seconds() * speed;
        } else {
            velocity.0.z += time.delta_seconds() * speed;
        }

        // Move to center of bounds
        let center = bounds.min + (bounds.max - bounds.min) / 2.0;
        let target = center + transform.translation.truncate();

        if current == target {
            continue;
        }

        let dir = target - current;
        let dir = dir.normalize();
        let acc = dir * time.delta_seconds() * MOVE_SPEED;

        velocity.0.x += acc.x;
        velocity.0.y += acc.y;
    }
}

pub fn apply_camera_velocity(
    time: Res<Time>,
    mut camera: Query<(&mut Transform, &mut CameraVelocity), With<Camera>>,
) {
    for (mut transform, mut velocity) in camera.iter_mut() {
        let vel = velocity.0 * time.delta_seconds();

        transform.translation.x += vel.x;
        transform.translation.y += vel.y;
        transform.scale += vel.z;

        // Dampen velocity
        velocity.0 -= vel;
    }
}
