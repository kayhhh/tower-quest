use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use bevy_round_ui::prelude::RoundUiPlugin;
use postprocessing::PostProcessSettings;

mod menu;
mod postprocessing;

pub fn start() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            RoundUiPlugin,
            menu::MenuPlugin,
            postprocessing::PostProcessPlugin,
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
                clear_color: ClearColorConfig::Custom(Color::rgb(1.0, 1.0, 1.0)),
            },
            ..default()
        },
        PostProcessSettings::default(),
    ));
}
