use bevy::prelude::*;
use rand::Rng;

use crate::GameState;

use self::units::{
    presets::KnightBundle,
    spawn::{UnitSpawn, UnitSpawnBundle},
    Formation, Team,
};

pub mod camera;
mod enemy;
pub mod units;
mod victory;

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(units::UnitsPlugin)
            .add_systems(Startup, create_initial_units)
            .add_systems(
                Update,
                (
                    (
                        camera::calc_bounds,
                        camera::set_camera_velocity,
                        camera::apply_camera_velocity,
                    )
                        .chain(),
                    victory::detect_victory,
                )
                    .run_if(in_state(GameState::Battle)),
            )
            .add_systems(
                OnExit(GameState::Victory),
                (victory::increase_floor, enemy::upgrade_enemy).chain(),
            );
    }
}

pub const ARENA_HEIGHT: f32 = 200.0;
pub const ARENA_WIDTH: f32 = 600.0;
pub const INITIAL_UNITS: usize = 10;
pub const TEAM_GAP: f32 = 100.0;

pub fn rand_unit_transform(team: &Team) -> Transform {
    let mut rng = rand::thread_rng();

    let x = rng.gen_range((TEAM_GAP / 2.0)..(ARENA_WIDTH / 2.0));
    let y = rng.gen_range(-(ARENA_HEIGHT / 2.0)..(ARENA_HEIGHT / 2.0));

    let x = match team {
        Team::Player => -x,
        Team::Enemy => x,
    };

    Transform::from_xyz(x, y, 0.0)
}

fn create_initial_units(mut commands: Commands) {
    let mut player_transform = rand_unit_transform(&Team::Player);
    player_transform.translation.x = -TEAM_GAP;

    let mut enemy_transform = rand_unit_transform(&Team::Enemy);
    enemy_transform.translation.x = TEAM_GAP;

    commands.spawn(UnitSpawnBundle {
        spawn: UnitSpawn {
            formation: rand_formation(),
            team: Team::Player,
            unit: KnightBundle::default(),
            unit_count: INITIAL_UNITS,
            unit_size: Vec2::splat(10.0),
            ..default()
        },
        transform: TransformBundle {
            local: player_transform,
            ..default()
        },
    });

    commands.spawn(UnitSpawnBundle {
        spawn: UnitSpawn {
            formation: rand_formation(),
            team: Team::Enemy,
            unit: KnightBundle::default(),
            unit_count: INITIAL_UNITS / 2,
            unit_size: Vec2::splat(10.0),
            ..default()
        },
        transform: TransformBundle {
            local: enemy_transform,
            ..default()
        },
    });
}

fn rand_formation() -> Formation {
    let mut rng = rand::thread_rng();

    match rng.gen_range(0..=1) {
        0 => Formation::Box,
        _ => Formation::Box,
    }
}
