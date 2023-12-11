use bevy::prelude::*;

use super::{
    squad::{Unit, UnitType},
    Team,
};

#[derive(Default, Resource)]
pub struct UnitSprites {
    archer_enemy: Handle<Image>,
    archer_friendly: Handle<Image>,
    knight_enemy: Handle<Image>,
    knight_friendly: Handle<Image>,
}

pub fn load_sprites(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(UnitSprites {
        archer_enemy: asset_server.load("images/units/KnightEnemy.png"),
        archer_friendly: asset_server.load("images/units/KnightFriendly.png"),
        knight_enemy: asset_server.load("images/units/KnightEnemy.png"),
        knight_friendly: asset_server.load("images/units/KnightFriendly.png"),
    });
}

pub fn spawn_sprites(
    mut commands: Commands,
    sprites: Res<UnitSprites>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut units: Query<
        (Entity, &UnitType, &Team, &Transform),
        (With<Unit>, Without<TextureAtlasSprite>),
    >,
) {
    for (ent, unit, team, transform) in units.iter_mut() {
        let atlas = match unit {
            UnitType::Archer => texture_atlases.add(TextureAtlas::from_grid(
                match team {
                    Team::Player => sprites.archer_friendly.clone(),
                    Team::Enemy => sprites.archer_enemy.clone(),
                },
                UnitType::Archer.sprite_size(),
                3,
                1,
                None,
                None,
            )),

            UnitType::Knight => texture_atlases.add(TextureAtlas::from_grid(
                match team {
                    Team::Player => sprites.knight_friendly.clone(),
                    Team::Enemy => sprites.knight_enemy.clone(),
                },
                UnitType::Knight.sprite_size(),
                3,
                1,
                None,
                None,
            )),
        };

        let sprite = TextureAtlasSprite {
            index: 0,
            flip_x: transform.translation.x > 0.0,
            ..default()
        };

        commands.entity(ent).insert((sprite, atlas));
    }
}
