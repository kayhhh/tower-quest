use bevy::prelude::*;
use rand::Rng;
use rand_distr::Normal;

use crate::Floor;

use super::{
    rand_formation, rand_unit_transform,
    units::{
        presets::KnightBundle,
        spawn::{UnitSpawn, UnitSpawnBundle},
        Team,
    },
    INITIAL_UNITS,
};

/// Upgrade enemy units for the next battle
pub fn upgrade_enemy(mut commands: Commands, floor: Res<Floor>) {
    let base = (INITIAL_UNITS as f32) * (1.0 + floor.0 as f32 / 10.0) - (INITIAL_UNITS / 2) as f32;

    let mut rng = rand::thread_rng();
    let normal = Normal::new(base, base / 3.0).unwrap();

    commands.spawn(UnitSpawnBundle {
        spawn: UnitSpawn {
            formation: rand_formation(),
            team: Team::Enemy,
            unit: KnightBundle::default(),
            unit_count: rng.sample(normal) as usize,
            unit_size: Vec2::splat(10.0),
            ..default()
        },
        transform: TransformBundle {
            local: rand_unit_transform(Team::Enemy),
            ..default()
        },
    });
}
