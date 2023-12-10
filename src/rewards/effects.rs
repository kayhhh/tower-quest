use bevy::prelude::*;
use rand::Rng;

use crate::{
    battle::{
        layout::{slot_coords, EnemyUnlockedSlots, FriendlyUnlockedSlots, SquadSlot},
        units::{
            squad::{Squad, SquadBundle},
            Team,
        },
    },
    GameState,
};

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AddColumn>()
            .add_event::<AddRow>()
            .add_event::<AddSquad>()
            .add_event::<AddMovementSpeed>()
            .add_systems(
                Update,
                (add_column, add_row, add_squad, add_movement_speed)
                    .run_if(in_state(GameState::PreBattle)),
            );
    }
}

#[derive(Component)]
pub enum ItemEffect {
    AddColumn,
    AddMovementSpeed(f32),
    AddRow,
    AddSquad(SquadBundle),
}

#[derive(Resource)]
pub struct SpeedModifier(pub f32);

impl Default for SpeedModifier {
    fn default() -> Self {
        Self(1.0)
    }
}

#[derive(Event)]
pub struct AddColumn {
    pub team: Team,
}

#[derive(Event)]
pub struct AddRow {
    pub team: Team,
}

#[derive(Event)]
pub struct AddSquad {
    pub squad: SquadBundle,
    pub team: Team,
}

#[derive(Event)]
pub struct AddMovementSpeed(pub f32);

fn add_movement_speed(
    mut speed_modifier: ResMut<SpeedModifier>,
    mut events: EventReader<AddMovementSpeed>,
) {
    for AddMovementSpeed(speed) in events.read() {
        speed_modifier.0 += speed;
    }
}

fn add_squad(
    mut commands: Commands,
    mut events: EventReader<AddSquad>,
    open_slots: Query<(Entity, &Team), (With<SquadSlot>, Without<Squad>)>,
) {
    // Wait for slots to be spawned
    if open_slots.iter().count() == 0 {
        return;
    }

    for AddSquad { squad, team } in events.read() {
        let open_slots = open_slots
            .iter()
            .filter(|(_, t)| **t == *team)
            .map(|(ent, _)| ent)
            .collect::<Vec<_>>();

        let count = open_slots.len();

        if count == 0 {
            error!("No open slots");
            continue;
        }

        let mut rng = rand::thread_rng();
        let slot = open_slots[rng.gen_range(0..count)];

        info!("Adding {:?} squad to slot: {:?}", team, slot);

        commands.entity(slot).insert(squad.clone());
    }
}

fn add_column(
    mut commands: Commands,
    mut events: EventReader<AddColumn>,
    mut friendly_slots: ResMut<FriendlyUnlockedSlots>,
    mut enemy_slots: ResMut<EnemyUnlockedSlots>,
) {
    for event in events.read() {
        let slots = match event.team {
            Team::Player => &mut friendly_slots.0,
            Team::Enemy => &mut enemy_slots.0,
        };

        slots.columns += 1;

        for row in 1..=slots.rows {
            let (x, y) = slot_coords(row, slots.columns);

            let x = match event.team {
                Team::Player => -x,
                Team::Enemy => x,
            };

            commands.spawn((
                event.team.clone(),
                SquadSlot,
                TransformBundle {
                    local: Transform::from_xyz(x, y, 0.0),
                    ..default()
                },
                VisibilityBundle::default(),
            ));
        }
    }
}

fn add_row(
    mut commands: Commands,
    mut events: EventReader<AddRow>,
    mut friendly_slots: ResMut<FriendlyUnlockedSlots>,
    mut enemy_slots: ResMut<EnemyUnlockedSlots>,
) {
    for event in events.read() {
        let slots = match event.team {
            Team::Player => &mut friendly_slots.0,
            Team::Enemy => &mut enemy_slots.0,
        };

        slots.rows += 1;

        for column in 1..=slots.columns {
            let (x, y) = slot_coords(slots.rows, column);

            let x = match event.team {
                Team::Player => -x,
                Team::Enemy => x,
            };

            commands.spawn((
                event.team.clone(),
                SquadSlot,
                TransformBundle {
                    local: Transform::from_xyz(x, y, 0.0),
                    ..default()
                },
                VisibilityBundle::default(),
            ));
        }
    }
}
