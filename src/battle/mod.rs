use bevy::prelude::*;

use crate::GameState;

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Battle), setup)
            .add_systems(OnExit(GameState::Battle), cleanup);
    }
}

#[derive(Component)]
pub enum Unit {
    Archer,
    Knight,
}

#[derive(Component)]
pub enum Team {
    Player,
    Enemy,
}

fn setup(mut commands: Commands) {
    for i in 0..10 {
        commands.spawn((
            Unit::Knight,
            Team::Player,
            TransformBundle {
                local: Transform::from_translation(Vec3::new(i as f32 * 1.0, 0.0, 0.0)),
                ..default()
            },
        ));
    }

    for i in 0..5 {
        commands.spawn((
            Unit::Knight,
            Team::Enemy,
            TransformBundle {
                local: Transform::from_translation(Vec3::new(i as f32 * 1.0, 10.0, 0.0)),
                ..default()
            },
        ));
    }
}

fn cleanup(mut commands: Commands) {}
