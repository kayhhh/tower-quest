use bevy::prelude::*;

use crate::{zoom::Zoom, GameState};

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Battle), setup)
            .add_systems(OnExit(GameState::Battle), cleanup);
    }
}

#[derive(Component)]
pub enum Unit {
    Archer,
    Knight,
}

#[derive(Component)]
pub enum Team {
    Player,
    Enemy,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut camera_zoom: Query<&mut Zoom, With<Camera>>,
) {
    info!("Battle setup");

    let mut camera_zoom = camera_zoom.single_mut();
    camera_zoom.zoom_level = 3.0;

    let texture_handle = asset_server.load("sprites/Knight.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(11.0, 8.0),
        3,
        1,
        Some(Vec2::splat(1.0)),
        Some(Vec2::splat(1.0)),
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        Unit::Knight,
        Team::Player,
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            ..default()
        },
    ));
}

fn cleanup(mut commands: Commands, units: Query<Entity, With<Unit>>) {
    info!("Battle cleanup");

    for ent in &mut units.iter() {
        commands.entity(ent).despawn_recursive();
    }
}
