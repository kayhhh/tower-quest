use battle::camera::CameraVelocity;
use bevy::{asset::AssetMetaCheck, core_pipeline::clear_color::ClearColorConfig, prelude::*};
use bevy_round_ui::prelude::RoundUiPlugin;
use bevy_xpbd_2d::{plugins::PhysicsPlugins, resources::Gravity};

mod battle;
mod menu;
mod music;
mod rewards;

pub fn start() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            PhysicsPlugins::default(),
            // PhysicsDebugPlugin::default(),
            RoundUiPlugin,
            battle::BattlePlugin,
            menu::MenuPlugin,
            music::MusicPlugin,
            rewards::RewardsPlugin,
        ))
        .init_resource::<Floor>()
        .insert_resource(Gravity(Vec2::ZERO))
        .add_state::<GameState>()
        .add_systems(Startup, setup)
        .run();
}

#[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum GameState {
    #[default]
    Menu,
    InitBattle,
    PreBattle,
    Battle,
    Victory,
    Defeat,
}

#[derive(Resource, Default)]
pub struct Floor(pub usize);

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::hex(menu::colors::BG_DARK).unwrap()),
            },
            transform: Transform {
                scale: Vec3::splat(0.5),
                ..default()
            },
            ..default()
        },
        CameraVelocity::default(),
    ));
}
