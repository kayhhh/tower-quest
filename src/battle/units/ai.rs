use bevy::prelude::*;

#[derive(Component, Clone, Default)]
pub enum UnitAI {
    #[default]
    Melee,
    Ranged,
}
