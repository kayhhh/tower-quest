use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};

use crate::GameState;

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_music)
            .add_systems(OnEnter(GameState::Menu), play_downtime)
            .add_systems(OnEnter(GameState::InitBattle), play_intro)
            .add_systems(
                OnEnter(GameState::Battle),
                (unpause_battle, despawn_downtime),
            )
            .add_systems(Update, transition_intro)
            .add_systems(
                OnExit(GameState::Battle),
                (pause_battle, play_downtime).chain(),
            )
            .add_systems(OnEnter(GameState::Defeat), remove_battle);
    }
}

#[derive(Resource)]
struct MusicSources {
    pub battle_intro: Handle<AudioSource>,
    pub battle: Handle<AudioSource>,
    pub downtime: Handle<AudioSource>,
}

fn load_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(MusicSources {
        battle_intro: asset_server.load("sounds/groove_battle_intro.ogg"),
        battle: asset_server.load("sounds/groove_battle_main.ogg"),
        downtime: asset_server.load("sounds/groove_downtime.ogg"),
    });
}

#[derive(Component)]
pub struct MusicDowntime;

#[derive(Component)]
pub struct MusicBattle;

fn play_downtime(mut commands: Commands, music: Res<MusicSources>) {
    commands.spawn((
        AudioBundle {
            source: music.downtime.clone(),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                volume: Volume::new_relative(0.7),
                ..default()
            },
        },
        MusicDowntime,
    ));
}

fn despawn_downtime(mut commands: Commands, sources: Query<Entity, With<MusicDowntime>>) {
    for ent in sources.iter() {
        commands.entity(ent).despawn_recursive();
    }
}

#[derive(Component)]
pub struct IntroTimer(pub Timer);

fn play_intro(mut commands: Commands, music: Res<MusicSources>) {
    commands.spawn((
        AudioBundle {
            source: music.battle_intro.clone(),
            settings: PlaybackSettings {
                mode: PlaybackMode::Once,
                volume: Volume::new_relative(0.7),
                ..default()
            },
        },
        IntroTimer(Timer::from_seconds(9.58, TimerMode::Once)),
    ));
}

fn transition_intro(
    mut commands: Commands,
    sources: Res<MusicSources>,
    time: Res<Time>,
    mut intro: Query<(Entity, &mut IntroTimer)>,
) {
    for (ent, mut timer) in intro.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            commands.entity(ent).despawn_recursive();
            commands.spawn((
                AudioBundle {
                    source: sources.battle.clone(),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Loop,
                        volume: Volume::new_relative(0.7),
                        ..default()
                    },
                },
                MusicBattle,
            ));
        }
    }
}

fn pause_battle(sources: Query<&AudioSink, With<MusicBattle>>) {
    for sink in sources.iter() {
        sink.pause();
    }
}

fn unpause_battle(sources: Query<&AudioSink, With<MusicBattle>>) {
    for sink in sources.iter() {
        sink.play();
    }
}

fn remove_battle(mut commands: Commands, sources: Query<Entity, With<MusicBattle>>) {
    for ent in sources.iter() {
        commands.entity(ent).despawn_recursive();
    }
}
