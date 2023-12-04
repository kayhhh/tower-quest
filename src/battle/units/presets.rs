use bevy::prelude::*;

use super::{
    ai::{AttackRange, MovementSpeed},
    Health, UnitSprite,
};

#[derive(Bundle, Clone)]
pub struct KnightBundle {
    pub sprite: UnitSprite,
    pub health: Health,
    pub range: AttackRange,
    pub speed: MovementSpeed,
}

impl Default for KnightBundle {
    fn default() -> Self {
        Self {
            sprite: UnitSprite::Knight,
            health: Health(100.0),
            range: AttackRange(5.0),
            speed: MovementSpeed(10.0),
        }
    }
}

#[derive(Bundle, Clone)]
pub struct ArcherBundle {
    pub sprite: UnitSprite,
    pub health: Health,
    pub range: AttackRange,
    pub speed: MovementSpeed,
}

impl Default for ArcherBundle {
    fn default() -> Self {
        Self {
            sprite: UnitSprite::Archer,
            health: Health(50.0),
            range: AttackRange(50.0),
            speed: MovementSpeed(5.0),
        }
    }
}
