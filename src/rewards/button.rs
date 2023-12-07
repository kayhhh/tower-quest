use bevy::prelude::*;
use bevy_round_ui::prelude::{RoundUiBorder, RoundUiMaterial, RoundUiOffset};
use rand::Rng;

use crate::{
    battle::{
        layout::SquadSlot,
        units::{squad::Squad, Team},
    },
    menu::colors,
    GameState,
};

use super::{
    effects::{ItemEffect, SpeedModifier},
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
) {
    for (interaction, mut material) in &mut interaction_query {
        *material = match *interaction {
            Interaction::Pressed => button_style.press.clone(),
            Interaction::Hovered => button_style.hover.clone(),
            Interaction::None => button_style.default.clone(),
        };
    }
}

#[derive(Component)]
pub struct ItemSelect(pub Entity);

pub fn handle_item_select(
    mut commands: Commands,
    interaction_query: Query<(&Interaction, &ItemSelect), Changed<Interaction>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut items: Query<(&Name, &mut ItemMaxCopies, &mut ItemLevel, &ItemEffect)>,
    mut speed_modified: ResMut<SpeedModifier>,
    open_slots: Query<(Entity, &Team), (With<SquadSlot>, Without<Squad>)>,
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

            match effect {
                ItemEffect::AddMovementSpeed(multiplier) => {
                    speed_modified.0 += multiplier;
                }
                ItemEffect::AddSquad(squad) => {
                    let open_slots = open_slots
                        .iter()
                        .filter(|(_, team)| **team == Team::Player)
                        .map(|(ent, _)| ent)
                        .collect::<Vec<_>>();

                    let count = open_slots.len();

                    if count == 0 {
                        error!("No open slots");
                        continue;
                    }

                    let mut rng = rand::thread_rng();
                    let slot = open_slots[rng.gen_range(0..count)];

                    commands.entity(slot).insert(squad.clone());
                }
            };

            next_state.set(GameState::Battle);
        }
    }
}
