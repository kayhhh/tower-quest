use bevy::prelude::*;

use super::{Formation, Team, UnitSprite, UnitSprites};

#[derive(Component, Default)]
pub struct UnitSpawn<T: Bundle + Default> {
    pub unit: T,
    pub team: Team,
    pub formation: Formation,
    /// Number of units to spawn
    pub unit_count: usize,
    /// Size of each unit
    pub unit_size: Vec2,
    /// Whether the units have been spawned yet
    pub spawned: bool,
}

#[derive(Bundle, Default)]
pub struct UnitSpawnBundle<T: Bundle + Default> {
    pub spawn: UnitSpawn<T>,
    pub transform: TransformBundle,
}

pub fn spawn_units<T: Bundle + Clone + Default>(
    mut commands: Commands,
    mut spawns: Query<(&mut UnitSpawn<T>, &Transform)>,
) {
    for (mut spawn, transform) in spawns.iter_mut() {
        if spawn.spawned {
            continue;
        }

        let coords = match spawn.formation {
            Formation::Column => coords_column(spawn.unit_count),
            Formation::Box => coords_box(spawn.unit_count),
        };

        for (mut x, mut y) in coords {
            x *= spawn.unit_size.x;
            y *= spawn.unit_size.y;

            x += transform.translation.x;
            y += transform.translation.y;

            commands.spawn((
                spawn.unit.clone(),
                spawn.team.clone(),
                TransformBundle::from_transform(Transform::from_xyz(x, y, 0.0)),
                VisibilityBundle::default(),
            ));
        }

        spawn.spawned = true;
    }
}

fn coords_column(count: usize) -> Vec<(f32, f32)> {
    let mut coords = Vec::new();

    let x = 0.0;
    let mut y = 0.0;

    for _ in 0..count {
        coords.push((x, y));

        y += 1.0;
    }

    coords
}

fn coords_box(count: usize) -> Vec<(f32, f32)> {
    let mut coords = Vec::new();

    let mut x = 0.0;
    let mut y = 0.0;

    // Find closest square, greater than or equal to unit_count
    let mut square = 1;
    while square * square < count {
        square += 1;
    }

    for _ in 0..count {
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
    mut units: Query<(Entity, &UnitSprite, &Transform), Without<TextureAtlasSprite>>,
) {
    for (ent, unit, transform) in units.iter_mut() {
        let atlas = match unit {
            UnitSprite::Archer => texture_atlases.add(TextureAtlas::from_grid(
                sprites.archer.clone(),
                UnitSprite::Archer.sprite_size(),
                3,
                1,
                None,
                None,
            )),

            UnitSprite::Knight => texture_atlases.add(TextureAtlas::from_grid(
                sprites.knight.clone(),
                UnitSprite::Knight.sprite_size(),
                3,
                1,
                None,
                None,
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

pub fn reset_spawns<T: Bundle + Default>(mut spawns: Query<&mut UnitSpawn<T>>) {
    for mut spawn in &mut spawns.iter_mut() {
        spawn.spawned = false;
    }
}
