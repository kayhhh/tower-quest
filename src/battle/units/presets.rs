use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

use super::ai::{AttackCooldown, AttackDamage, AttackRange, Health, MovementSpeed, MovementStyle};

#[derive(Bundle, Clone)]
pub struct UnitBundle {
    pub attack_speed: AttackCooldown,
    pub collider: Collider,
    pub damage: AttackDamage,
    pub density: ColliderDensity,
    pub health: Health,
    pub locked: LockedAxes,
    pub movement_speed: MovementSpeed,
    pub movement_style: MovementStyle,
    pub range: AttackRange,
    pub rigid_body: RigidBody,
}

impl UnitBundle {
    pub fn knight() -> Self {
        Self {
            attack_speed: AttackCooldown(1.0),
            collider: Collider::ball(3.0),
            damage: AttackDamage(20.0),
            density: ColliderDensity(1.0),
            health: Health(100.0),
            locked: LockedAxes::ROTATION_LOCKED,
            movement_speed: MovementSpeed(15.0),
            movement_style: MovementStyle::WithinRange,
            range: AttackRange(9.0),
            rigid_body: RigidBody::Dynamic,
        }
    }

    pub fn archer() -> Self {
        Self {
            attack_speed: AttackCooldown(2.0),
            collider: Collider::ball(2.0),
            damage: AttackDamage(5.0),
            density: ColliderDensity(0.75),
            health: Health(50.0),
            locked: LockedAxes::ROTATION_LOCKED,
            movement_speed: MovementSpeed(10.0),
            movement_style: MovementStyle::WithinRange,
            range: AttackRange(50.0),
            rigid_body: RigidBody::Dynamic,
        }
    }
}
