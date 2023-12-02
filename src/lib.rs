use bevy::prelude::*;

mod menu;

pub fn start() {
    App::new()
        .add_plugins((DefaultPlugins, menu::MenuPlugin))
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
