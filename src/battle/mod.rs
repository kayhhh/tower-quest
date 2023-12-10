use bevy::prelude::*;

use crate::GameState;

use self::{
    layout::{EnemyUnlockedSlots, FriendlyUnlockedSlots},
    units::Team,
};

pub mod camera;
mod defeat;
mod enemy;
pub mod layout;
pub mod units;
mod victory;

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FriendlyUnlockedSlots>()
            .init_resource::<EnemyUnlockedSlots>()
            .add_plugins(units::UnitsPlugin)
            .add_systems(Startup, layout::load_marker_images)
            .add_systems(
                OnEnter(GameState::InitBattle),
                (
                    despawn_slots,
                    init_unlocked_slots,
                    layout::init_slots,
                    layout::init_units,
                ),
            )
            .add_systems(
                Update,
                (
                    finish_init_battle.run_if(in_state(GameState::InitBattle)),
                    finish_pre_battle.run_if(in_state(GameState::PreBattle)),
                    layout::add_markers,
                    layout::spawn_marker_sprites,
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
                ),
            )
            .add_systems(OnExit(GameState::Victory), victory::increase_floor)
            .add_systems(OnEnter(GameState::Defeat), defeat::spawn_menu)
            .add_systems(OnExit(GameState::Defeat), defeat::cleanup_menu);
    }
}

pub const INITIAL_UNITS: usize = 10;

fn finish_init_battle(mut next_state: ResMut<NextState<GameState>>) {
    info!("Exiting InitBattle");
    next_state.set(GameState::PreBattle);
}

fn finish_pre_battle(mut next_state: ResMut<NextState<GameState>>) {
    info!("Exiting PreBattle");
    next_state.set(GameState::Battle);
}

fn despawn_slots(mut commands: Commands, slots: Query<Entity, With<layout::SquadSlot>>) {
    for ent in &mut slots.iter() {
        commands.entity(ent).despawn_recursive();
    }
}

fn init_unlocked_slots(mut commands: Commands) {
    commands.insert_resource(FriendlyUnlockedSlots::default());
    commands.insert_resource(EnemyUnlockedSlots::default());
}
