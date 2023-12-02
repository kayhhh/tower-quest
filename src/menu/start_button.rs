use bevy::prelude::*;

const TEXT_COLOR: Color = Color::hsl(140.0, 0.2, 0.15);

#[derive(Component)]
pub struct StartButton;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // commands.spawn(SpriteBundle {
    //     texture: asset_server.load("sprites/StartButton.png"),
    //     transform: Transform::from_scale(Vec3::new(4.0, 4.0, 1.0)),
    //     ..default()
    // });

    let font = asset_server.load("font/vt323.ttf");

    commands
        .spawn((
            StartButton,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        border: UiRect::all(Val::Px(4.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(24.0)),
                        ..default()
                    },
                    background_color: Color::hsl(140.0, 0.6, 0.4).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Start",
                        TextStyle {
                            font,
                            font_size: 52.0,
                            color: TEXT_COLOR,
                        },
                    ));
                });
        });
}

pub fn interaction(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<StartButton>)>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                info!("Start button pressed");
            }
            Interaction::Hovered => {
                info!("Start button hovered");
            }
            Interaction::None => {}
        }
    }
}
