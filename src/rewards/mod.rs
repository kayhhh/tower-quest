use bevy::prelude::*;
use bevy_round_ui::prelude::{RoundUiBorder, RoundUiMaterial};

use crate::{
    menu::{
        button::{ButtonAction, ButtonStyle},
        colors, spawn_button,
    },
    GameState,
};

pub struct RewardsPlugin;

impl Plugin for RewardsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Victory), setup_rewards)
            .add_systems(OnExit(GameState::Victory), cleanup);
    }
}

#[derive(Component)]
pub struct VictoryMenu;

pub fn setup_rewards(
    mut commands: Commands,
    button_style: Res<ButtonStyle>,
    mut materials: ResMut<Assets<RoundUiMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("font/vt323.ttf");

    let panel_width = 400.0;
    let panel_height = 400.0;

    let panel_material = materials.add(RoundUiMaterial {
        background_color: Color::hex(colors::BG).unwrap(),
        border_radius: RoundUiBorder::all(20.0).into(),
        size: Vec2::new(panel_width, panel_height),
        ..default()
    });

    commands
        .spawn((
            VictoryMenu,
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
                p.spawn(NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        bottom: Val::Px(40.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|p| {
                    p.spawn(TextBundle::from_section(
                        "Victory!",
                        TextStyle {
                            color: Color::hex(colors::BG_LIGHT).unwrap(),
                            font_size: 44.0,
                            font: font.clone(),
                        },
                    ));
                });

                spawn_button(
                    p,
                    &button_style,
                    "Continue",
                    font.clone(),
                    ButtonAction::Start,
                );
            });
        });
}

pub fn cleanup(mut commands: Commands, query: Query<Entity, With<VictoryMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
