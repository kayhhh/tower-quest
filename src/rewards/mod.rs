use bevy::prelude::*;
use bevy_round_ui::{
    autosize::{RoundUiAutosizeNode, RoundUiAutosizeNodePadding},
    prelude::{RoundUiBorder, RoundUiMaterial},
};

use crate::{menu::colors, GameState};

use self::{
    button::{ItemCard, ItemCardStyle, ItemSelect},
    effects::SpeedModifier,
    items::{gen_item_choices, ItemChoice, ItemCopies, ItemDescription, ItemLevel, ItemRarity},
};

mod button;
pub mod effects;
pub mod items;

pub struct RewardsPlugin;

impl Plugin for RewardsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ItemCardStyle>()
            .insert_resource(SpeedModifier(1.0))
            .add_systems(Startup, items::init_items)
            .add_systems(
                Update,
                (button::handle_interactions, button::handle_item_select),
            )
            .add_systems(OnEnter(GameState::Victory), setup_rewards)
            .add_systems(OnExit(GameState::Victory), cleanup);
    }
}

#[derive(Component)]
pub struct VictoryMenu;

pub fn setup_rewards(
    mut commands: Commands,
    button_style: Res<ItemCardStyle>,
    mut materials: ResMut<Assets<RoundUiMaterial>>,
    asset_server: Res<AssetServer>,
    items: Query<(
        Entity,
        &Name,
        &ItemCopies,
        &ItemDescription,
        &Handle<Image>,
        &ItemRarity,
        &ItemLevel,
    )>,
) {
    let font = asset_server.load("font/vt323.ttf");

    let panel_width = 800.0;
    let panel_height = 500.0;

    let panel_material = materials.add(RoundUiMaterial {
        background_color: Color::hex(colors::BG).unwrap(),
        border_radius: RoundUiBorder::all(20.0).into(),
        size: Vec2::new(panel_width, panel_height),
        ..default()
    });

    let items = items
        .iter()
        .map(
            |(ent, name, copies, description, image, rarity, level)| ItemChoice {
                entity: ent,
                name: name.to_string(),
                description: description.0.clone(),
                image: image.clone(),
                copies: copies.0,
                rarity: rarity.clone(),
                level: level.clone(),
            },
        )
        .collect::<Vec<_>>();

    let choices = gen_item_choices(items);

    commands
        .spawn((
            VictoryMenu,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|p| {
            p.spawn(NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
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

                p.spawn(TextBundle::from_section(
                    "Choose your reward.",
                    TextStyle {
                        color: Color::hex(colors::BG_LIGHT).unwrap(),
                        font_size: 24.0,
                        font: font.clone(),
                    },
                ));
            });

            p.spawn(MaterialNodeBundle {
                material: panel_material,
                style: Style {
                    width: Val::Px(panel_width),
                    height: Val::Px(panel_height),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceAround,
                    padding: UiRect::horizontal(Val::Px(40.0)),
                    ..default()
                },
                ..default()
            })
            .with_children(|p| {
                choices
                    .iter()
                    .for_each(|item| spawn_item_card(p, &button_style, font.clone(), item));
            });
        });
}

pub fn spawn_item_card(
    parent: &mut ChildBuilder,
    button_style: &Res<ItemCardStyle>,
    font: Handle<Font>,
    item: &ItemChoice,
) {
    let item_name = match item.level.max_level {
        1 => item.name.clone(),
        _ => format!("{} {}", item.name, to_roman(item.level.level)),
    };

    parent
        .spawn((
            ItemCard,
            ItemSelect(item.entity),
            RoundUiAutosizeNode,
            RoundUiAutosizeNodePadding,
            MaterialNodeBundle {
                material: button_style.default.clone(),
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    flex_direction: FlexDirection::Column,
                    width: Val::Px(button_style.width),
                    height: Val::Px(button_style.height),
                    padding: UiRect::vertical(Val::Px(60.0)),
                    ..default()
                },
                ..default()
            },
            Interaction::default(),
        ))
        .with_children(|p| {
            p.spawn(NodeBundle {
                style: Style {
                    margin: UiRect::vertical(Val::Px(10.0)),
                    ..default()
                },
                ..default()
            })
            .with_children(|p| {
                p.spawn(ImageBundle {
                    image: UiImage::new(item.image.clone()),
                    transform: Transform::from_scale(Vec3::splat(6.0)),
                    ..default()
                });
            });

            p.spawn(TextBundle::from_section(
                item_name,
                TextStyle {
                    color: Color::hex(colors::ACCENT).unwrap(),
                    font_size: 32.0,
                    font: font.clone(),
                },
            ));

            p.spawn(TextBundle::from_section(
                item.description.clone(),
                TextStyle {
                    color: Color::hex(colors::ACCENT).unwrap(),
                    font_size: 20.0,
                    font,
                },
            ));
        });
}

pub fn cleanup(mut commands: Commands, query: Query<Entity, With<VictoryMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn to_roman(num: usize) -> String {
    match num {
        1 => "I",
        2 => "II",
        3 => "III",
        4 => "IV",
        5 => "V",
        _ => "",
    }
    .to_string()
}
