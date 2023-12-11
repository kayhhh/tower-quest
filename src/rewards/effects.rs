use bevy::prelude::*;
use rand::Rng;

use crate::battle::{
    layout::{slot_coords, EnemyUnlockedSlots, FriendlyUnlockedSlots, SquadSlot},
    units::{
        formation::rand_formation,
        squad::{Squad, SquadBundle, UnitType},
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
            .add_event::<SquadSizeMultiplier>()
            .add_systems(
                Update,
                (
                    add_column,
                    add_row,
                    add_squad,
                    add_movement_speed,
                    apply_squad_size_modifier,
                ),
            );
    }
}

#[derive(Component, Clone)]
pub enum ItemEffect {
    AddColumn,
    AddMovementSpeed(f32),
    AddRow,
    AddSquad(SquadBundle),
    SquadSizeMultiplier { multiplier: f32, unit: UnitType },
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

#[derive(Event)]
pub struct SquadSizeMultiplier {
    pub multiplier: f32,
    pub unit: UnitType,
    pub team: Team,
}

pub struct SquadSizeModifier(pub f32);

impl Default for SquadSizeModifier {
    fn default() -> Self {
        Self(1.0)
    }
}

#[derive(Resource, Default)]
pub struct FriendlyKnightSquadSizeModifier(pub SquadSizeModifier);

#[derive(Resource, Default)]
pub struct EnemyKnightSquadSizeModifier(pub SquadSizeModifier);

fn add_movement_speed(
    mut events: EventReader<AddMovementSpeed>,
    mut enemy_modifier: ResMut<EnemySpeedModifier>,
    mut friendly_modifier: ResMut<FriendlySpeedModifier>,
) {
    for AddMovementSpeed { speed, team } in events.read() {
        info!("Adding movement speed to {:?}", team);

        match team {
            Team::Player => friendly_modifier.0 .0 += speed,
            Team::Enemy => enemy_modifier.0 .0 += speed,
        };
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

        commands
            .entity(slot)
            .insert(squad.clone())
            .insert(rand_formation());
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

fn apply_squad_size_modifier(
    mut events: EventReader<SquadSizeMultiplier>,
    mut friendly_modifier: ResMut<FriendlyKnightSquadSizeModifier>,
    mut enemy_modifier: ResMut<EnemyKnightSquadSizeModifier>,
) {
    for SquadSizeMultiplier {
        multiplier,
        unit,
        team,
    } in events.read()
    {
        info!("Applying squad size modifier to {:?}", team);

        match team {
            Team::Player => match unit {
                UnitType::Knight => friendly_modifier.0 .0 *= multiplier,
                _ => {}
            },
            Team::Enemy => match unit {
                UnitType::Knight => enemy_modifier.0 .0 *= multiplier,
                _ => {}
            },
        };
    }
}
