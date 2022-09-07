use std::ops::Add;

use crate::game::{FruitTilesTextures, GameStates};
use bevy::{prelude::*, utils::HashMap};
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet};
use rand::Rng;

use super::{camera::MainCamera};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameStates::Game, spawn_tiles)
            .register_inspectable::<LightSource>()
            .add_system_set(
                ConditionSet::new()
                  .run_in_state(GameStates::Game)
                  .with_system(remove_tiles_outside_view)
                  .with_system(spawn_tiles_inside_view).into()
              )
            .add_system(update_lighting);
    }
}

// The square root of the number of tiles the map has (i.e. the map area)
pub const TILE_LIMIT: i32 = 30;
// The size of a tile in pixels
pub const TILE_SIZE: Vec2 = Vec2::splat(32.);
pub const TILE_SCALE: f32 = 1.;
// How far a tile can be from the screen without being despawned
const OUT_OF_BORDERS_TILE_TOLERANCE: f32 = 100.;

#[derive(Component, Inspectable)]
pub struct LightSource {
    pub max_range: f32,
    pub inner_range: f32,
    pub color: Color,
}

#[derive(Component)]
struct Tile;

#[derive(Component)]
struct TileCover;

fn spawn_tile (
    commands: &mut Commands,
    tile_textures: &Res<FruitTilesTextures>,
    mut translation: Vec3,
) {

    translation.z = -1.;

    let tile_textures_vec = [
        &tile_textures.grass1,
        &tile_textures.grass2,
        // Repeat 3 for better odds of being the chosen one
        // Or else there wil be too many flowers
        &tile_textures.grass3,
        &tile_textures.grass3,
        &tile_textures.grass3,
        // Same with 4
        &tile_textures.grass4,
        &tile_textures.grass4,
    ];
    let tile_textures_vec_len = tile_textures_vec.len();

    // Choose random tile
    let texture =
    tile_textures_vec[rand::thread_rng().gen_range(0..tile_textures_vec_len)].clone();


    commands
                .spawn_bundle(SpriteBundle {
                    transform: Transform {
                        translation,
                        scale: Vec3::splat(TILE_SCALE),
                        ..Default::default()
                    },
                    texture,
                    ..Default::default()
                })
                .insert(TileCover);
}

fn spawn_tiles(mut commands: Commands, tile_textures: Res<FruitTilesTextures>) {
    commands
        .spawn_bundle(TransformBundle::from_transform(Transform::from_xyz(
            400., 400., 0.,
        )))
        .insert(LightSource {
            max_range: 250.,
            inner_range: 200.,
            color: Color::rgb(0.80, 0.70, 0.60) * 0.7,
        });

    // The vertical loop
    for y in -TILE_LIMIT..=TILE_LIMIT {
        // The horizontal loop
        for x in -TILE_LIMIT..=TILE_LIMIT {
            // Spawn tile

            spawn_tile(&mut commands, &tile_textures,  Vec3 {
                x: TILE_SIZE.x * TILE_SCALE * x as f32,
                y: TILE_SIZE.y * TILE_SCALE * y as f32,
                z: 0., // The z is overwritten
            })
        }
    }
}

fn update_lighting(
    mut query: Query<(&mut Sprite, &Transform), With<TileCover>>,
    light_source_query: Query<(&LightSource, &Transform)>,
) {
    for (mut sprite, tf) in query.iter_mut() {
        let mut color = Color::rgb(0.15, 0.15, 0.3);

        for (light_source, light_tf) in light_source_query.iter() {
            color += light_source.color
                * (1.
                    - (light_tf.translation.distance(tf.translation) - light_source.inner_range)
                        / (light_source.max_range - light_source.inner_range))
                    .clamp(0., 1.);
        }

        // Convert color into Vec4 for easier modification
        let color: Vec4 = color.into();
        sprite.color = color.min(Vec4::ONE).into();
    }
}

fn remove_tiles_outside_view(
    camera_query: Query<&Transform, With<MainCamera>>,
    tiles_query: Query<(Entity, &Transform), (Or<(With<TileCover>, With<Tile>)>, Without<MainCamera>)>,
    window: Res<Windows>,
    mut commands: Commands,
) {
    let window = window.get_primary().unwrap();
    
    let window_right = window.width() / 2. + 50.; //+ TILE_SIZE.x / 2.;
    let window_up = window.height() / 2. + 50.;//+ TILE_SIZE.y / 2.;

    for camera_tf in camera_query.iter() {
        let camera_tl = camera_tf.translation;

        // The camera corners are to be used as a reference.
        let camera_corners = [
        Vec2 {x: camera_tl.x + window_right, y: camera_tl.y + window_up}, // Top-Right
        Vec2 {x: camera_tl.x - window_right, y: camera_tl.y - window_up}, // Bottom-Left
        ];

        for (tile_entity, tile_tf) in tiles_query.iter() {
            let tile_tl = tile_tf.translation;

            let is_not_viewable =
            tile_tl.x >= camera_corners[0].x   + OUT_OF_BORDERS_TILE_TOLERANCE|| // Right
            tile_tl.x <= camera_corners[1].x   - OUT_OF_BORDERS_TILE_TOLERANCE|| // Left
            tile_tl.y >= camera_corners[0].y   + OUT_OF_BORDERS_TILE_TOLERANCE|| // Up
            tile_tl.y <= camera_corners[1].y   - OUT_OF_BORDERS_TILE_TOLERANCE; // Down

            if is_not_viewable {
                commands.entity(tile_entity).despawn();
                continue;
            }
            }
        }

    
}


fn spawn_tiles_inside_view(
    camera_query: Query<&Transform, With<MainCamera>>,
    tiles_query: Query<(Entity, &Transform), (Or<(With<TileCover>, With<Tile>)>, Without<MainCamera>)>,
    window: Res<Windows>,
    mut commands: Commands,
    tile_textures: Res<FruitTilesTextures>,

    // This is used to store the camera's position when tiles were last spawned in each direction
    // (It is an array because I wasn't able to get an hashmap working ↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓)
    // http://static1.squarespace.com/static/54ad91eae4b04d2abc8d6247/t/568b4fe4c21b86066ae97093/1451970532940/?format=1500w
    mut last_spawned: Local<[f32; 2]>, // This is the order: [Vertical, Horizontal]
) {
    
    if last_spawned.is_empty() {
        for index in 0..2 {
            last_spawned[index] = 0.;
        }
    }


    // Window boilerplate 
    let window = window.get_primary().unwrap();
    let window_w = window.width();
    let window_h = window.height();

    for camera_tf in camera_query.iter() {
        let camera_tl = camera_tf.translation;

        // You can change this to 0 and see what happens when you walk diagonally
        // It's defined here so it can be used in all the following if blocks
        let extra_tiles = 6; // 6 to be safe

        if camera_tl.y >= last_spawned[0] + TILE_SIZE.y { // Up

            // Fill the whole vertical part
            //
            // -----------------------
            // ######################|
            // ######################|
            // --------------        |
            //    TILES     |        |
            //              |        |
            let number_of_horizontal_tiles = (window.width() / TILE_SIZE.x).ceil() as i32 + extra_tiles * 2;
                
            // The horizontal loop
            for horizontal_tile_index in 0..=number_of_horizontal_tiles {
                
                let mut x = camera_tl.x - window_w / 2. + horizontal_tile_index as f32 * TILE_SIZE.x -
                                extra_tiles as f32 * TILE_SIZE.y / 2.;
                let mut y = camera_tl.y + window_h / 2. + OUT_OF_BORDERS_TILE_TOLERANCE;

                // Snap to grid
                y -= y % TILE_SIZE.y;
                x -= x % TILE_SIZE.x;

                spawn_tile(&mut commands, &tile_textures, Vec3 { 
                    x,
                    y, 
                    z: 0. 
                });

                // Spawn another one below for safety
                spawn_tile(&mut commands, &tile_textures, Vec3 { 
                    x,
                    y: y - TILE_SIZE.y, 
                    z: 0. 
                });



            }

            
            last_spawned[0] = camera_tl.y;
        }
        if camera_tl.y <= last_spawned[0] - TILE_SIZE.y { // Down

            let number_of_horizontal_tiles = (window.width() / TILE_SIZE.x).ceil() as i32 + extra_tiles * 2;
                
            // The horizontal loop
            for horizontal_tile_index in 0..=number_of_horizontal_tiles {
                
                let mut x = camera_tl.x - window_w / 2. + horizontal_tile_index as f32 * TILE_SIZE.x -
                                extra_tiles as f32 * TILE_SIZE.y / 2.;
                let mut y = camera_tl.y - window_h / 2. - OUT_OF_BORDERS_TILE_TOLERANCE;

                // Snap to grid
                y -= y % TILE_SIZE.y;
                x -= x % TILE_SIZE.x;

                spawn_tile(&mut commands, &tile_textures, Vec3 { 
                    x,
                    y, 
                    z: 0. 
                });

                // Spawn another one below for safety
                spawn_tile(&mut commands, &tile_textures, Vec3 { 
                    x,
                    y: y + TILE_SIZE.y, 
                    z: 0. 
                });

            }

            

            last_spawned[0] = camera_tl.y;
        }
        if camera_tl.x >= last_spawned[1] + TILE_SIZE.x { // Right
                
            let number_of_vertical_tiles = (window.height() / TILE_SIZE.x).ceil() as i32 + extra_tiles * 2;
            
            // The vertical loop
            for vertical_tile_index in 0..=number_of_vertical_tiles {

                let mut x = camera_tl.x + window_w / 2. + OUT_OF_BORDERS_TILE_TOLERANCE;
                let mut y = camera_tl.y - window_h / 2. + vertical_tile_index as f32 * TILE_SIZE.y -
                                extra_tiles as f32 * TILE_SIZE.y / 2.;

                // Snap to grid
                y -= y % TILE_SIZE.y;
                x -= x % TILE_SIZE.x;

                spawn_tile(&mut commands, &tile_textures, Vec3 { 
                    x,
                    y, 
                    z: 0. 
                });

                // Spawn another one below for safety
                spawn_tile(&mut commands, &tile_textures, Vec3 { 
                    x: x - TILE_SIZE.x,
                    y, 
                    z: 0. 
                });

            }

            

            last_spawned[1] = camera_tl.x;
        }
        if camera_tl.x <= last_spawned[1] - TILE_SIZE.x { // Left

            let number_of_vertical_tiles = (window.height() / TILE_SIZE.x).ceil() as i32 + extra_tiles * 2;
            
            // The vertical loop
            for vertical_tile_index in 0..=number_of_vertical_tiles {

                let mut x = camera_tl.x - window_w / 2. - OUT_OF_BORDERS_TILE_TOLERANCE;
                let mut y = camera_tl.y - window_h / 2. + vertical_tile_index as f32 * TILE_SIZE.y -
                                extra_tiles as f32 * TILE_SIZE.y / 2.;

                // Snap to grid
                y -= y % TILE_SIZE.y;
                x -= x % TILE_SIZE.x;

                spawn_tile(&mut commands, &tile_textures, Vec3 { 
                    x,
                    y, 
                    z: 0. 
                });

                // Spawn another one below for safety
                spawn_tile(&mut commands, &tile_textures, Vec3 { 
                    x: x + TILE_SIZE.x,
                    y, 
                    z: 0. 
                });

            }

            last_spawned[1] = camera_tl.x;
        }
    }
}