use bevy::prelude::*;

pub fn spawn(
   mut commands: Commands,
   asset_server: Res<AssetServer>
){

    commands.spawn((
        Sprite::from_image(asset_server.load("GGJ_25/bubble_02.png")),
        Transform{
            translation: Vec3::new(-500.0, 300.0, 1.0),
            scale: Vec3::splat(10.0),
            ..default()
        }
    ));


    commands.spawn((
        Sprite::from_image(asset_server.load("GGJ_25/bubble_03.png")),
        Transform{
            translation: Vec3::new(500.0, 50.0, 1.0),
            scale: Vec3::splat(10.0),
            ..default()
        }
    ));

}