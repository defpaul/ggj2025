mod button;
mod buble;
mod story;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::story::Situation;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Resource)]
struct NextSituation{
   next: bool,
    next_id: String
}

fn main() {
      App::new()
          .insert_resource(NextSituation{next:true, next_id: "shop_greating".to_string()})
          .insert_resource(Situation{..default()})
          .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
          .add_plugins(WorldInspectorPlugin::new())
          .add_systems(Startup, setup)
          .add_systems(Startup, button::spawn)
          .add_systems(Startup, buble::spawn)
          .add_systems(Update, button::status_update)
          .add_systems(Update, story::next)

          .run();
}

fn setup(
     mut commands: Commands,
){
      commands.spawn(Camera2d);
}


fn spawn_buble(
    commands: Commands,
    asset_server: AssetServer
) {
}