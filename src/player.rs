/*- Global allowings -*/
#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_mut,
    unused_assignments
)]

/*- Imports -*/
use bevy_rapier2d::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy::{
    prelude::*,
    sprite::collide_aabb::collide
};
use crate::{
    TILESIZE,
    ascii::{
        spawn_ascii_sprite,
        AsciiSheet
    },
    tilemap::TileCollider
};

/*- Constants -*/
const UP:[KeyCode; 2] = [KeyCode::W, KeyCode::Up];
const DOWN:[KeyCode; 2] = [KeyCode::S, KeyCode::Down];
const LEFT:[KeyCode; 2] = [KeyCode::A, KeyCode::Left];
const RIGHT:[KeyCode; 2] = [KeyCode::D, KeyCode::Right];

const MOVEMENT_DIVIDER:f32 = 1000.0f32;

/*- Enums & structs -*/
pub struct PlayerPlugin;

/*- Main player component -*/
#[derive(Component, Inspectable, Default)]
pub struct Player {
    speed: f32,
    // direction: Direction,
}

/*- Player direction -*/
#[derive ( Inspectable )]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/*- Bevy requires all player asset to implement default -*/
impl Default for Direction {
    fn default() -> Self {
        Direction::Right
    }
}

/*- Add systems for player -*/
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(camera_follow.after("movement"))
            .add_system(player_movement.label("movement"));
            // .add_system(animate_sprite);
    }
}

/*- Make camera follow player on update -*/
fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>) {
        let player_transform = player_query.single();
        let mut camera_transform = camera_query.single_mut();
        
        camera_transform.translation.x = player_transform.translation.x;
        camera_transform.translation.y = player_transform.translation.y;
    }


// fn animate_sprite(
//     time: Res<Time>,
//     texture_atlases: Res<Assets<TextureAtlas>>,
//     mut query: Query<(
//         &mut Animation,
//         &mut TextureAtlasSprite,
//         &Handle<TextureAtlas>,
//     )>,
// ) {
//     for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
//         timer.tick(time.delta());
//         if timer.just_finished() {

//             let texture_atlas = match texture_atlases.get(texture_atlas_handle) {
//                 Some(texture_atlas) => texture_atlas,
//                 None => continue
//             };
//             sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
//         }
//     }
// }

fn player_movement(
    mut player_query: Query<(&Player, &mut Transform)>,
    mut ext_forces: Query<&mut ExternalForce>,
    mut ext_impulses: Query<&mut ExternalImpulse>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    /*- Get player and its tranform as mut because we want to cahnge it -*/
    let (player, mut transform) = player_query.single_mut();

    /*- Directions -*/
    let mut y_delta = 0.0;
    let mut x_delta = 0.0;
    
    /*- Movement difference -*/
    let diff:f32 = 1.;
 
    /*- Handle movement -*/
    // if keyboard.any_pressed(DOWN)  { y_delta -= diff; }
    if keyboard.any_pressed(LEFT)  { x_delta -= diff; }
    if keyboard.any_pressed(RIGHT) { x_delta += diff; }
    // if keyboard.any_pressed(UP)    {
    //     for mut ext_force in ext_forces.iter_mut() {
    //         ext_force.force = Vec2::new(0.0, 200000.0);
    //         ext_force.torque = 2.4;
    //     }
    //     for mut ext_impulse in ext_impulses.iter_mut() {
    //         ext_impulse.impulse = Vec2::new(100.0, 200.0);
    //         ext_impulse.torque_impulse = 0.4;
    //     }
    // }

    /*- Check if player is moving somewhere -*/
    if x_delta != 0.0 || y_delta != 0.0 {
        // let magnitude = (x_delta * x_delta + y_delta * y_delta).sqrt();
        let mut x = 0.0;
        let mut y = 0.0;

        /*- Total end player-speed -*/
        let y_def = y_delta / MOVEMENT_DIVIDER * player.speed * TILESIZE * 10.;
        let x_def = x_delta / MOVEMENT_DIVIDER * player.speed * TILESIZE * 10.;

        /*- Check for weird negative delta -*/
        if x_delta < 0.0 { x = -x_def.abs(); }
        else { x = x_def; };
        
        if y_delta < 0.0 { y = -y_def.abs(); }
        else { y = y_def; };

        /*- End x position -*/
        let target_x = transform.translation + Vec3::new(x, 0.0, 0.0);
        if wall_collision_check(target_x, &wall_query) { transform.translation = target_x; }
        
        /*- End y position -*/
        let target_y = transform.translation + Vec3::new(0.0, y, 0.0);
        if wall_collision_check(target_y, &wall_query) { transform.translation = target_y; }
    }

}

fn wall_collision_check(
    target_player_pos: Vec3,
    wall_query: &Query<&Transform, (With<TileCollider>, Without<Player>)>,
) -> bool {
    for wall_transform in wall_query.iter() {
        let collision = collide(
            target_player_pos,
            Vec2::splat(TILESIZE * 0.9),
            wall_transform.translation,
            Vec2::splat(TILESIZE),
        );
        if collision.is_some() {
            return false;
        }
    }
    true
}

/*- Spawn player with commands & config -*/
fn spawn_player(
    mut commands: Commands,
    ascii: Res<AsciiSheet>,
) {

    /*- Initialize physics -*/
    let rigid_body = RigidBody::Dynamic;
    let collider = Collider::cuboid(TILESIZE / 2.0, TILESIZE / 2.0);

    /*- Spawn -*/
    let player = spawn_ascii_sprite(
        &mut commands, 
        &ascii, 
        0, /* index */
        Color::hex("2596be").unwrap(), 
        Vec3::new(2.0*TILESIZE, -2.0*TILESIZE, 900.),
        Some(rigid_body),
        Some(collider)
    );

    /*- Insert player -*/
    commands
        .entity(player)
        .insert(Name::new("Player"))
        .insert(Player { speed: 5.0 });
}