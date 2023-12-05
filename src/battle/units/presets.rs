use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

use super::{
    ai::{AttackCooldown, AttackDamage, AttackRange, Health, MovementSpeed, MovementStyle},
    UnitSprite,
};

#[derive(Bundle, Clone)]
pub struct KnightBundle {
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
    pub sprite: UnitSprite,
}

impl Default for KnightBundle {
    fn default() -> Self {
        Self {
            attack_speed: AttackCooldown(1.0),
            collider: Collider::ball(2.5),
            damage: AttackDamage(10.0),
            density: ColliderDensity(1.0),
            health: Health(100.0),
            locked: LockedAxes::ROTATION_LOCKED,
            movement_speed: MovementSpeed(15.0),
            movement_style: MovementStyle::WithinRange,
            range: AttackRange(8.0),
            rigid_body: RigidBody::Dynamic,
            sprite: UnitSprite::Knight,
        }
    }
}

#[derive(Bundle, Clone)]
pub struct ArcherBundle {
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
    pub sprite: UnitSprite,
}

impl Default for ArcherBundle {
    fn default() -> Self {
        Self {
            attack_speed: AttackCooldown(2.0),
            collider: Collider::ball(0.5),
            damage: AttackDamage(5.0),
            density: ColliderDensity(0.75),
            health: Health(50.0),
            locked: LockedAxes::ROTATION_LOCKED,
            movement_speed: MovementSpeed(10.0),
            movement_style: MovementStyle::WithinRange,
            range: AttackRange(50.0),
            rigid_body: RigidBody::Dynamic,
            sprite: UnitSprite::Archer,
        }
    }
}
