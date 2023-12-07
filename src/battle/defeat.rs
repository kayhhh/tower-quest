use bevy::prelude::*;
use bevy_round_ui::prelude::{RoundUiBorder, RoundUiMaterial};

use crate::{
    menu::{
        button::{self, ButtonAction},
        colors, spawn_button,
    },
    Floor,
};

#[derive(Component)]
pub struct ScoreMenu;

pub fn spawn_menu(
    mut commands: Commands,
    button_style: Res<button::ButtonStyle>,
    mut materials: ResMut<Assets<RoundUiMaterial>>,
    asset_server: Res<AssetServer>,
    floor: Res<Floor>,
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
            ScoreMenu,
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
                        flex_direction: FlexDirection::Column,
                        bottom: Val::Px(button_style.height),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|p| {
                    p.spawn(TextBundle::from_section(
                        "Game Over",
                        TextStyle {
                            color: Color::hex(colors::BG_LIGHT).unwrap(),
                            font_size: 44.0,
                            font: font.clone(),
                        },
                    ));

                    p.spawn(TextBundle::from_section(
                        format!("You reached floor {}", floor.0),
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
                    "New Game",
                    font.clone(),
                    ButtonAction::Start,
                );
                spawn_button(p, &button_style, "Quit", font.clone(), ButtonAction::Quit);
            });
        });
}

pub fn cleanup_menu(mut commands: Commands, menu: Query<Entity, With<ScoreMenu>>) {
    for ent in menu.iter() {
        commands.entity(ent).despawn_recursive();
    }
}
