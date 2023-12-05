use bevy::prelude::*;
use bevy_xpbd_2d::components::LinearVelocity;

#[derive(Event)]
pub struct AttackEvent {
    pub attacker: Entity,
    pub target: Entity,
}

#[derive(Component)]
pub struct AnimationTimer(Timer);

#[derive(Component)]
pub struct AtlasAnimation {
    pub start: usize,
    pub end: usize,
}

pub fn animate_attack(mut commands: Commands, mut events: EventReader<AttackEvent>) {
    for event in events.read() {
        commands.entity(event.attacker).insert((
            AnimationTimer(Timer::from_seconds(0.25, TimerMode::Once)),
            AtlasAnimation { start: 0, end: 3 },
        ));
    }
}

pub fn animate_atlas(
    mut commands: Commands,
    time: Res<Time>,
    mut animations: Query<(
        Entity,
        &mut AnimationTimer,
        &AtlasAnimation,
        &mut TextureAtlasSprite,
    )>,
) {
    for (ent, mut timer, animation, mut sprite) in animations.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            sprite.index = animation.start;
            commands
                .entity(ent)
                .remove::<AnimationTimer>()
                .remove::<AtlasAnimation>();
            continue;
        }

        let num_frames = animation.end - animation.start;
        let frame = (timer.0.percent() * num_frames as f32) as usize;
        sprite.index = animation.start + frame;
    }
}

#[derive(Component)]
pub struct LastFlip(f32);

const FLIP_COOLDOWN: f32 = 0.1;

pub fn flip_units(
    mut commands: Commands,
    time: Res<Time>,
    mut units: Query<(
        Entity,
        &mut TextureAtlasSprite,
        &LinearVelocity,
        Option<&LastFlip>,
    )>,
) {
    for (ent, mut sprite, velocity, last) in units.iter_mut() {
        let new_flip = match velocity.x.total_cmp(&0.0) {
            std::cmp::Ordering::Greater => false,
            std::cmp::Ordering::Equal => sprite.flip_x,
            std::cmp::Ordering::Less => true,
        };

        if sprite.flip_x == new_flip {
            continue;
        }

        let now = time.elapsed_seconds();

        if let Some(last) = last {
            if now - last.0 < FLIP_COOLDOWN {
                continue;
            }
        }

        sprite.flip_x = new_flip;

        commands.entity(ent).insert(LastFlip(now));
    }
}
