use bevy::prelude::*;

use super::{formation::Formation, presets::UnitBundle, Team};

#[derive(Component)]
pub struct Unit;

#[derive(Component, Clone, Default)]
pub struct Squad;

#[derive(Component, Clone, Default, Debug)]
pub enum UnitType {
    Archer,
    #[default]
    Knight,
}

impl UnitType {
    pub fn spacing(&self) -> Vec2 {
        match self {
            UnitType::Knight => Vec2::splat(12.0),
            UnitType::Archer => Vec2::splat(12.0),
        }
    }

    pub fn sprite_size(&self) -> Vec2 {
        match self {
            UnitType::Knight => Vec2::new(15.0, 8.0),
            UnitType::Archer => Vec2::new(15.0, 8.0),
        }
    }
}

#[derive(Component, Clone, Default)]
pub struct SquadCount(pub usize);

#[derive(Bundle, Clone, Default)]
pub struct SquadBundle {
    pub count: SquadCount,
    pub formation: Formation,
    pub squad: Squad,
    pub team: Team,
    pub transform: TransformBundle,
    pub unit: UnitType,
}

pub fn spawn_units(
    mut commands: Commands,
    mut squads: Query<(Entity, &Formation, &Team, &SquadCount, &UnitType), With<Squad>>,
) {
    for (ent, formation, team, count, unit) in squads.iter_mut() {
        let coords = formation.coords(count.0);

        for (mut x, mut y) in coords {
            x *= unit.spacing().x;
            y *= unit.spacing().y;

            let x = match team {
                Team::Player => -x,
                Team::Enemy => x,
            };

            let unit_bundle = match unit {
                UnitType::Knight => UnitBundle::knight(),
                UnitType::Archer => UnitBundle::archer(),
            };

            commands
                .spawn((
                    Unit,
                    TransformBundle::from_transform(Transform::from_xyz(x, y, 0.0)),
                    VisibilityBundle::default(),
                    team.clone(),
                    unit.clone(),
                    unit_bundle,
                ))
                .set_parent(ent);
        }
    }
}
