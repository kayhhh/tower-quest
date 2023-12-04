use bevy::prelude::*;

#[derive(Component, Default)]
pub enum UnitAI {
    #[default]
    Melee,
    Ranged,
}
