use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use bevy_round_ui::prelude::RoundUiPlugin;
use zoom::Zoom;

mod battle;
mod menu;
mod zoom;

pub fn start() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            RoundUiPlugin,
            battle::BattlePlugin,
            menu::MenuPlugin,
            zoom::ZoomPlugin,
        ))
        .add_state::<GameState>()
        .add_systems(Startup, setup)
        .run();
}

#[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum GameState {
    #[default]
    Menu,
    Prep,
    Battle,
    GameOver,
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::hex(menu::colors::DARK).unwrap()),
            },
            ..default()
        },
        Zoom::default(),
    ));
}
