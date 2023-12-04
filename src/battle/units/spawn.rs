use bevy::prelude::*;

use crate::battle::units::{Health, UnitBundle};

use super::{Formation, Team, Unit, UnitSprites};

#[derive(Component, Default)]
pub struct UnitSpawn {
    pub unit: Unit,
    pub team: Team,
    pub formation: Formation,
    /// Number of units to spawn
    pub unit_count: usize,
    /// Whether the units have been spawned yet
    pub spawned: bool,
}

#[derive(Bundle, Default)]
pub struct UnitSpawnBundle {
    pub spawn: UnitSpawn,
    pub transform: TransformBundle,
}

pub fn spawn_units(mut commands: Commands, mut spawns: Query<(&mut UnitSpawn, &Transform)>) {
    for (mut spawn, transform) in spawns.iter_mut() {
        if spawn.spawned {
            continue;
        }

        let coords = match spawn.formation {
            Formation::Line => coords_line(&spawn),
            Formation::Column => coords_column(&spawn),
            Formation::Box => coords_box(&spawn),
        };

        for (mut x, mut y) in coords {
            x *= spawn.unit.sprite_size().x;
            y *= spawn.unit.sprite_size().y;

            x += transform.translation.x;
            y += transform.translation.y;

            info!("Spawning {:?} at ({}, {})", spawn.unit, x, y);

            commands.spawn(UnitBundle {
                unit: spawn.unit.clone(),
                team: spawn.team.clone(),
                health: Health(spawn.unit.max_health()),
                transform: TransformBundle {
                    local: Transform::from_xyz(x, y, 0.0),
                    ..default()
                },
                ..default()
            });
        }

        spawn.spawned = true;
    }
}

fn coords_line(spawn: &UnitSpawn) -> Vec<(f32, f32)> {
    let mut coords = Vec::new();

    let mut x = 0.0;
    let y = 0.0;

    for _ in 0..spawn.unit_count {
        coords.push((x, y));

        x += 1.0;
    }

    coords
}

fn coords_column(spawn: &UnitSpawn) -> Vec<(f32, f32)> {
    let mut coords = Vec::new();

    let x = 0.0;
    let mut y = 0.0;

    for _ in 0..spawn.unit_count {
        coords.push((x, y));

        y += 1.0;
    }

    coords
}

fn coords_box(spawn: &UnitSpawn) -> Vec<(f32, f32)> {
    let mut coords = Vec::new();

    let mut x = 0.0;
    let mut y = 0.0;

    // Find closest square, greater than or equal to unit_count
    let mut square = 1;
    while square * square < spawn.unit_count {
        square += 1;
    }

    for _ in 0..spawn.unit_count {
        coords.push((x, y));

        x += 1.0;

        if x >= square as f32 {
            x = 0.0;
            y += 1.0;
        }
    }

    coords
}

pub fn spawn_sprites(
    mut commands: Commands,
    sprites: Res<UnitSprites>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut units: Query<(Entity, &Unit, &Transform), Without<TextureAtlasSprite>>,
) {
    for (ent, unit, transform) in units.iter_mut() {
        info!("Spawning sprite for {:?}", unit);

        let atlas = match unit {
            Unit::Archer => texture_atlases.add(TextureAtlas::from_grid(
                sprites.archer.clone(),
                Unit::Archer.sprite_size(),
                3,
                1,
                Some(Vec2::splat(1.0)),
                Some(Vec2::splat(1.0)),
            )),

            Unit::Knight => texture_atlases.add(TextureAtlas::from_grid(
                sprites.knight.clone(),
                Unit::Knight.sprite_size(),
                3,
                1,
                Some(Vec2::splat(1.0)),
                Some(Vec2::splat(1.0)),
            )),
        };

        let sprite = TextureAtlasSprite {
            index: 0,
            flip_x: transform.translation.x > 0.0,
            ..default()
        };

        commands.entity(ent).insert((sprite, atlas));
    }
}

pub fn reset_spawns(mut spawns: Query<&mut UnitSpawn>) {
    for mut spawn in &mut spawns.iter_mut() {
        spawn.spawned = false;
    }
}
