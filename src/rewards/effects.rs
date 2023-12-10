use bevy::prelude::*;
use rand::Rng;

use crate::battle::{
    layout::{slot_coords, EnemyUnlockedSlots, FriendlyUnlockedSlots, SquadSlot},
    units::{
        squad::{Squad, SquadBundle},
        Team,
    },
};

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AddColumn>()
            .add_event::<AddRow>()
            .add_event::<AddSquad>()
            .add_event::<AddMovementSpeed>()
            .add_systems(Update, (add_column, add_row, add_squad, add_movement_speed));
    }
}

#[derive(Component)]
pub enum ItemEffect {
    AddColumn,
    AddMovementSpeed(f32),
    AddRow,
    AddSquad(SquadBundle),
}

pub struct SpeedModifier(pub f32);

impl Default for SpeedModifier {
    fn default() -> Self {
        Self(1.0)
    }
}

#[derive(Resource, Default)]
pub struct FriendlySpeedModifier(pub SpeedModifier);

#[derive(Resource, Default)]
pub struct EnemySpeedModifier(pub SpeedModifier);

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
pub struct AddMovementSpeed {
    pub speed: f32,
    pub team: Team,
}

fn add_movement_speed(
    mut events: EventReader<AddMovementSpeed>,
    enemy_modifier: ResMut<EnemySpeedModifier>,
    friendly_modifier: ResMut<FriendlySpeedModifier>,
) {
    for AddMovementSpeed { speed, team } in events.read() {
        info!("Adding movement speed to {:?}", team);

        let mut modifier = match team {
            Team::Player => friendly_modifier.0 .0,
            Team::Enemy => enemy_modifier.0 .0,
        };

        modifier += speed;
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
        info!("Adding {:?} squad", team);

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
        info!("Adding column to {:?}", event.team);

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
        info!("Adding row to {:?}", event.team);

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
