use bevy::prelude::*;
use rand::Rng;

use crate::battle::{
    layout::SquadSlot,
    units::{squad::Squad, Team},
};

use super::items::{
    ItemDescription, ItemLevel, ItemMaxCopies, ItemRarity, ItemRequirement, ItemRequirements,
};

#[derive(Resource)]
pub struct NumItemChoices(pub usize);

#[derive(Resource, Default)]
pub struct ItemChoices(pub Vec<ItemChoice>);

#[derive(Clone)]
pub struct ItemChoice {
    pub entity: Entity,
    pub name: String,
    pub description: String,
    pub image: Handle<Image>,
    pub copies: usize,
    pub rarity: ItemRarity,
    pub level: ItemLevel,
}

pub fn set_item_choices(
    mut choices: ResMut<ItemChoices>,
    num_choices: Res<NumItemChoices>,
    items: Query<(
        Entity,
        &Name,
        &ItemMaxCopies,
        &ItemDescription,
        &Handle<Image>,
        &ItemRarity,
        &ItemLevel,
        &ItemRequirements,
    )>,
    open_slots: Query<(&SquadSlot, &Team), Without<Squad>>,
) {
    let mut rng = rand::thread_rng();

    let open_slots = open_slots
        .iter()
        .filter(|(_, team)| **team == Team::Player)
        .count();

    // Create a weighted list valid item choices
    let mut weighted_items = vec![];

    for (ent, name, copies, description, image, rarity, level, requirements) in items.iter() {
        if copies.0 == 0 {
            continue;
        }

        if requirements.0.iter().any(|req| match req {
            ItemRequirement::OpenSlot => open_slots == 0,
        }) {
            continue;
        }

        for _ in 0..rarity.weight() {
            weighted_items.push(ItemChoice {
                entity: ent,
                name: name.to_string(),
                description: description.0.clone(),
                image: image.clone(),
                copies: copies.0,
                rarity: rarity.clone(),
                level: level.clone(),
            });
        }
    }

    // Randomly select items from the weighted list
    choices.0 = (0..num_choices.0)
        .map(|_| rng.gen_range(0..weighted_items.len()))
        .map(|i| weighted_items[i].clone())
        .collect();
}