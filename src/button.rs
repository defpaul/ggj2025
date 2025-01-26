use bevy::prelude::*;
use crate::{ButtonAction, Buttons, GameState};
use crate::story::Situation;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

const RED: Color = Color::srgb(1., 0.0, 0.0);

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands
        .spawn(Node {
            width: Val::Percent(50.0),
            height: Val::Percent(130.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                    Buttons::Left
                ))
                .with_child((
                    Text::new("Button"),
                    TextFont {
                        font: asset_server.load("fonts/ComicShannsMonoNerdFont-Regular.otf"),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
        });

    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(130.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                    Buttons::Mitter,
                ))
                .with_child((
                    Text::new("Button"),
                    TextFont {
                        font: asset_server.load("fonts/ComicShannsMonoNerdFont-Regular.otf"),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
        });

    commands
        .spawn(Node {
            width: Val::Percent(120.0),
            height: Val::Percent(130.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                    Buttons::Right
                ))
                .with_child((
                    Text::new("Button"),
                    TextFont {
                        font: asset_server.load("fonts/ComicShannsMonoNerdFont-Regular.otf"),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
        });

}


pub fn status_update(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
            &Buttons,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut game_state: ResMut<GameState>,
    situation: Res<Situation>
) {


    let mut pressed: bool = false;
    let mut hoverd: bool = false;

    let mut button_num = 0;

    let mut button:Buttons = Buttons::No;

    for (interaction, mut color, mut border_color, children, button_b) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();

        button_num += 1;


        match *interaction {
            Interaction::Pressed => {
                pressed = true;
                button = button_b.clone();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
                hoverd = true;
                button = button_b.clone();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }


        if game_state.buttons.next {
            **text = "Next".to_string()
        }else {
            if *button_b == Buttons::Right {
                **text = situation.answers[0].short.clone()
            }
            if *button_b == Buttons::Mitter {
                **text = situation.answers[1].short.clone()
            }
            if *button_b == Buttons::Left {
                **text = situation.answers[2].short.clone()
            }

        }


    }


    if !game_state.nextstage.next {
        if game_state.buttons.delay == 0 {
            if pressed && game_state.buttons.next{
                game_state.buttons.delay = 20;
                game_state.dialogstage += 1;
            }

            if !game_state.buttons.next {
                if hoverd{
                    game_state.buttons.action = ButtonAction::Hoverd;
                    game_state.buttons.button = button.clone()
                };
                if pressed {
                    game_state.buttons.action = ButtonAction::Pressed;
                    if button == Buttons::Right {
                        game_state.nextstage.next = true;
                        game_state.nextstage.next_id = situation.answers[0].next.clone()
                    }
                    if button == Buttons::Mitter {
                        game_state.nextstage.next = true;
                        game_state.nextstage.next_id = situation.answers[1].next.clone()
                    }
                    if button == Buttons::Left {
                        game_state.nextstage.next = true;
                        game_state.nextstage.next_id = situation.answers[2].next.clone()
                    }
                }
            };


        } else {
            game_state.buttons.delay -=1;
        }

    }

}