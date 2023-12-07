use bevy::prelude::*;

use crate::GameState;

use self::units::Team;

pub mod camera;
mod defeat;
mod enemy;
pub mod layout;
pub mod units;
mod victory;

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(units::UnitsPlugin)
            .add_systems(Startup, (layout::init_slots, layout::load_flag_images))
            .add_systems(
                Update,
                (
                    (
                        camera::calc_bounds,
                        camera::set_camera_velocity,
                        camera::apply_camera_velocity,
                    )
                        .chain(),
                    layout::add_flags,
                    layout::spawn_flag_sprites,
                    victory::detect_victory,
                )
                    .run_if(in_state(GameState::Battle)),
            )
            .add_systems(
                OnExit(GameState::Victory),
                (victory::increase_floor, enemy::upgrade_enemy).chain(),
            )
            .add_systems(OnEnter(GameState::Defeat), defeat::spawn_menu)
            .add_systems(OnExit(GameState::Defeat), defeat::cleanup_menu);
    }
}

pub const INITIAL_UNITS: usize = 10;
