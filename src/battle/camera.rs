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
    mut camera: Query<Entity, With<Camera>>,
    mut units: Query<&Transform, (With<UnitSprite>, Without<Dead>)>,
) {
    let mut min = Vec2::new(f32::MAX, f32::MAX);
    let mut max = Vec2::new(f32::MIN, f32::MIN);

    for unit in units.iter_mut() {
        let pos = unit.translation.truncate();
        min = min.min(pos);
        max = max.max(pos);
    }

    // If there are no units, don't do anything
    if min == Vec2::new(f32::MAX, f32::MAX) {
        return;
    }

    let camera = camera.single_mut();
    commands.entity(camera).insert(TargetBounds { min, max });
}

#[derive(Component)]
pub struct CameraTarget(Transform);

const MOVE_SPEED: f32 = 1.0;
const PADDING: f32 = 100.0;
const ZOOM_SPEED: f32 = 0.2;

pub fn move_camera(
    time: Res<Time>,
    mut camera: Query<(&Frustum, &TargetBounds, &mut Transform), With<Camera>>,
) {
    for (frustum, bounds, mut transform) in camera.iter_mut() {
        let current = transform.translation.truncate();

        // Find a target transform + zoom that covers the target bounds
        let left = frustum.half_spaces[0].d() + bounds.min.x;
        let right = frustum.half_spaces[1].d() - bounds.max.x;
        let bottom = frustum.half_spaces[2].d() + bounds.min.y;
        let top = frustum.half_spaces[3].d() - bounds.max.y;

        // If we have extra space on every side, zoom in
        // Otherwise, zoom out
        if left > PADDING && right > PADDING && bottom > PADDING && top > PADDING {
            info!("Zooming in");
            transform.scale -= time.delta_seconds() * ZOOM_SPEED;
        } else {
            info!("Zooming out");
            transform.scale += time.delta_seconds() * ZOOM_SPEED;
        }

        // Move to center of bounds
        let target = bounds.min + (bounds.max - bounds.min) / 2.0;

        if current == target {
            continue;
        }

        let new = current + (target - current) * time.delta_seconds() * MOVE_SPEED;

        transform.translation.x = new.x;
        transform.translation.y = new.y;
    }
}
