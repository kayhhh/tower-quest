use bevy::prelude::*;

const NORMAL_BG: Color = Color::hsl(140.0, 0.6, 0.45);
const NORMAL_BORDER: Color = Color::hsl(140.0, 0.6, 0.25);
const NORMAL_TEXT: Color = Color::rgb(0.0, 0.0, 0.0);

const HOVERED_BG: Color = Color::hsl(140.0, 0.6, 0.55);
const HOVERED_BORDER: Color = Color::hsl(140.0, 0.6, 0.35);
const HOVERED_TEXT: Color = Color::rgb(0.0, 0.0, 0.0);

const PRESSED_BG: Color = Color::hsl(140.0, 0.6, 0.65);
const PRESSED_BORDER: Color = Color::hsl(140.0, 0.6, 0.45);
const PRESSED_TEXT: Color = Color::rgb(0.0, 0.0, 0.0);

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // Start button
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
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
                    border_color: BorderColor(NORMAL_BORDER),
                    background_color: NORMAL_BG.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Start Game",
                        TextStyle {
                            font_size: 40.0,
                            color: NORMAL_TEXT,
                            ..default()
                        },
                    ));
                });
        });
}

pub fn update(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();

        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BG.into();
                border_color.0 = PRESSED_BORDER;
                text.sections[0].style.color = PRESSED_TEXT;
            }
            Interaction::Hovered => {
                *color = HOVERED_BG.into();
                border_color.0 = HOVERED_BORDER;
                text.sections[0].style.color = HOVERED_TEXT;
            }
            Interaction::None => {
                *color = NORMAL_BG.into();
                border_color.0 = NORMAL_BORDER;
                text.sections[0].style.color = NORMAL_TEXT;
            }
        }
    }
}
