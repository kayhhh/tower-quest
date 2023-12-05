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

pub fn flip_units(mut units: Query<(&mut TextureAtlasSprite, &LinearVelocity)>) {
    for (mut sprite, velocity) in units.iter_mut() {
        sprite.flip_x = velocity.x < 0.0;
    }
}
