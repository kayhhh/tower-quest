use bevy::prelude::*;
use rand::Rng;

#[derive(Clone)]
pub enum ItemRarity {
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

#[derive(Clone)]
pub struct ItemMetadata {
    pub name: String,
    pub description: String,
    pub sprite: Handle<Image>,
    pub rarity: ItemRarity,
}

#[derive(Resource)]
pub struct Items {
    pub coffee: ItemMetadata,
    pub knights: ItemMetadata,
}

pub fn load_item_sprites(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Items {
        coffee: ItemMetadata {
            name: "Coffee".to_string(),
            description: "+25% movement speed".to_string(),
            sprite: asset_server.load("sprites/Coffee.png"),
            rarity: ItemRarity::Rare,
        },
        knights: ItemMetadata {
            name: "Knights".to_string(),
            description: "+10 knights".to_string(),
            sprite: asset_server.load("sprites/KnightItem.png"),
            rarity: ItemRarity::Common,
        },
    });
}

/// Generate a random list of item choices
pub fn gen_item_choices(items: &Items) -> Vec<ItemMetadata> {
    let num_choices = 3;

    let mut rng = rand::thread_rng();

    let items = vec![&items.coffee, &items.knights];

    // Create a list of all items, weighted by rarity
    let mut weighted_items = vec![];

    for item in items {
        for _ in 0..item.rarity.weight() {
            weighted_items.push(item.clone());
        }
    }

    // Randomly select items from the weighted list
    let indices = (0..num_choices)
        .map(|_| rng.gen_range(0..weighted_items.len()))
        .collect::<Vec<_>>();

    weighted_items
        .iter()
        .enumerate()
        .filter(|(i, _)| indices.contains(i))
        .map(|(_, item)| item.clone())
        .collect()
}
