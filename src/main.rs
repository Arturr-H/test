/*- Global allowings -*/
#![allow(
    non_snake_case,
    clippy::redundant_field_names
)]

/*- Constants -*/
pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub(crate) const TILESIZE: f32 = 0.1;
pub(crate) const GRAVITY : f32 = 0.02;

/*- Imports -*/
use bevy::{prelude::*, render::{texture::ImageSettings ,camera::ScalingMode}};
use player::PlayerPlugin;
use debug::DebugPlugin;
use ascii::AsciiPlugin;
use tilemap::TileMapPlugin;
use bevy_rapier2d::prelude::*;

/*- Module imports -*/
mod player;
mod debug;
mod ascii;
mod tilemap;

/*- Initialize -*/
fn main() {

    /*- Initialize app -*/
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: 1600.0,
            height: 900.0,
            title: "Game".to_string(),
            resizable: false,
            present_mode: bevy::window::PresentMode::Fifo,
            decorations: true,
            ..Default::default()
        })
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_plugin(PlayerPlugin)
        .add_plugin(AsciiPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(TileMapPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .run();
}

/*- Initialize camera -*/
fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            top: 1.0,
            bottom: -1.0,
            right: 1.0 * RESOLUTION,
            left: -1.0 * RESOLUTION,
            scaling_mode: ScalingMode::None,
            ..default()
        },
        ..default()
    });
}

