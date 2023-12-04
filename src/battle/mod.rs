use bevy::prelude::*;

use crate::{zoom::Zoom, GameState};

use self::units::{
    spawn::{UnitSpawn, UnitSpawnBundle},
    Formation, Team, Unit,
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

    commands.spawn(UnitSpawnBundle {
        spawn: UnitSpawn {
            formation: Formation::Box,
            unit_count: 10,
            team: Team::Player,
            unit: Unit::Knight,
            ..default()
        },
        transform: TransformBundle {
            local: Transform::from_xyz(-100.0, 0.0, 0.0),
            ..default()
        },
    });

    commands.spawn(UnitSpawnBundle {
        spawn: UnitSpawn {
            formation: Formation::Box,
            unit_count: 10,
            team: Team::Enemy,
            unit: Unit::Knight,
            ..default()
        },
        transform: TransformBundle {
            local: Transform::from_xyz(100.0, 0.0, 0.0),
            ..default()
        },
    });
}
