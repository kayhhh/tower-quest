use bevy::prelude::*;
use rand::Rng;

use crate::battle::units::squad::{SquadBundle, SquadCount, UnitType};

use super::{Team, INITIAL_UNITS};

pub const ARENA_WIDTH: f32 = 600.0;
pub const ARENA_HEIGHT: f32 = 200.0;
/// Gap between the two territories
const TEAM_GAP: f32 = 100.0;

pub const ROWS: usize = 3;
pub const COLUMNS: usize = 3;

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
    let units_row = rng.gen_range(1..=ROWS);

    for row in 1..=ROWS {
        for column in 1..=COLUMNS {
            let territory_width = (ARENA_WIDTH / 2.0) - (TEAM_GAP / 2.0);
            let column_width = territory_width / COLUMNS as f32;
            let row_height = ARENA_HEIGHT / ROWS as f32;

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

            if column == 1 && row == units_row {
                let num_units = match team {
                    Team::Player => INITIAL_UNITS,
                    Team::Enemy => INITIAL_UNITS / 2,
                };

                let _squad = commands.entity(slot).insert(SquadBundle {
                    unit: UnitType::Knight,
                    count: SquadCount(num_units),
                    ..default()
                });
            }
        }
    }
}

#[derive(Resource)]
pub struct RallyFlagSprites {
    friendly: Handle<Image>,
    enemy: Handle<Image>,
}

pub fn load_flag_images(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(RallyFlagSprites {
        friendly: asset_server.load("images/units/RallyFriendly.png"),
        enemy: asset_server.load("images/units/RallyEnemy.png"),
    });
}

#[derive(Component, Default)]
pub struct RallyFlag {
    pub spawned: bool,
}

pub fn add_flags(
    mut commands: Commands,
    slots: Query<Entity, (With<SquadSlot>, Without<RallyFlag>)>,
) {
    for ent in slots.iter() {
        commands.entity(ent).insert(RallyFlag::default());
    }
}

pub fn spawn_flag_sprites(
    mut commands: Commands,
    images: Res<RallyFlagSprites>,
    mut flags: Query<(Entity, &Team, &mut RallyFlag)>,
) {
    for (ent, team, mut flag) in flags.iter_mut() {
        if flag.spawned {
            continue;
        }

        commands
            .spawn(SpriteBundle {
                texture: match team {
                    Team::Player => images.friendly.clone(),
                    Team::Enemy => images.enemy.clone(),
                },
                ..default()
            })
            .set_parent(ent);

        flag.spawned = true;
    }
}
