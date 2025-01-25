use bevy::prelude::*;
use bevy::color::palettes::css::*;
use bevy::sprite::Anchor;
use crate::{GameState};
use crate::story::Situation;
use crate::text::Persons::{Npc, Player};

#[derive(Eq, PartialEq)]
enum Persons {
    Npc,
    Player,
    No,
}

#[derive(Component, Eq, PartialEq)]
pub struct TextStruckt{
    pub person: Persons
}

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>
){

    let text_justify = JustifyText::Center;
    let archer = Anchor::TopLeft;
    let text_color = Color::Srgba(BLACK);

    let font_nice = asset_server.load("fonts/ComicShannsMonoNerdFont-Regular.otf");
    let text_font = TextFont{
        font: font_nice.clone(),
        font_size: 30.0,
        ..default()
    };

   commands.spawn((
       Text2d::new("test"),
       text_font.clone(),
       Transform{
           translation: Vec3::new(-800.0, 450.0, 2.0),
           ..default()
       },
       TextColor::from(text_color),
       TextLayout::new_with_justify(text_justify),
       TextStruckt{person: Npc },
       archer,
   ));
    commands.spawn((
        Text2d::new("test player"),
        text_font,
        Transform{
            translation: Vec3::new(200.0, 200.0, 2.0),
            ..default()
        },
        TextColor::from(text_color),
        TextLayout::new_with_justify(text_justify),
        TextStruckt{person: Player },
        archer,
    ));
}


pub fn update (
   mut game_state: ResMut<GameState> ,
   situation: Res<Situation>,
   mut q_text: Query<(&mut Text2d, &TextStruckt)>,
){
    //game_state.dialogstage = 1;


    let dialog_state = game_state.dialogstage as usize;
    let max_dialog = situation.dialog.len() ;

    let mut text_npc: String = "".to_string();
    let mut text_player: String = "".to_string();


    if dialog_state == max_dialog {
        game_state.buttonnext = false;
        text_npc = situation.dialog[dialog_state].text.clone();
    }else if dialog_state >= 1 {
        let text = situation.dialog[dialog_state].text.clone();
        let text_bevor = situation.dialog[dialog_state-1].text.clone();
        if text != text_bevor {
            if situation.dialog[dialog_state].talker == "npc".to_string() {
                text_npc = situation.dialog[dialog_state].text.clone()
            }
            if situation.dialog[dialog_state].talker == "player".to_string() {
                text_player = situation.dialog[dialog_state].text.clone();
                text_npc = situation.dialog[dialog_state-1].text.clone()
            }

        } else if text == text_bevor {
            if situation.dialog[dialog_state].talker == "npc".to_string() {
                text_npc = situation.dialog[dialog_state].text.clone()
            }
            if situation.dialog[dialog_state].talker == "player".to_string() {
                text_player = situation.dialog[dialog_state].text.clone();
            }
        }
    } else if dialog_state == 0 {
        if situation.dialog[dialog_state].talker == "npc".to_string() {
            text_npc = situation.dialog[dialog_state].text.clone()
        }
        if situation.dialog[dialog_state].talker == "player".to_string() {
            text_player = situation.dialog[dialog_state].text.clone()
        }
    }

    for (mut text, id) in &mut q_text {
        if id.person == Persons::Player{
            text.0 = text_player.to_string()
        }
        if id.person == Persons::Npc {
            text.0 = text_npc.to_string()
        }
    }
}