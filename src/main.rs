mod button;
mod buble;
mod story;
mod text;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::story::{Situation};

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Resource)]
struct NextSituation{
   next: bool,
    next_id: String,
    path: String,
}

#[derive(Component, Clone, PartialOrd, PartialEq)]
enum Buttons{
    Mitter,
    Left,
    Right,
    No
}

#[derive(PartialOrd, PartialEq)]
enum ButtonAction{
    Pressed,
    Hoverd,
    No
}

#[derive(Resource)]
struct Buttonstate{
    delay: usize,
    action: ButtonAction,
    button: Buttons,
    next: bool,

}

#[derive(Resource)]
struct GameState {
    dialogstage: u16,
    nextstage: NextSituation,
    buttons: Buttonstate
}

fn main() {
      App::new()
          .insert_resource(GameState{
              dialogstage: 0,
                nextstage: NextSituation{
                    next: true,
                    next_id: "question_000".to_string(),
                    path: "story/output_jsons/".to_string()
                    //next_id: "shop_greating".to_string(),
                },
              buttons: Buttonstate{
                  delay: 20,
                  action: ButtonAction::No,
                  button: Buttons::No,
                  next: true,

              }
          })
          .insert_resource(Situation{..default()})
          //.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
          .add_plugins(WorldInspectorPlugin::new())
          .add_systems(Startup, setup)
          .add_systems(Startup, button::spawn)
          .add_systems(Startup, buble::spawn)
          .add_systems(Startup, text::spawn)
          .add_systems(Update, (story::next, button::status_update, text::update, text::update_info).chain())
          //.add_systems(Update, check)

          .run();
}

fn setup(
     mut commands: Commands,
){
      commands.spawn(Camera2d);
}


fn check(
    situation: ResMut<Situation>
) {
   dbg!(situation);
}

