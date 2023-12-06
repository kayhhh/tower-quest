use bevy::prelude::*;
use rand::Rng;

use crate::battle::units::squad::{SquadBundle, SquadCount, UnitType};

use super::Team;

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
                    SquadSlot,
                    Team::Player,
                    TransformBundle {
                        local: Transform::from_xyz(x, y, 0.0),
                        ..default()
                    },
                ))
                .id();

            if column == 1 && row == units_row {
                info!("Spawning {:?} units in row {}", team, row);

                let squad = commands
                    .spawn(SquadBundle {
                        unit: UnitType::Knight,
                        count: SquadCount(10),
                        team: team.clone(),
                        ..default()
                    })
                    .id();

                commands.entity(slot).add_child(squad);
            }
        }
    }
}
