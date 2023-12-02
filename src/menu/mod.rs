use bevy::prelude::*;

use crate::{postprocessing::PostProcessSettings, GameState};

mod start_button;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), (setup, start_button::setup))
            .add_systems(
                Update,
                start_button::update.run_if(in_state(GameState::Menu)),
            );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), PostProcessSettings::default()));
}
