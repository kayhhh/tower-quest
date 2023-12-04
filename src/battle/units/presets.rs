use bevy::prelude::*;

use super::{
    ai::{AttackCooldown, AttackDamage, AttackRange, MovementSpeed},
    Health, UnitSprite,
};

#[derive(Bundle, Clone)]
pub struct KnightBundle {
    pub attack_speed: AttackCooldown,
    pub damage: AttackDamage,
    pub health: Health,
    pub movement_speed: MovementSpeed,
    pub range: AttackRange,
    pub sprite: UnitSprite,
}

impl Default for KnightBundle {
    fn default() -> Self {
        Self {
            attack_speed: AttackCooldown(1.0),
            damage: AttackDamage(10.0),
            health: Health(100.0),
            movement_speed: MovementSpeed(15.0),
            range: AttackRange(5.0),
            sprite: UnitSprite::Knight,
        }
    }
}

#[derive(Bundle, Clone)]
pub struct ArcherBundle {
    pub attack_speed: AttackCooldown,
    pub damage: AttackDamage,
    pub health: Health,
    pub range: AttackRange,
    pub speed: MovementSpeed,
    pub sprite: UnitSprite,
}

impl Default for ArcherBundle {
    fn default() -> Self {
        Self {
            attack_speed: AttackCooldown(2.0),
            damage: AttackDamage(5.0),
            health: Health(50.0),
            range: AttackRange(50.0),
            speed: MovementSpeed(10.0),
            sprite: UnitSprite::Archer,
        }
    }
}
