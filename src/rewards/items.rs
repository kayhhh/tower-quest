use bevy::prelude::*;
use rand::Rng;

use crate::battle::units::squad::{SquadBundle, SquadCount, UnitType};

use super::effects::ItemEffect;

#[derive(Component, Clone, Default)]
pub enum ItemRarity {
    #[default]
    Common,
    Rare,
    Epic,
    Legendary,
}

impl ItemRarity {
    pub fn weight(&self) -> usize {
        match self {
            ItemRarity::Common => 6,
            ItemRarity::Rare => 4,
            ItemRarity::Epic => 2,
            ItemRarity::Legendary => 1,
        }
    }
}

#[derive(Component)]
pub struct ItemCopies(pub usize);

impl Default for ItemCopies {
    fn default() -> Self {
        Self(1)
    }
}

#[derive(Component, Clone)]
pub struct ItemLevel {
    pub level: usize,
    pub max_level: usize,
}

impl ItemLevel {
    pub fn new(max_level: usize) -> Self {
        Self {
            max_level,
            ..default()
        }
    }
}

impl Default for ItemLevel {
    fn default() -> Self {
        Self {
            level: 1,
            max_level: 1,
        }
    }
}

#[derive(Component, Default)]
pub struct ItemDescription(pub String);

#[derive(Bundle)]
pub struct ItemBundle {
    copies: ItemCopies,
    description: ItemDescription,
    image: Handle<Image>,
    level: ItemLevel,
    name: Name,
    rarity: ItemRarity,
    effect: ItemEffect,
}

pub fn init_items(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(ItemBundle {
        copies: ItemCopies(1),
        description: ItemDescription("+25% movement speed".to_string()),
        effect: ItemEffect::AddMovementSpeed(0.25),
        image: asset_server.load("images/items/Coffee.png"),
        level: ItemLevel::new(3),
        name: Name::new("Coffee"),
        rarity: ItemRarity::Rare,
    });

    commands.spawn(ItemBundle {
        copies: ItemCopies(6),
        description: ItemDescription("+1 knight squad".to_string()),
        effect: ItemEffect::AddSquad(SquadBundle {
            unit: UnitType::Knight,
            count: SquadCount(10),
            ..default()
        }),
        image: asset_server.load("images/items/KnightItem.png"),
        name: Name::new("Knight Squad"),
        level: ItemLevel::default(),
        rarity: ItemRarity::Common,
    });
}

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

/// Generate a random list of item choices
pub fn gen_item_choices(items: Vec<ItemChoice>) -> Vec<ItemChoice> {
    let num_choices = 3;

    let mut rng = rand::thread_rng();

    // Create a list of all items, weighted by rarity
    let mut weighted_items = vec![];

    for item in &items {
        if item.copies == 0 {
            continue;
        }

        for _ in 0..item.rarity.weight() {
            weighted_items.push(item);
        }
    }

    // Randomly select items from the weighted list
    let indices = (0..num_choices)
        .map(|_| rng.gen_range(0..weighted_items.len()))
        .collect::<Vec<_>>();

    indices
        .iter()
        .map(|&i| weighted_items[i].clone())
        .collect::<Vec<_>>()
}
