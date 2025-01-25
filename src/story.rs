use bevy::prelude::*;
use std::fs;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use crate::{GameState, NextSituation};

#[derive(Serialize, Deserialize, Debug)]
pub struct Dialog{
    pub talker: String,
    pub text: String
}


#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Anser{
    pub short: String,
    pub long: String,
    pub next: String
}

#[derive(Serialize, Deserialize, Resource, Default, Debug)]
pub struct Situation{
    pub person: String,
    pub dialog: Vec<Dialog>,
    pub ansers: [Anser; 3]
}

fn pars_situation(
    next_situation: &str,
) -> Situation {

    let path = format!("{next_situation}.json");
    println!("{}", path);

    let file = fs::read_to_string(path)
        .expect("culd not open");

    let situation: Situation = serde_json::from_str(&file)
        .expect("some err");
    situation

}

pub fn next(
    mut game_state: ResMut<GameState>,
    mut situation: ResMut<Situation>,
){
    if game_state.nextstage.next {

        *situation= pars_situation(&game_state.nextstage.next_id);

        game_state.nextstage.next = false;
        game_state.buttonnext = true;

    }

}