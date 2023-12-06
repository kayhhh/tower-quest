use bevy::prelude::*;

use crate::battle::units::{
    presets::{ArcherBundle, KnightBundle},
    spawn::UnitSpawn,
};

#[derive(Component)]
pub enum ItemEffect {
    AddMovementSpeed(f32),
    SpawnArchers(UnitSpawn<ArcherBundle>),
    SpawnKnights(UnitSpawn<KnightBundle>),
}

#[derive(Resource)]
pub struct SpeedModifier(pub f32);
