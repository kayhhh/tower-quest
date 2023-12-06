use bevy::prelude::*;

use crate::GameState;

pub mod ai;
pub mod animation;
pub mod presets;
pub mod spawn;

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UnitSprites>()
            .add_event::<animation::AttackEvent>()
            .add_systems(Startup, load_sprites)
            .add_systems(OnEnter(GameState::Battle), despawn_units)
            .add_systems(
                Update,
                (
                    (ai::set_target, ai::move_units, ai::attack).chain(),
                    (
                        animation::animate_atlas,
                        animation::animate_attack,
                        animation::flip_units,
                        spawn::spawn_sprites,
                        spawn::spawn_units::<presets::KnightBundle>,
                        spawn::spawn_units::<presets::ArcherBundle>,
                    )
                        .run_if(in_state(GameState::Battle)),
                ),
            )
            .add_systems(
                OnExit(GameState::Battle),
                (
                    spawn::reset_spawns::<presets::KnightBundle>,
                    spawn::reset_spawns::<presets::ArcherBundle>,
                ),
            );
    }
}

#[derive(Default, Resource)]
pub struct UnitSprites {
    archer_enemy: Handle<Image>,
    archer_friendly: Handle<Image>,
    knight_enemy: Handle<Image>,
    knight_friendly: Handle<Image>,
}

fn load_sprites(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(UnitSprites {
        archer_enemy: asset_server.load("images/units/ArcherEnemy.png"),
        archer_friendly: asset_server.load("images/units/ArcherFriendly.png"),
        knight_enemy: asset_server.load("images/units/KnightEnemy.png"),
        knight_friendly: asset_server.load("images/units/KnightFriendly.png"),
    });
}

#[derive(Component, Clone, Debug, Default, PartialEq)]
pub enum UnitSprite {
    Archer,
    #[default]
    Knight,
}

impl UnitSprite {
    pub fn sprite_size(&self) -> Vec2 {
        match self {
            UnitSprite::Knight => Vec2::new(15.0, 8.0),
            UnitSprite::Archer => Vec2::new(15.0, 8.0),
        }
    }
}

#[derive(Component, Clone, Debug, Default, PartialEq)]
pub enum Team {
    #[default]
    Player,
    Enemy,
}

#[derive(Clone, Default)]
pub enum Formation {
    Column,
    #[default]
    Box,
}

fn despawn_units(mut commands: Commands, units: Query<Entity, With<UnitSprite>>) {
    for ent in &mut units.iter() {
        commands.entity(ent).despawn_recursive();
    }
}
