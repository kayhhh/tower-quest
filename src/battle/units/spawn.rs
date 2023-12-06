use bevy::prelude::*;

use super::{formation::Formation, Team, UnitSprite, UnitSprites};

#[derive(Component, Clone, Default)]
pub struct UnitSpawn<T: Bundle + Default> {
    pub unit: T,
    pub team: Team,
    pub formation: Formation,
    /// Number of units to spawn
    pub unit_count: usize,
    /// Size of each unit
    pub unit_size: Vec2,
    /// Whether the units have been spawned yet
    pub spawned: bool,
}

#[derive(Bundle, Default)]
pub struct UnitSpawnBundle<T: Bundle + Default> {
    pub spawn: UnitSpawn<T>,
    pub transform: TransformBundle,
}

pub fn reset_spawns<T: Bundle + Default>(mut spawns: Query<&mut UnitSpawn<T>>) {
    for mut spawn in &mut spawns.iter_mut() {
        spawn.spawned = false;
    }
}
