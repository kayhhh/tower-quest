use bevy::prelude::*;
use bevy_round_ui::prelude::RoundUiPlugin;

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
