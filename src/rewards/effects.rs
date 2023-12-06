use bevy::prelude::*;

use crate::battle::units::squad::SquadBundle;

#[derive(Component)]
pub enum ItemEffect {
    AddMovementSpeed(f32),
    AddSquad(SquadBundle),
}

#[derive(Resource)]
pub struct SpeedModifier(pub f32);
