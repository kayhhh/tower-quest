use bevy::prelude::*;

use crate::GameState;

pub mod spawn;

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UnitSprites>()
            .add_systems(Startup, load_sprites)
            .add_systems(Update, (spawn::spawn_units, spawn::spawn_sprites))
            .add_systems(
                OnExit(GameState::Battle),
                (despawn_units, spawn::reset_spawns),
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

#[derive(Component, Debug, Default, PartialEq)]
pub enum Unit {
    Archer,
    #[default]
    Knight,
}

impl Unit {
    pub fn sprite_size(&self) -> Vec2 {
        match self {
            Unit::Knight => Vec2::new(11.0, 8.0),
            Unit::Archer => Vec2::new(11.0, 8.0),
        }
    }
}

#[derive(Component, Default)]
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

fn despawn_units(mut commands: Commands, units: Query<Entity, With<Unit>>) {
    for ent in &mut units.iter() {
        commands.entity(ent).despawn_recursive();
    }
}
