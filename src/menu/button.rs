use bevy::{app::AppExit, prelude::*};
use bevy_round_ui::prelude::{RoundUiBorder, RoundUiMaterial, RoundUiOffset};

use crate::GameState;

use super::colors;

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
                background_color: Color::hex(colors::PRIMARY).unwrap(),
                border_color: Color::hex(colors::PRIMARY_DARK).unwrap(),
                border_radius: border_radius.into(),
                size: Vec2::new(width, height),
                offset: RoundUiOffset::bottom(offset).into(),
            }),
            hover: materials.add(RoundUiMaterial {
                background_color: Color::hex(colors::PRIMARY_LIGHT).unwrap(),
                border_color: Color::hex(colors::PRIMARY).unwrap(),
                border_radius: border_radius.into(),
                size: Vec2::new(width, height),
                offset: RoundUiOffset::bottom(offset).into(),
            }),
            press: materials.add(RoundUiMaterial {
                background_color: Color::hex(colors::PRIMARY_DARK).unwrap(),
                border_color: Color::NONE,
                border_radius: border_radius.into(),
                size: Vec2::new(width, height),
                offset: RoundUiOffset::top(offset).into(),
            }),
        }
    }
}

pub fn handle_interactions(
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

#[derive(Component)]
pub struct DeferredAction {
    pub action: ButtonAction,
    time: u128,
}

pub fn defer_actions(
    mut commands: Commands,
    time: Res<Time>,
    interaction_query: Query<(&Interaction, &ButtonAction), Changed<Interaction>>,
) {
    for (interaction, action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            info!("Button pressed: {:?}", action);

            let time = time.elapsed().as_millis();

            // Delay action so player can see the button press animation
            // (this sucks but whatever)
            match action {
                ButtonAction::Start => commands.spawn(DeferredAction {
                    action: ButtonAction::Start,
                    time,
                }),
                ButtonAction::Quit => commands.spawn(DeferredAction {
                    action: ButtonAction::Quit,
                    time,
                }),
            };
        }
    }
}

pub fn handle_actions(
    actions: Query<(&DeferredAction, Entity)>,
    time: Res<Time>,
    mut commands: Commands,
    mut app_exit_events: EventWriter<AppExit>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (deferred, entity) in &mut actions.iter() {
        let now = time.elapsed().as_millis();
        let elapsed = now - deferred.time;

        if elapsed < 80 {
            continue;
        }

        match deferred.action {
            ButtonAction::Start => next_state.set(GameState::Battle),
            ButtonAction::Quit => app_exit_events.send(AppExit),
        }

        info!("Button action complete: {:?}", deferred.action);

        commands.entity(entity).despawn();
    }
}
