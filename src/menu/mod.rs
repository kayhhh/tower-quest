use bevy::prelude::*;
use bevy_round_ui::{
    autosize::{RoundUiAutosizeNode, RoundUiAutosizeNodePadding},
    prelude::{RoundUiBorder, RoundUiMaterial, RoundUiOffset},
};

use crate::GameState;

use self::button::{ButtonAction, ButtonStyle, RoundButton};

mod button;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<button::ButtonStyle>()
            .add_systems(OnEnter(GameState::Menu), setup)
            .add_systems(OnExit(GameState::Menu), cleanup)
            .add_systems(
                Update,
                (
                    button::handle_button_actions,
                    button::handle_button_interactions,
                ),
            );
    }
}

#[derive(Component)]
struct Menu;

pub fn setup(
    mut commands: Commands,
    button_style: Res<button::ButtonStyle>,
    mut materials: ResMut<Assets<RoundUiMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("font/vt323.ttf");

    let panel_width = 400.0;
    let panel_height = 400.0;

    let panel_material = materials.add(RoundUiMaterial {
        background_color: Color::hex("5cb3af").unwrap(),
        border_color: Color::WHITE,
        border_radius: RoundUiBorder::all(20.0).into(),
        size: Vec2::new(panel_width, panel_height),
        offset: RoundUiOffset::all(6.0).into(),
    });

    commands
        .spawn((
            Menu,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|p| {
            p.spawn(MaterialNodeBundle {
                material: panel_material,
                style: Style {
                    width: Val::Px(panel_width),
                    height: Val::Px(panel_height),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            })
            .with_children(|p| {
                // Spawn the title
                p.spawn(NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::bottom(Val::Px(30.)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|p| {
                    p.spawn(TextBundle::from_section(
                        "TOWER QUEST",
                        TextStyle {
                            color: Color::WHITE,
                            font_size: 40.0,
                            font: font.clone(),
                        },
                    ));
                });

                spawn_button(p, &button_style, "Start", font.clone(), ButtonAction::Start);
                spawn_button(p, &button_style, "Quit", font.clone(), ButtonAction::Quit);
            });
        });
}

fn spawn_button(
    parent: &mut ChildBuilder,
    button_style: &Res<ButtonStyle>,
    text: impl Into<String>,
    font: Handle<Font>,
    extras: impl Bundle,
) -> Entity {
    parent
        .spawn((
            RoundButton,
            RoundUiAutosizeNode,
            RoundUiAutosizeNodePadding,
            MaterialNodeBundle {
                material: button_style.default.clone(),
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Px(button_style.width),
                    height: Val::Px(button_style.height),
                    margin: UiRect::bottom(Val::Px(10.)),
                    ..default()
                },
                ..default()
            },
            extras,
            Interaction::default(),
        ))
        .with_children(|p| {
            p.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    color: Color::WHITE,
                    font_size: 20.0,
                    font,
                },
            ));
        })
        .id()
}

fn cleanup(mut commands: Commands, menu: Query<Entity, With<Menu>>) {
    for ent in menu.iter() {
        commands.entity(ent).despawn_recursive();
    }
}
