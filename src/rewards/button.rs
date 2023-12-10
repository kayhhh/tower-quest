use bevy::prelude::*;
use bevy_round_ui::prelude::{RoundUiBorder, RoundUiMaterial, RoundUiOffset};

use crate::{
    battle::units::Team,
    menu::{
        colors,
        sounds::{HoverSound, SelectSound},
    },
    GameState,
};

use super::{
    effects::{AddColumn, AddMovementSpeed, AddRow, AddSquad, ItemEffect},
    items::{ItemLevel, ItemMaxCopies},
};

#[derive(Component)]
pub struct ItemCard;

#[derive(Resource)]
pub struct ItemCardStyle {
    pub width: f32,
    pub height: f32,
    pub default: Handle<RoundUiMaterial>,
    pub hover: Handle<RoundUiMaterial>,
    pub press: Handle<RoundUiMaterial>,
}

impl FromWorld for ItemCardStyle {
    fn from_world(world: &mut World) -> Self {
        let cell = world.cell();
        let mut materials = cell
            .get_resource_mut::<Assets<RoundUiMaterial>>()
            .expect("Failed to get Assets<RoundUiMaterial>");

        let width = 200.0;
        let height = 250.0;
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
        (Changed<Interaction>, With<ItemCard>),
    >,
    button_style: Res<ItemCardStyle>,
    mut hover_sound: EventWriter<HoverSound>,
    mut select_sound: EventWriter<SelectSound>,
) {
    for (interaction, mut material) in &mut interaction_query {
        *material = match *interaction {
            Interaction::Pressed => {
                select_sound.send_default();
                button_style.press.clone()
            }
            Interaction::Hovered => {
                hover_sound.send_default();
                button_style.hover.clone()
            }
            Interaction::None => button_style.default.clone(),
        };
    }
}

#[derive(Component)]
pub struct ItemSelect(pub Entity);

pub fn handle_item_select(
    interaction_query: Query<(&Interaction, &ItemSelect), Changed<Interaction>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut items: Query<(&Name, &mut ItemMaxCopies, &mut ItemLevel, &ItemEffect)>,
    mut add_movement_writer: EventWriter<AddMovementSpeed>,
    mut add_squad_writer: EventWriter<AddSquad>,
    mut add_column_writer: EventWriter<AddColumn>,
    mut add_row_writer: EventWriter<AddRow>,
) {
    for (interaction, action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            let (name, mut copies, mut level, effect) = match items.get_mut(action.0) {
                Ok(item) => item,
                Err(_) => {
                    error!("Failed to get item");
                    continue;
                }
            };

            info!("Item selected: {}", name);

            level.level += 1;

            if level.level >= level.max_level {
                copies.0 -= 1;
            }

            activate_item_effect(
                effect,
                Team::Player,
                &mut add_movement_writer,
                &mut add_squad_writer,
                &mut add_column_writer,
                &mut add_row_writer,
            );

            next_state.set(GameState::PreBattle);
        }
    }
}

pub fn activate_item_effect(
    effect: &ItemEffect,
    team: Team,
    add_movement_writer: &mut EventWriter<AddMovementSpeed>,
    add_squad_writer: &mut EventWriter<AddSquad>,
    add_column_writer: &mut EventWriter<AddColumn>,
    add_row_writer: &mut EventWriter<AddRow>,
) {
    match effect {
        ItemEffect::AddMovementSpeed(multiplier) => {
            add_movement_writer.send(AddMovementSpeed(*multiplier));
        }
        ItemEffect::AddSquad(squad) => {
            add_squad_writer.send(AddSquad {
                squad: squad.clone(),
                team,
            });
        }
        ItemEffect::AddColumn => {
            add_column_writer.send(AddColumn { team });
        }
        ItemEffect::AddRow => {
            add_row_writer.send(AddRow { team });
        }
    };
}
