use bevy::prelude::*;
use rand::Rng;
use rand_distr::{Distribution, Normal};

use crate::GameState;

use self::units::{
    presets::KnightBundle,
    spawn::{UnitSpawn, UnitSpawnBundle},
    Formation, Team,
};

mod camera;
pub mod units;

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(units::UnitsPlugin)
            .add_systems(OnEnter(GameState::Battle), setup)
            .add_systems(
                Update,
                (camera::calc_bounds, camera::move_camera)
                    .chain()
                    .run_if(in_state(GameState::Battle)),
            );
    }
}

fn setup(mut commands: Commands) {
    let normal = Normal::new(100.0, 2.0).unwrap();
    let mut rng = rand::thread_rng();

    let count_a = normal.sample(&mut rng) as usize;
    let count_b = normal.sample(&mut rng) as usize;

    let (count_large, count_small) = match count_a.cmp(&count_b) {
        std::cmp::Ordering::Greater => (count_a, count_b),
        std::cmp::Ordering::Equal => (count_a + 1, count_b),
        std::cmp::Ordering::Less => (count_b, count_a),
    };

    info!("Player: {}, Enemy: {}", count_large, count_small);

    commands.spawn(UnitSpawnBundle {
        spawn: UnitSpawn {
            formation: rand_formation(),
            team: Team::Player,
            unit: KnightBundle::default(),
            unit_count: count_large,
            unit_size: Vec2::splat(10.0),
            ..default()
        },
        transform: TransformBundle {
            local: Transform::from_xyz(-100.0, 0.0, 0.0),
            ..default()
        },
    });

    commands.spawn(UnitSpawnBundle {
        spawn: UnitSpawn {
            formation: rand_formation(),
            team: Team::Enemy,
            unit: KnightBundle::default(),
            unit_count: count_small,
            unit_size: Vec2::splat(10.0),
            ..default()
        },
        transform: TransformBundle {
            local: Transform::from_xyz(100.0, 0.0, 0.0),
            ..default()
        },
    });
}

fn rand_formation() -> Formation {
    let mut rng = rand::thread_rng();

    match rng.gen_range(0..=1) {
        0 => Formation::Column,
        _ => Formation::Box,
    }
}
