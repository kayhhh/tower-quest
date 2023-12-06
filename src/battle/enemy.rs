use bevy::prelude::*;
use rand::Rng;
use rand_distr::Normal;

use crate::Floor;

use super::{
    units::{
        formation::rand_formation,
        squad::{SquadBundle, SquadCount, UnitType},
    },
    INITIAL_UNITS,
};

#[derive(Debug)]
enum EnemyUpgrade {
    IncreaseUnitCount,
    AddSquad,
}

struct UpgradeOption {
    weight: usize,
    upgrade: EnemyUpgrade,
}

/// Upgrade enemy units for the next battle
pub fn upgrade_enemy(mut commands: Commands, floor: Res<Floor>) {
    // Create upgrade options
    let options = vec![UpgradeOption {
        weight: 2,
        upgrade: EnemyUpgrade::IncreaseUnitCount,
    }];

    // Create weighted pool of options
    let mut pool = Vec::new();

    for option in options.iter() {
        for _ in 0..option.weight {
            pool.push(&option.upgrade);
        }
    }

    // Pick from pool
    let mut rng = rand::thread_rng();

    let upgrade = match pool.get(rng.gen_range(0..pool.len())) {
        Some(upgrade) => *upgrade,
        None => return,
    };

    info!("Enemy upgrade: {:?}", upgrade);

    match upgrade {
        EnemyUpgrade::IncreaseUnitCount => increase_unit_count(&mut commands, floor.0),
        EnemyUpgrade::AddSquad => add_sqaud(&mut commands, floor.0),
    }
}

fn increase_unit_count(_commands: &mut Commands, _floor: usize) {}

fn add_sqaud(commands: &mut Commands, floor: usize) {
    let base = (INITIAL_UNITS as f32) * (1.0 + floor as f32 / 10.0) - (INITIAL_UNITS / 2) as f32;

    let mut rng = rand::thread_rng();
    let normal = Normal::new(base, base / 3.0).unwrap();

    commands.spawn(SquadBundle {
        unit: UnitType::Knight,
        count: SquadCount(rng.sample(normal) as usize),
        formation: rand_formation(),
        ..default()
    });
}
