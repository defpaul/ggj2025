use bevy::prelude::*;
use std::fs;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Dialog{
    speker: String,
    text: String
}


#[derive(Serialize, Deserialize)]
struct Anser{
    short: String,
    long: String,
    next: String
}

#[derive(Serialize, Deserialize)]
struct Situation{
    speker: String,
    dialog: Vec<String>,
    dialog_stage: u16,
    anser: [Anser; 3]
}

fn pars_situation(
    next_situation: &str,
){

    let file = fs::read_to_string("story.json")
        .expect("culd not open");

    let json: serde_json::Value = serde_json::from_str(&file)
        .expect("some err");

    let situation = json.get(next_situation)
        .expect("some other err");
}