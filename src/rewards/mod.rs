use bevy::prelude::*;
use bevy_round_ui::{
    autosize::{RoundUiAutosizeNode, RoundUiAutosizeNodePadding},
    prelude::{RoundUiBorder, RoundUiMaterial},
};
use rand::Rng;

use crate::{battle::units::Team, menu::colors, GameState};

use self::{
    button::{activate_item_effect, ItemCard, ItemCardStyle, ItemSelect},
    choices::{EnemyItemChoices, FriendlyItemChoices, ItemChoice, NumItemChoices},
    effects::{
        AddColumn, AddMovementSpeed, AddRow, AddSquad, EnemySpeedModifier, FriendlySpeedModifier,
        ItemEffect,
    },
};

mod button;
pub mod choices;
pub mod effects;
pub mod items;

pub struct RewardsPlugin;

impl Plugin for RewardsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(effects::EffectsPlugin)
            .init_resource::<EnemyItemChoices>()
            .init_resource::<EnemySpeedModifier>()
            .init_resource::<FriendlyItemChoices>()
            .init_resource::<FriendlySpeedModifier>()
            .init_resource::<ItemCardStyle>()
            .init_resource::<NumItemChoices>()
            .add_systems(Startup, items::init_items)
            .add_systems(OnEnter(GameState::InitBattle), init_resources)
            .add_systems(
                Update,
                (button::handle_interactions, button::handle_item_select),
            )
            .add_systems(
                OnEnter(GameState::Victory),
                (choices::set_item_choices, (setup_rewards, upgrade_enemy)).chain(),
            )
            .add_systems(OnExit(GameState::Victory), cleanup);
    }
}

fn init_resources(mut commands: Commands) {
    commands.insert_resource(EnemyItemChoices::default());
    commands.insert_resource(EnemySpeedModifier::default());
    commands.insert_resource(FriendlyItemChoices::default());
    commands.insert_resource(FriendlySpeedModifier::default());
    commands.insert_resource(NumItemChoices::default());
}

fn upgrade_enemy(
    choices: Res<EnemyItemChoices>,
    effects: Query<(Entity, &ItemEffect)>,
    mut add_movement_writer: EventWriter<AddMovementSpeed>,
    mut add_squad_writer: EventWriter<AddSquad>,
    mut add_column_writer: EventWriter<AddColumn>,
    mut add_row_writer: EventWriter<AddRow>,
) {
    let mut rng = rand::thread_rng();

    // Pick a random item from the choices
    let item = rng.gen_range(0..choices.0.len());
    let item = &choices.0[item];

    info!("Enemy chose item: {}", item.name);

    let effect = effects.get(item.entity).unwrap().1;

    activate_item_effect(
        effect,
        Team::Enemy,
        &mut add_movement_writer,
        &mut add_squad_writer,
        &mut add_column_writer,
        &mut add_row_writer,
    );
}

#[derive(Component)]
pub struct VictoryMenu;

pub fn setup_rewards(
    mut commands: Commands,
    button_style: Res<ItemCardStyle>,
    mut materials: ResMut<Assets<RoundUiMaterial>>,
    asset_server: Res<AssetServer>,
    choices: Res<FriendlyItemChoices>,
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
                    .0
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
        _ => format!("{} {}", item.name, to_roman(item.level.level + 1)),
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
