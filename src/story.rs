use bevy::prelude::*;
use std::fs;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use crate::NextSituation;

#[derive(Serialize, Deserialize)]
pub struct Dialog{
    pub talker: String,
    pub text: String
}


#[derive(Serialize, Deserialize, Default)]
pub struct Anser{
    pub short: String,
    pub long: String,
    pub next: String
}

#[derive(Serialize, Deserialize, Resource, Default)]
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
    mut next_situation: ResMut<NextSituation>,
    mut situation: ResMut<Situation>,
){
    if next_situation.next {

        *situation= pars_situation(&next_situation.next_id);

        next_situation.next = false

    }
}