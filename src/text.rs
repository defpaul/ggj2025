use bevy::prelude::*;
use bevy::color::palettes::css::*;
use bevy::sprite::Anchor;
use crate::{ButtonAction, Buttons, GameState};
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

#[derive(Component, PartialOrd, PartialEq)]
pub enum TextFunktion{
    Npc,
    Player,
    Info
}

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>
){

    let text_justify = JustifyText::Center;
    let archer = Anchor::TopLeft;
    let text_color = Color::Srgba(BLACK);
    let text_color_info = Color::Srgba(ALICE_BLUE);

    let font_nice = asset_server.load("fonts/ComicShannsMonoNerdFont-Regular.otf");
    let text_font = TextFont{
        font: font_nice.clone(),
        font_size: 30.0,
        ..default()
    };

    let text_font_info = TextFont{
        font: font_nice.clone(),
        font_size: 50.0,
        ..default()
    };


   commands.spawn((
       Text2d::new("test"),
       text_font.clone(),
       Transform{
           translation: Vec3::new(-800.0, 350.0, 2.0),
           ..default()
       },
       TextColor::from(text_color),
       TextLayout::new_with_justify(text_justify),
       TextStruckt{person: Npc },
       archer,
   ));
    commands.spawn((
        Text2d::new("test player"),
        text_font.clone(),
        Transform{
            translation: Vec3::new(200.0, 200.0, 2.0),
            ..default()
        },
        TextColor::from(text_color),
        TextLayout::new_with_justify(text_justify),
        TextStruckt{person: Player },
        archer,
    ));


    commands.spawn((
        Text2d::new("test"),
        text_font_info.clone(),
        Transform{
            translation: Vec3::new(0.0, 450.0, 2.0),
            ..default()
        },
        TextColor::from(text_color_info),
        TextLayout::new_with_justify(text_justify),
        TextFunktion::Info,
        Anchor::TopCenter,
    ));
    commands.spawn((
        Text2d::new("you"),
        text_font_info.clone(),
        Transform{
            translation: Vec3::new(770.0, -120.0, 2.0),
            ..default()
        },
        TextColor::from(text_color_info),
        TextLayout::new_with_justify(text_justify),
        TextFunktion::Player,
        archer,
    ));
    commands.spawn((
        Text2d::new("test"),
        text_font_info.clone(),
        Transform{
            translation: Vec3::new( -850.0, 0.0, 2.0),
            ..default()
        },
        TextColor::from(text_color_info),
        TextLayout::new_with_justify(text_justify),
        TextFunktion::Npc,
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


    if !game_state.nextstage.next {

        if max_dialog == 1 {
            game_state.buttons.next = false;
            text_npc = situation.dialog[dialog_state].text.clone();
            if game_state.buttons.action == ButtonAction::Hoverd {
                if game_state.buttons.button == Buttons::Right {
                    text_player = situation.answers[0].text.clone()
                }
                if game_state.buttons.button == Buttons::Mitter {
                    text_player = situation.answers[1].text.clone()
                }
                if game_state.buttons.button == Buttons::Left {
                    text_player = situation.answers[2].text.clone()
                }
            }
        } else if dialog_state == max_dialog-1{
            game_state.buttons.next = false;
            text_npc = situation.dialog[dialog_state].text.clone();
            if game_state.buttons.action == ButtonAction::Hoverd{
                if game_state.buttons.button == Buttons::Right {
                    text_player = situation.answers[0].text.clone()
                }
                if game_state.buttons.button == Buttons::Mitter {
                    text_player = situation.answers[1].text.clone()
                }
                if game_state.buttons.button == Buttons::Left {
                    text_player = situation.answers[2].text.clone()
                }
            }



        }else if dialog_state == 0 {

            if situation.dialog[dialog_state].talker == "npc".to_string(){
               text_npc = situation.dialog[dialog_state].text.clone()
             }
            if situation.dialog[dialog_state].talker == "player".to_string(){
                text_player = situation.dialog[dialog_state].text.clone()
            }

        }else if dialog_state > 0 {
            let text = situation.dialog[dialog_state].talker.clone();
            let mut text_bevor = situation.dialog[dialog_state-1].talker.clone();

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


}


pub fn update_info(

    mut game_state: ResMut<GameState> ,
    situation: Res<Situation>,
    mut q_text: Query<(&mut Text2d, &TextFunktion)>,
    
){
    if !game_state.nextstage.next {
        for (mut text, fuktion) in &mut q_text {
            if *fuktion == TextFunktion::Player {
                continue
            }else if *fuktion == TextFunktion::Info {
                text.0 = situation.place.clone()
            }else if *fuktion == TextFunktion::Npc {
                text.0 = situation.person.clone()
            }
        } 
        
    }
}