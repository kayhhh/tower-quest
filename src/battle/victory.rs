use bevy::prelude::*;

use crate::{Floor, GameState};

use super::units::{ai::Dead, squad::Unit, Team};

pub fn detect_victory(
    mut battle_started: Local<bool>,
    units: Query<&Team, (With<Unit>, Without<Dead>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if !*battle_started {
        if units.iter().count() > 0 {
            *battle_started = true;
        }

        return;
    }

    let mut player_alive = false;
    let mut enemy_alive = false;

    for team in units.iter() {
        match team {
            Team::Player => player_alive = true,
            Team::Enemy => enemy_alive = true,
        }
    }

    if player_alive && enemy_alive {
        return;
    }

    if player_alive {
        info!("Player wins!");
        next_state.set(GameState::Victory);
    } else if !enemy_alive {
        info!("Enemy wins!");
        next_state.set(GameState::Defeat);
    }

    *battle_started = false;
}

pub fn increase_floor(mut floor: ResMut<Floor>) {
    floor.0 += 1;
}
