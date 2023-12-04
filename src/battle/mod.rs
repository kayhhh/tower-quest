use bevy::prelude::*;

use crate::{zoom::Zoom, GameState};

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UnitSprites>()
            .add_systems(Startup, load_sprites)
            .add_systems(OnEnter(GameState::Battle), setup)
            .add_systems(Update, (spawn_units, spawn_sprites))
            .add_systems(OnExit(GameState::Battle), (despawn_units, reset_spawns));
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

#[derive(Default)]
pub struct SpawnCount(pub usize);

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

fn setup(mut commands: Commands, mut camera_zoom: Query<&mut Zoom, With<Camera>>) {
    let mut camera_zoom = camera_zoom.single_mut();
    camera_zoom.zoom_level = 3.0;

    commands.spawn(UnitSpawnBundle {
        spawn: UnitSpawn {
            formation: Formation::Box,
            unit_count: 10,
            team: Team::Player,
            unit: Unit::Knight,
            ..default()
        },
        transform: TransformBundle {
            local: Transform::from_xyz(-100.0, 0.0, 0.0),
            ..default()
        },
    });

    commands.spawn(UnitSpawnBundle {
        spawn: UnitSpawn {
            formation: Formation::Box,
            unit_count: 10,
            team: Team::Enemy,
            unit: Unit::Knight,
            ..default()
        },
        transform: TransformBundle {
            local: Transform::from_xyz(100.0, 0.0, 0.0),
            ..default()
        },
    });
}

fn spawn_units(mut commands: Commands, mut spawns: Query<(&mut UnitSpawn, &Transform)>) {
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

            let mut ent = commands.spawn((
                TransformBundle {
                    local: Transform::from_xyz(x, y, 0.0),
                    ..default()
                },
                VisibilityBundle::default(),
            ));

            match spawn.unit {
                Unit::Knight => ent.insert(Unit::Knight),
                Unit::Archer => ent.insert(Unit::Archer),
            };

            match spawn.team {
                Team::Player => ent.insert(Team::Player),
                Team::Enemy => ent.insert(Team::Enemy),
            };
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

fn spawn_sprites(
    mut commands: Commands,
    sprites: Res<UnitSprites>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut units: Query<(Entity, &Unit), Without<TextureAtlasSprite>>,
) {
    for (ent, unit) in units.iter_mut() {
        info!("Spawning sprite for {:?}", unit);

        match unit {
            Unit::Archer => {
                let atlas = texture_atlases.add(TextureAtlas::from_grid(
                    sprites.archer.clone(),
                    Unit::Archer.sprite_size(),
                    3,
                    1,
                    Some(Vec2::splat(1.0)),
                    Some(Vec2::splat(1.0)),
                ));

                commands
                    .entity(ent)
                    .insert((TextureAtlasSprite::new(0), atlas));
            }

            Unit::Knight => {
                let atlas = texture_atlases.add(TextureAtlas::from_grid(
                    sprites.knight.clone(),
                    Unit::Knight.sprite_size(),
                    3,
                    1,
                    Some(Vec2::splat(1.0)),
                    Some(Vec2::splat(1.0)),
                ));

                commands
                    .entity(ent)
                    .insert((TextureAtlasSprite::new(0), atlas));
            }
        };
    }
}

fn despawn_units(mut commands: Commands, units: Query<Entity, With<Unit>>) {
    for ent in &mut units.iter() {
        commands.entity(ent).despawn_recursive();
    }
}

fn reset_spawns(mut spawns: Query<&mut UnitSpawn>) {
    for mut spawn in &mut spawns.iter_mut() {
        spawn.spawned = false;
    }
}
