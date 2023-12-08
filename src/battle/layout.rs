use bevy::prelude::*;
use rand::Rng;

use crate::battle::units::squad::{SquadBundle, SquadCount, UnitType};

use super::{units::squad::Squad, Team, INITIAL_UNITS};

pub const ARENA_WIDTH: f32 = 500.0;
pub const ARENA_HEIGHT: f32 = 300.0;
/// Gap between the two territories
const TEAM_GAP: f32 = 75.0;

pub const MAX_ROWS: usize = 5;
pub const MAX_COLUMNS: usize = 3;

const INITIAL_ROWS: usize = 3;
const INITIAL_COLUMNS: usize = 1;

pub struct UnlockedSlots {
    pub rows: usize,
    pub columns: usize,
}

impl Default for UnlockedSlots {
    fn default() -> Self {
        Self {
            rows: INITIAL_ROWS,
            columns: INITIAL_COLUMNS,
        }
    }
}

#[derive(Resource, Default)]
pub struct FriendlyUnlockedSlots(UnlockedSlots);

#[derive(Resource, Default)]
pub struct EnemyUnlockedSlots(UnlockedSlots);

#[derive(Component)]
pub struct SquadSlot;

pub fn init_slots(mut commands: Commands) {
    spawn_slots(&mut commands, &Team::Player);
    spawn_slots(&mut commands, &Team::Enemy);
}

/// Each team has a grid of slots that units can be placed in.
fn spawn_slots(commands: &mut Commands, team: &Team) {
    let mut rng = rand::thread_rng();

    // Which row to spawn units in
    let units_row = rng.gen_range(1..=INITIAL_ROWS);
    let units_column = rng.gen_range(1..=INITIAL_COLUMNS);

    for row in 1..=MAX_ROWS {
        for column in 1..=MAX_COLUMNS {
            let territory_width = (ARENA_WIDTH / 2.0) - (TEAM_GAP / 2.0);
            let column_width = territory_width / MAX_COLUMNS as f32;
            let row_height = ARENA_HEIGHT / MAX_ROWS as f32;

            let x = column_width * column as f32 + column_width / 2.0;
            let y = row_height * row as f32 + row_height / 2.0 - ARENA_HEIGHT / 2.0;

            let x = match team {
                Team::Player => -x,
                Team::Enemy => x,
            };

            let slot = commands
                .spawn((
                    team.clone(),
                    SquadSlot,
                    TransformBundle {
                        local: Transform::from_xyz(x, y, 0.0),
                        ..default()
                    },
                    VisibilityBundle::default(),
                ))
                .id();

            if row == units_row && column == units_column {
                let num_units = match team {
                    Team::Player => INITIAL_UNITS,
                    Team::Enemy => INITIAL_UNITS / 2,
                };

                commands.entity(slot).insert(SquadBundle {
                    unit: UnitType::Knight,
                    count: SquadCount(num_units),
                    ..default()
                });
            }
        }
    }
}

#[derive(Resource)]
pub struct MarkerImages {
    friendly_flag: Handle<Image>,
    enemy_flag: Handle<Image>,
    x: Handle<Image>,
}

pub fn load_marker_images(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(MarkerImages {
        friendly_flag: asset_server.load("images/arena/RallyFriendly.png"),
        enemy_flag: asset_server.load("images/arena/RallyEnemy.png"),
        x: asset_server.load("images/arena/x.png"),
    });
}

#[derive(Component, Default)]
pub struct RallyMarkers {
    pub spawned: bool,
    pub has_squad: bool,
}

pub fn add_markers(
    mut commands: Commands,
    slots: Query<Entity, (With<SquadSlot>, Without<RallyMarkers>)>,
) {
    for ent in slots.iter() {
        commands.entity(ent).insert(RallyMarkers::default());
    }
}

pub fn spawn_marker_sprites(
    mut commands: Commands,
    images: Res<MarkerImages>,
    mut flags: Query<(Entity, &Team, &mut RallyMarkers, Option<&Squad>)>,
) {
    for (ent, team, mut flag, squad) in flags.iter_mut() {
        let has_squad = squad.is_some();

        if flag.spawned && flag.has_squad == has_squad {
            continue;
        }

        flag.has_squad = has_squad;

        let image = match squad {
            Some(_) => match team {
                Team::Player => images.friendly_flag.clone(),
                Team::Enemy => images.enemy_flag.clone(),
            },
            None => images.x.clone(),
        };

        commands.entity(ent).insert((Sprite::default(), image));
    }
}
