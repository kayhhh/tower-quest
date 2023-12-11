use bevy::prelude::*;
use rand::Rng;

use crate::rewards::effects::FriendlyKnightSquadSizeModifier;

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
    pub unit: UnitType,
}

pub fn spawn_units(
    mut commands: Commands,
    mut squads: Query<(Entity, &Formation, &Team, &SquadCount, &UnitType), With<Squad>>,
    friendly_squad_size_modifier: Res<FriendlyKnightSquadSizeModifier>,
    enemy_squad_size_modifier: Res<FriendlyKnightSquadSizeModifier>,
) {
    let mut rng = rand::thread_rng();

    for (ent, formation, team, count, unit) in squads.iter_mut() {
        let modifier = match team {
            Team::Player => &friendly_squad_size_modifier.0,
            Team::Enemy => &enemy_squad_size_modifier.0,
        };

        let count = (count.0 as f32 * modifier.0) as usize;
        let coords = formation.coords(count);

        for (mut x, mut y) in coords {
            x *= unit.spacing().x;
            y *= unit.spacing().y;

            x += rng.gen_range(-1.0..=1.0);
            y += rng.gen_range(-1.0..=1.0);

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
