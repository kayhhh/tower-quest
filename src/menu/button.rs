use bevy::{app::AppExit, prelude::*};
use bevy_round_ui::prelude::{RoundUiBorder, RoundUiMaterial, RoundUiOffset};

use crate::GameState;

#[derive(Component)]
pub struct RoundButton;

#[derive(Resource)]
pub struct ButtonStyle {
    pub width: f32,
    pub height: f32,
    pub default: Handle<RoundUiMaterial>,
    pub hover: Handle<RoundUiMaterial>,
    pub press: Handle<RoundUiMaterial>,
}

impl FromWorld for ButtonStyle {
    fn from_world(world: &mut World) -> Self {
        let cell = world.cell();
        let mut materials = cell
            .get_resource_mut::<Assets<RoundUiMaterial>>()
            .expect("Failed to get Assets<RoundRectMaterial>");

        let width = 200.0;
        let height = 40.0;
        let offset = 5.0;
        let border_radius = RoundUiBorder::all(15.0);

        Self {
            width,
            height,
            default: materials.add(RoundUiMaterial {
                background_color: Color::hex("#F76161").unwrap(),
                border_color: Color::hex("#A53A3D").unwrap(),
                border_radius: border_radius.into(),
                size: Vec2::new(width, height),
                offset: RoundUiOffset::bottom(offset).into(),
            }),
            hover: materials.add(RoundUiMaterial {
                background_color: Color::hex("#F61A39").unwrap(),
                border_color: Color::hex("#A0102A").unwrap(),
                border_radius: border_radius.into(),
                size: Vec2::new(width, height),
                offset: RoundUiOffset::bottom(offset).into(),
            }),
            press: materials.add(RoundUiMaterial {
                background_color: Color::hex("#A0102A").unwrap(),
                border_color: Color::NONE,
                border_radius: border_radius.into(),
                size: Vec2::new(width, height),
                offset: RoundUiOffset::top(offset).into(),
            }),
        }
    }
}

pub fn handle_button_interactions(
    mut interaction_query: Query<
        (&Interaction, &mut Handle<RoundUiMaterial>),
        (Changed<Interaction>, With<RoundButton>),
    >,
    button_style: Res<ButtonStyle>,
) {
    for (interaction, mut material) in &mut interaction_query {
        *material = match *interaction {
            Interaction::Pressed => button_style.press.clone(),
            Interaction::Hovered => button_style.hover.clone(),
            Interaction::None => button_style.default.clone(),
        };
    }
}

#[derive(Component, Debug)]
pub enum ButtonAction {
    Start,
    Quit,
}

pub fn handle_button_actions(
    interaction_query: Query<(&Interaction, &ButtonAction), Changed<Interaction>>,
    mut app_exit_events: EventWriter<AppExit>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            info!("Button pressed: {:?}", action);

            match action {
                ButtonAction::Start => next_state.set(GameState::Prep),
                ButtonAction::Quit => app_exit_events.send(AppExit),
            }
        }
    }
}
