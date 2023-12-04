use bevy::prelude::*;

use super::{ai::UnitAI, Health, UnitSprite};

#[derive(Bundle, Clone)]
pub struct KnightBundle {
    pub ai: UnitAI,
    pub sprite: UnitSprite,
    pub health: Health,
}

impl Default for KnightBundle {
    fn default() -> Self {
        Self {
            ai: UnitAI::Melee,
            sprite: UnitSprite::Knight,
            health: Health(100.0),
        }
    }
}

#[derive(Bundle, Clone)]
pub struct ArcherBundle {
    pub ai: UnitAI,
    pub sprite: UnitSprite,
    pub health: Health,
}

impl Default for ArcherBundle {
    fn default() -> Self {
        Self {
            ai: UnitAI::Ranged,
            sprite: UnitSprite::Archer,
            health: Health(50.0),
        }
    }
}
