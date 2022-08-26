use bevy::prelude::*;
use bevy_rapier2d::prelude::{RigidBody, Collider, GravityScale, LockedAxes};

use crate::{ TILESIZE, GRAVITY };

pub struct AsciiPlugin;
pub struct AsciiSheet(Handle<TextureAtlas>);

impl Plugin for AsciiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_ascii);
    }
}


#[derive(Bundle)]
struct SpritePhysicsBundle {
    rb: RigidBody,
    cl: Collider,
}

pub fn spawn_ascii_sprite(
    commands: &mut Commands,
    ascii: &AsciiSheet,
    index: usize,
    color: Color,
    translation: Vec3,
    physics:Option<(RigidBody, Collider)>
) -> Entity {
    assert!(index < 256, "Index out of Ascii Range");

    let mut sprite = TextureAtlasSprite::new(index);
    sprite.color = color;
    sprite.custom_size = Some(Vec2::splat(TILESIZE));

    /*- Initialize sprite -*/
    let mut commands = commands
        .spawn_bundle(SpriteSheetBundle {
            sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform { translation, ..Default::default() },
            ..Default::default()
        });

    /*- Check if physics is enabled -*/
    if let Some(physics) = physics {
        commands
            .insert_bundle(SpritePhysicsBundle {
                rb: physics.0,
                cl: physics.1
            });
    };

    /*- Insert gravity -*/
    commands
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(GravityScale(GRAVITY))
        .id()
}

fn load_ascii(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    let image = assets.load("Ascii.png");
    let atlas = TextureAtlas::from_grid_with_padding(
        image,
        Vec2::splat(9.0),
        16,
        16,
        Vec2::new(2.0, 2.0),
        Vec2::splat(0.0),
    );

    let atlas_handle = texture_atlases.add(atlas);

    commands.insert_resource(AsciiSheet(atlas_handle));
}