use crate::game::{FruitTilesTextures, GameStates};
use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet};
use rand::Rng;

use super::{camera::MainCamera, vec3_to_vec2};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameStates::Game, spawn_tiles)
            .register_inspectable::<LightSource>()
            .add_system_set(
                ConditionSet::new()
                  .run_in_state(GameStates::Game)
                  .with_system(move_tiles_outside_view).into()
              )
            .add_system(update_lighting);
    }
}

// The square root of the number of tiles the map has (i.e. the map area)
pub const TILE_LIMIT: i32 = 30;
// The size of a tile in pixels
pub const TILE_SIZE: Vec2 = Vec2::splat(32.);
pub const TILE_SCALE: f32 = 1.;

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

fn move_tiles_outside_view(
    player_query: Query<&Transform, With<MainCamera>>,
    tiles_query: Query<(Entity, &Transform), (Or<(With<TileCover>, With<Tile>)>, Without<MainCamera>)>,
    window: Res<Windows>,
    mut commands: Commands,
    tile_textures: Res<FruitTilesTextures>,
) {
    let window = window.get_primary().unwrap();
    
    let window_right = window.width() / 2.;
    let window_up = window.height() / 2.;

    for camera_tf in player_query.iter() {
        let camera_tl = camera_tf.translation;

    
        // The camera corners are to be used as a reference.
        let camera_corners = [
        Vec2 {x: camera_tl.x + window_right, y: camera_tl.y + window_up}, // Top-Right
        Vec2 {x: camera_tl.x - window_right, y: camera_tl.y - window_up}, // Bottom-Left
        ];

        // This variable is used in order to see where the top-rightmost and the
        // Bottom-leftmost points of the tiles are at present, 
        // And if they are inside the camera's field of view
        // i.e. the tiles don't cover the whole window,
        // They will be spawned accordingly.
        let mut tile_corners =[
            vec3_to_vec2(camera_tl), // Top-Right
            vec3_to_vec2(camera_tl), // Bottom-Left
        ];
        for (tile_entity, tile_tf) in tiles_query.iter() {
            let tile_tl = tile_tf.translation;

            let is_not_viewable =
            tile_tl.x >= camera_corners[0].x   || // Right
            tile_tl.x <= camera_corners[1].x   || // Left
            tile_tl.y >= camera_corners[0].y  || // Up
            tile_tl.y <= camera_corners[1].y ; // Down

            if is_not_viewable {
                commands.entity(tile_entity).despawn();
                continue;
            }

            if tile_tl.x > tile_corners[0].x && tile_tl.y > tile_corners[0].y {
                tile_corners[0] = vec3_to_vec2(tile_tl);
            }
            if tile_tl.x < tile_corners[1].x && tile_tl.y < tile_corners[1].y {
                tile_corners[1] = vec3_to_vec2(tile_tl);
            }
        }



        // If the player hasn't moved
        if camera_corners[0].x <= tile_corners[0].x && camera_corners[0].y <= tile_corners[0].y && 
        camera_corners[1].x <= tile_corners[1].x && camera_corners[1].y <= tile_corners[1].y{
            continue; // Since there is only one camera, this is equal to return;
            // I use continue because I can, not because I need to.
            //...
            // HMMMMMM I'mma "𝘤𝘰𝘯𝘵𝘪𝘯𝘶𝘦;" my e̶x̶t̶r̶e̶m̶e̶l̶y̶ d̶e̶t̶r̶i̶m̶e̶n̶t̶a̶l̶ a̶n̶d̶ u̶n̶d̶e̶n̶i̶a̶b̶l̶y̶ t̶e̶r̶r̶i̶b̶l̶e̶  𝓽𝓮𝓻𝓻𝓲𝓯𝓲𝓬 𝔀𝓸𝓻𝓴
        }

        // If the player has moved in the top/right direction
        if camera_corners[0].x > tile_corners[0].x || camera_corners[0].y > tile_corners[0].y {

            dbg!(camera_corners, tile_corners);

            let number_of_tiles_to_spawn = ((camera_corners[0] - tile_corners[0]) / TILE_SIZE).ceil() + TILE_SIZE;

            // Fill the whole vertical part
            //
            // -----------------------
            // ######################|
            // ######################|
            // --------------        |
            //    TILES     |        |
            //              |        |
            let number_of_horizontal_tiles = (window.width() / TILE_SIZE.x).ceil() as i32;
            // The vertical loop
            // println!("{}, {}", number_of_tiles_to_spawn.y, number_of_horizontal_tiles);
            for vertical_tile_index in 0..=(number_of_tiles_to_spawn.y as i32) {
                // The horizontal loop
                for horizontal_tile_index in 0..=number_of_horizontal_tiles {
                    spawn_tile(&mut commands, &tile_textures, Vec3 { 
                        // Start point       +           index * size
                        x: tile_corners[1].x + horizontal_tile_index as f32 * TILE_SIZE.x , 
                        y: tile_corners[0].y + vertical_tile_index as f32 * TILE_SIZE.y, 
                        z: 0. 
                    });
                    //println!("Spawn: {}, {}", horizontal_spawn, vertical_spawn);
                }
            }



        }

    }

    
}