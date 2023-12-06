use bevy::prelude::*;
use rand::Rng;

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
        Self(8)
    }
}

#[derive(Component, Default)]
pub struct ItemDescription(pub String);

#[derive(Bundle, Default)]
pub struct ItemBundle {
    copies: ItemCopies,
    description: ItemDescription,
    name: Name,
    rarity: ItemRarity,
    image: Handle<Image>,
}

pub fn load_item_sprites(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(ItemBundle {
        copies: ItemCopies(4),
        description: ItemDescription("+25% movement speed".to_string()),
        name: Name::new("Coffee"),
        rarity: ItemRarity::Rare,
        image: asset_server.load("sprites/Coffee.png"),
    });

    commands.spawn(ItemBundle {
        description: ItemDescription("+10 knights".to_string()),
        name: Name::new("Knights"),
        rarity: ItemRarity::Common,
        image: asset_server.load("sprites/KnightItem.png"),
        ..default()
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
