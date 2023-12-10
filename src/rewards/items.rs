use bevy::prelude::*;

use crate::battle::{
    layout::{INITIAL_COLUMNS, MAX_COLUMNS},
    units::squad::{SquadBundle, SquadCount, UnitType},
};

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
pub struct ItemMaxCopies(pub usize);

impl Default for ItemMaxCopies {
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
            level: 0,
            max_level: 1,
        }
    }
}

#[derive(Component, Default)]
pub struct ItemDescription(pub String);

pub enum ItemRequirement {
    OpenSlot,
}

#[derive(Component, Default)]
pub struct ItemRequirements(pub Vec<ItemRequirement>);

#[derive(Bundle)]
pub struct ItemBundle {
    copies: ItemMaxCopies,
    description: ItemDescription,
    image: Handle<Image>,
    level: ItemLevel,
    name: Name,
    rarity: ItemRarity,
    effect: ItemEffect,
    requirements: ItemRequirements,
}

pub fn init_items(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(ItemBundle {
        copies: ItemMaxCopies(1),
        description: ItemDescription("+25% movement speed".to_string()),
        effect: ItemEffect::AddMovementSpeed(0.25),
        image: asset_server.load("images/items/Coffee.png"),
        level: ItemLevel::new(3),
        name: Name::new("Coffee"),
        rarity: ItemRarity::Rare,
        requirements: ItemRequirements::default(),
    });

    commands.spawn(ItemBundle {
        copies: ItemMaxCopies(6),
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
        requirements: ItemRequirements(vec![ItemRequirement::OpenSlot]),
    });

    commands.spawn(ItemBundle {
        copies: ItemMaxCopies(MAX_COLUMNS - INITIAL_COLUMNS),
        description: ItemDescription("+1 column".to_string()),
        effect: ItemEffect::AddColumn,
        image: asset_server.load("images/items/AddColumn.png"),
        name: Name::new("Column"),
        level: ItemLevel::default(),
        rarity: ItemRarity::Epic,
        requirements: ItemRequirements::default(),
    });

    commands.spawn(ItemBundle {
        copies: ItemMaxCopies(1),
        description: ItemDescription("+1 row".to_string()),
        effect: ItemEffect::AddRow,
        image: asset_server.load("images/items/AddRow.png"),
        name: Name::new("Row"),
        level: ItemLevel::default(),
        rarity: ItemRarity::Rare,
        requirements: ItemRequirements::default(),
    });
}
