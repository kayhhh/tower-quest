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
            .add_systems(
                Startup,
                (layout::init_slots, layout::load_marker_images, start_music),
            )
            .add_systems(
                Update,
                (
                    (
                        camera::calc_bounds,
                        camera::set_camera_velocity,
                        camera::apply_camera_velocity,
                    )
                        .chain(),
                    layout::add_markers,
                    layout::spawn_marker_sprites,
                    victory::detect_victory,
                )
                    .run_if(in_state(GameState::Battle)),
            )
            .add_systems(
                OnExit(GameState::Victory),
                (victory::increase_floor, enemy::upgrade_enemy).chain(),
            )
            .add_systems(OnEnter(GameState::Defeat), defeat::spawn_menu)
            .add_systems(
                OnExit(GameState::Defeat),
                (defeat::cleanup_menu, cleanup_slots, layout::init_slots),
            );
    }
}

pub const INITIAL_UNITS: usize = 10;

fn start_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/groove.ogg"),
        ..default()
    });
}

fn cleanup_slots(mut commands: Commands, slots: Query<Entity, With<layout::SquadSlot>>) {
    for ent in &mut slots.iter() {
        commands.entity(ent).despawn_recursive();
    }
}
