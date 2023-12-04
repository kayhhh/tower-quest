use bevy::prelude::*;

use super::{Health, Team};

#[derive(Component, Clone, Default)]
pub struct MovementSpeed(pub f32);

#[derive(Component)]
pub enum Movement {
    WithinRange { target: Vec3, range: f32 },
}

#[derive(Component, Clone, Default)]
pub struct AttackDamage(pub f32);

#[derive(Component, Clone, Default)]
pub struct AttackRange(pub f32);

#[derive(Component)]
pub struct AttackTarget(pub Entity);

#[derive(Component, Clone, Default)]
pub struct AttackCooldown(pub f32);

#[derive(Component)]
pub struct LastAttackTime(pub f32);

#[derive(Component)]
pub struct Dead;

pub fn set_target(
    mut commands: Commands,
    units: Query<(Entity, &Team, &Transform, &AttackRange), Without<Dead>>,
) {
    for (ent, team, transform, range) in units.iter() {
        let nearest_enemy = units
            .iter()
            .filter(|(_, t, _, _)| **t != *team)
            .map(|(e, _, t, _)| (e, t, transform.translation.distance(t.translation)))
            .min_by(|(_, _, d1), (_, _, d2)| d1.partial_cmp(d2).unwrap());

        let (target_ent, target_translation) = match nearest_enemy {
            Some((e, t, _)) => (e, t.translation),
            None => {
                commands.entity(ent).remove::<AttackTarget>();
                continue;
            }
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

pub fn move_units(
    time: Res<Time>,
    mut units: Query<(&mut Transform, &Movement, &MovementSpeed), Without<Dead>>,
) {
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

pub fn attack(
    mut commands: Commands,
    time: Res<Time>,
    attackers: Query<
        (
            Entity,
            &AttackRange,
            &AttackTarget,
            &AttackDamage,
            &AttackCooldown,
            Option<&LastAttackTime>,
        ),
        Without<Dead>,
    >,
    mut healths: Query<&mut Health>,
    transforms: Query<&GlobalTransform>,
) {
    let now = time.elapsed_seconds();

    for (ent, range, target, damage, cooldown, last) in attackers.iter() {
        let translation = transforms.get(ent).unwrap().translation();
        let target_translation = transforms.get(target.0).unwrap().translation();
        let distance = translation.distance(target_translation);

        if distance > range.0 {
            continue;
        }

        let last_attack = match last {
            Some(last) => last.0,
            None => 0.0,
        };

        if now - last_attack < cooldown.0 {
            continue;
        }

        commands.entity(ent).insert(LastAttackTime(now));

        let mut health = match healths.get_mut(target.0) {
            Ok(health) => health,
            Err(_) => {
                error!("Target has no health component!");
                continue;
            }
        };

        if health.0 <= 0.0 {
            info!("Target already dead!");
            continue;
        }

        health.0 -= damage.0;

        if health.0 <= 0.0 {
            info!("Target dead!");

            commands
                .entity(target.0)
                .insert(Dead)
                .remove::<TextureAtlasSprite>();
        }
    }
}
