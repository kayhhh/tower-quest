use bevy::prelude::*;

use crate::GameState;

mod ai;
pub mod presets;
pub mod spawn;

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UnitSprites>()
            .add_systems(Startup, load_sprites)
            .add_systems(
                Update,
                (
                    (ai::set_target, ai::move_units, ai::attack).chain(),
                    spawn::spawn_sprites,
                    spawn::spawn_units::<presets::KnightBundle>,
                    spawn::spawn_units::<presets::ArcherBundle>,
                ),
            )
            .add_systems(
                OnExit(GameState::Battle),
                (
                    despawn_units,
                    spawn::reset_spawns::<presets::KnightBundle>,
                    spawn::reset_spawns::<presets::ArcherBundle>,
                ),
            );
    }
}

#[derive(Default, Resource)]
pub struct UnitSprites {
    archer: Handle<Image>,
    knight: Handle<Image>,
}

fn load_sprites(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(UnitSprites {
        archer: asset_server.load("sprites/Knight.png"),
        knight: asset_server.load("sprites/Knight.png"),
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
            UnitSprite::Knight => Vec2::new(11.0, 8.0),
            UnitSprite::Archer => Vec2::new(11.0, 8.0),
        }
    }
}

#[derive(Component, Clone, Default)]
pub struct Health(pub f32);

#[derive(Component, Clone, Debug, Default, PartialEq)]
pub enum Team {
    #[default]
    Player,
    Enemy,
}

#[derive(Default)]
pub enum Formation {
    Line,
    Column,
    #[default]
    Box,
}

fn despawn_units(mut commands: Commands, units: Query<Entity, With<UnitSprite>>) {
    for ent in &mut units.iter() {
        commands.entity(ent).despawn_recursive();
    }
}
