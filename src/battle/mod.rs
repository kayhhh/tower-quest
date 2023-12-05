use bevy::prelude::*;
use rand::Rng;
use rand_distr::{Distribution, Normal};

use crate::{zoom::Zoom, GameState};

use self::units::{
    presets::KnightBundle,
    spawn::{UnitSpawn, UnitSpawnBundle},
    Formation, Team, UnitSprite,
};

pub mod units;

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(units::UnitsPlugin)
            .add_systems(OnEnter(GameState::Battle), setup);
    }
}

fn setup(mut commands: Commands, mut camera_zoom: Query<&mut Zoom, With<Camera>>) {
    let mut camera_zoom = camera_zoom.single_mut();
    camera_zoom.zoom_level = 3.0;

    let normal = Normal::new(10.0, 2.0).unwrap();
    let mut rng = rand::thread_rng();

    let count_a = normal.sample(&mut rng) as usize;
    let count_b = normal.sample(&mut rng) as usize;

    let (count_large, count_small) = if count_a > count_b {
        (count_a, count_b)
    } else {
        (count_b, count_a)
    };

    info!("Player: {}, Enemy: {}", count_large, count_small);

    commands.spawn(UnitSpawnBundle {
        spawn: UnitSpawn {
            formation: rand_formation(),
            team: Team::Player,
            unit: KnightBundle::default(),
            unit_count: count_large,
            unit_size: UnitSprite::Knight.sprite_size(),
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
            unit_size: UnitSprite::Knight.sprite_size(),
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
