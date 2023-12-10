use bevy::prelude::*;

use crate::GameState;

pub mod ai;
pub mod animation;
pub mod formation;
pub mod presets;
mod sounds;
mod sprites;
pub mod squad;

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(sounds::SoundsPlugin)
            .init_resource::<sprites::UnitSprites>()
            .add_event::<animation::AttackEvent>()
            .add_systems(Startup, sprites::load_sprites)
            .add_systems(
                OnEnter(GameState::Battle),
                (despawn_units, squad::spawn_units),
            )
            .add_systems(
                Update,
                (
                    (ai::set_target, ai::move_units, ai::attack).chain(),
                    (
                        animation::animate_atlas,
                        animation::animate_attack,
                        animation::flip_units,
                        sprites::spawn_sprites,
                    )
                        .run_if(in_state(GameState::Battle)),
                ),
            );
    }
}

#[derive(Component, Clone, Debug, Default, PartialEq)]
pub enum Team {
    #[default]
    Player,
    Enemy,
}

fn despawn_units(mut commands: Commands, units: Query<Entity, With<squad::Unit>>) {
    for ent in &mut units.iter() {
        commands.entity(ent).despawn_recursive();
    }
}
