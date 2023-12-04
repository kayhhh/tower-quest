use bevy::prelude::*;

use super::Team;

#[derive(Component, Clone, Default)]
pub struct AttackRange(pub f32);

#[derive(Component, Clone, Default)]
pub struct MovementSpeed(pub f32);

#[derive(Component)]
pub enum Movement {
    WithinRange { target: Vec3, range: f32 },
}

#[derive(Component)]
pub struct AttackTarget(pub Entity);

pub fn set_target(mut commands: Commands, units: Query<(Entity, &Team, &Transform, &AttackRange)>) {
    for (ent, team, transform, range) in units.iter() {
        let nearest_enemy = units
            .iter()
            .filter(|(_, t, _, _)| **t != *team)
            .map(|(e, _, t, _)| (e, t, transform.translation.distance(t.translation)))
            .min_by(|(_, _, d1), (_, _, d2)| d1.partial_cmp(d2).unwrap());

        let (target_ent, target_translation) = match nearest_enemy {
            Some((e, t, _)) => (e, t.translation),
            None => continue,
        };

        commands.entity(ent).insert((
            AttackTarget(target_ent),
            Movement::WithinRange {
                target: target_translation,
                range: range.0,
            },
        ));
    }
}

pub fn move_units(time: Res<Time>, mut units: Query<(&mut Transform, &Movement, &MovementSpeed)>) {
    for (mut transform, movement, speed) in units.iter_mut() {
        match movement {
            Movement::WithinRange { target, range } => {
                let distance = transform.translation.distance(*target);

                if distance <= *range {
                    continue;
                }

                let direction = *target - transform.translation;
                let direction = direction.normalize();

                let translation = direction * speed.0 * time.delta_seconds();

                transform.translation += translation;
            }
        }
    }
}
