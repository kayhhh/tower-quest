use bevy::prelude::*;

use crate::GameState;

use self::units::Team;

pub mod camera;
mod enemy;
mod layout;
pub mod units;
mod victory;

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(units::UnitsPlugin)
            .add_systems(Startup, layout::init_slots)
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

pub const INITIAL_UNITS: usize = 10;
