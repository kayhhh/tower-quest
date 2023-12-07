use bevy::prelude::*;
use rand::Rng;

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
            UnitType::Knight => Vec2::splat(10.0),
            UnitType::Archer => Vec2::splat(10.0),
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
    mut squads: Query<
        (
            Entity,
            &GlobalTransform,
            &Formation,
            &Team,
            &SquadCount,
            &UnitType,
        ),
        With<Squad>,
    >,
) {
    let mut rng = rand::thread_rng();

    for (ent, transform, formation, team, count, unit) in squads.iter_mut() {
        let coords = formation.coords(count.0);

        // let translation = transform.translation();
        // println!("spawning at translation: {:?}", translation);

        for (mut x, mut y) in coords {
            x *= unit.spacing().x;
            y *= unit.spacing().y;

            x += rng.gen_range(-1.0..=1.0);
            y += rng.gen_range(-1.0..=1.0);

            // x += translation.x;
            // y += translation.y;

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
