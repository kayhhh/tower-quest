use bevy::{
    audio::{PlaybackMode, VolumeLevel},
    prelude::*,
};

pub struct SoundsPlugin;

impl Plugin for SoundsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SwingSound>()
            .add_event::<HitSound>()
            .add_event::<DeathSound>()
            .add_systems(Startup, load_sounds)
            .add_systems(
                Update,
                (play_swing_sounds, play_hit_sounds, play_death_sounds),
            );
    }
}

#[derive(Resource)]
pub struct Sounds {
    pub swing: Handle<AudioSource>,
    pub hit: Handle<AudioSource>,
    pub death: Handle<AudioSource>,
}

fn load_sounds(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Sounds {
        swing: asset_server.load("sounds/swing.ogg"),
        hit: asset_server.load("sounds/hit.ogg"),
        death: asset_server.load("sounds/death.ogg"),
    });
}

#[derive(Event, Default)]
pub struct SwingSound;

#[derive(Event, Default)]
pub struct HitSound;

#[derive(Event, Default)]
pub struct DeathSound;

fn play_swing_sounds(
    mut commands: Commands,
    sounds: Res<Sounds>,
    mut swing: EventReader<SwingSound>,
) {
    for _ in swing.read() {
        commands.spawn(AudioBundle {
            source: sounds.swing.clone(),
            settings: PlaybackSettings {
                volume: bevy::audio::Volume::Relative(VolumeLevel::new(0.4)),
                mode: PlaybackMode::Despawn,
                ..default()
            },
        });
    }
}

fn play_hit_sounds(mut commands: Commands, sounds: Res<Sounds>, mut hit: EventReader<HitSound>) {
    for _ in hit.read() {
        commands.spawn(AudioBundle {
            source: sounds.hit.clone(),
            settings: PlaybackSettings {
                volume: bevy::audio::Volume::Relative(VolumeLevel::new(0.5)),
                mode: PlaybackMode::Despawn,
                ..default()
            },
        });
    }
}

fn play_death_sounds(
    mut commands: Commands,
    sounds: Res<Sounds>,
    mut death: EventReader<DeathSound>,
) {
    for _ in death.read() {
        commands.spawn(AudioBundle {
            source: sounds.death.clone(),
            settings: PlaybackSettings {
                volume: bevy::audio::Volume::Relative(VolumeLevel::new(0.7)),
                mode: PlaybackMode::Despawn,
                ..default()
            },
        });
    }
}
