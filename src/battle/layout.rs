use bevy::prelude::*;

use crate::{
    battle::INITIAL_UNITS,
    rewards::effects::{AddColumn, AddRow, AddSquad},
};

use super::{
    enemy::rand_unit_count,
    units::squad::{Squad, SquadBundle, SquadCount, UnitType},
    Team,
};

pub const ARENA_WIDTH: f32 = 500.0;
pub const ARENA_HEIGHT: f32 = 300.0;
/// Gap between the two territories
const TEAM_GAP: f32 = 75.0;

pub const MAX_ROWS: usize = 5;
pub const MAX_COLUMNS: usize = 3;

pub const INITIAL_ROWS: usize = 3;
pub const INITIAL_COLUMNS: usize = 1;

#[derive(Default)]
pub struct UnlockedSlots {
    pub rows: usize,
    pub columns: usize,
}

#[derive(Resource, Default)]
pub struct FriendlyUnlockedSlots(pub UnlockedSlots);

#[derive(Resource, Default)]
pub struct EnemyUnlockedSlots(pub UnlockedSlots);

#[derive(Component)]
pub struct SquadSlot;

pub fn init_slots(
    mut add_column_writer: EventWriter<AddColumn>,
    mut add_row_writer: EventWriter<AddRow>,
) {
    for team in &[Team::Player, Team::Enemy] {
        for _ in 0..INITIAL_COLUMNS {
            add_column_writer.send(AddColumn { team: team.clone() });
        }

        for _ in 0..INITIAL_ROWS {
            add_row_writer.send(AddRow { team: team.clone() });
        }
    }
}

pub fn init_units(mut add_squad_writer: EventWriter<AddSquad>) {
    for team in &[Team::Player, Team::Enemy] {
        let num_units = match team {
            Team::Player => INITIAL_UNITS,
            Team::Enemy => rand_unit_count(1),
        };

        add_squad_writer.send(AddSquad {
            team: team.clone(),
            squad: SquadBundle {
                unit: UnitType::Knight,
                count: SquadCount(num_units),
                ..default()
            },
        });
    }
}

const TERRITORY_WIDTH: f32 = (ARENA_WIDTH / 2.0) - (TEAM_GAP / 2.0);
const COLUMN_WIDTH: f32 = TERRITORY_WIDTH / MAX_COLUMNS as f32;
const ROW_HEIGHT: f32 = ARENA_HEIGHT / MAX_ROWS as f32;

pub fn slot_coords(row: usize, column: usize) -> (f32, f32) {
    let x = column as f32 * COLUMN_WIDTH + COLUMN_WIDTH / 2.0;
    let y = row as f32 * ROW_HEIGHT + ROW_HEIGHT / 2.0 - ARENA_HEIGHT / 2.0;

    (x, y)
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
