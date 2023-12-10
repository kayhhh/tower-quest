use bevy::{audio::PlaybackMode, prelude::*};

#[derive(Resource)]
pub struct MenuSounds {
    pub hover: Handle<AudioSource>,
    pub select: Handle<AudioSource>,
}

pub fn load_sounds(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(MenuSounds {
        hover: asset_server.load("sounds/hover.ogg"),
        select: asset_server.load("sounds/select.ogg"),
    });
}

#[derive(Event, Default)]
pub struct HoverSound;

pub fn play_hover_sounds(
    mut commands: Commands,
    sounds: Res<MenuSounds>,
    mut select: EventReader<HoverSound>,
) {
    for _ in select.read() {
        commands.spawn(AudioBundle {
            source: sounds.hover.clone(),
            settings: PlaybackSettings {
                volume: bevy::audio::Volume::new_relative(0.5),
                mode: PlaybackMode::Despawn,
                ..default()
            },
        });
    }
}

#[derive(Event, Default)]
pub struct SelectSound;

pub fn play_select_sounds(
    mut commands: Commands,
    sounds: Res<MenuSounds>,
    mut select: EventReader<SelectSound>,
) {
    for _ in select.read() {
        commands.spawn(AudioBundle {
            source: sounds.select.clone(),
            settings: PlaybackSettings {
                volume: bevy::audio::Volume::new_relative(0.5),
                mode: PlaybackMode::Despawn,
                ..default()
            },
        });
    }
}
