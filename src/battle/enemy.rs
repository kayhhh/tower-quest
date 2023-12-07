use bevy::prelude::*;
use rand::Rng;
use rand_distr::Normal;

use crate::Floor;

use super::{
    layout::SquadSlot,
    units::{
        formation::rand_formation,
        squad::{Squad, SquadBundle, SquadCount, UnitType},
        Team,
    },
    INITIAL_UNITS,
};

#[derive(Debug)]
enum EnemyUpgrade {
    IncreaseUnitCount { slot: Entity, current_count: usize },
    AddSquad(Entity),
}

struct UpgradeOption {
    weight: usize,
    upgrade: EnemyUpgrade,
}

/// Upgrade enemy units for the next battle
pub fn upgrade_enemy(
    mut commands: Commands,
    floor: Res<Floor>,
    open_slots: Query<(Entity, &Team), (Without<Squad>, With<SquadSlot>)>,
    filled_slots: Query<(Entity, &Team, &SquadCount), (With<Squad>, With<SquadSlot>)>,
) {
    // Create upgrade options
    let mut options = vec![];

    for (ent, team, count) in filled_slots.iter() {
        if *team != Team::Enemy {
            continue;
        }

        options.push(UpgradeOption {
            weight: 2,
            upgrade: EnemyUpgrade::IncreaseUnitCount {
                slot: ent,
                current_count: count.0,
            },
        });
    }

    for (ent, team) in open_slots.iter() {
        if *team != Team::Enemy {
            continue;
        }

        options.push(UpgradeOption {
            weight: 1,
            upgrade: EnemyUpgrade::AddSquad(ent),
        });
    }

    if options.is_empty() {
        warn!("No enemy upgrades available");
        return;
    }

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
        EnemyUpgrade::IncreaseUnitCount {
            slot,
            current_count,
        } => increase_unit_count(&mut commands, floor.0, slot, current_count),
        EnemyUpgrade::AddSquad(slot) => add_sqaud(&mut commands, floor.0, slot),
    }
}

fn increase_unit_count(
    commands: &mut Commands,
    floor: usize,
    slot: &Entity,
    current_count: &usize,
) {
    commands
        .entity(*slot)
        .insert(SquadCount(current_count + rand_unit_count(floor)));
}

fn add_sqaud(commands: &mut Commands, floor: usize, slot: &Entity) {
    commands.entity(*slot).insert(SquadBundle {
        unit: UnitType::Knight,
        count: SquadCount(rand_unit_count(floor)),
        formation: rand_formation(),
        ..default()
    });
}

fn rand_unit_count(floor: usize) -> usize {
    let base = (INITIAL_UNITS as f32) * (1.0 + floor as f32 / 10.0) - (INITIAL_UNITS / 2) as f32;

    let mut rng = rand::thread_rng();
    let normal = Normal::new(base, base / 3.0).unwrap();

    rng.sample(normal) as usize
}
