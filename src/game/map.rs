use crate::game::{FruitTilesTextures, GameStates};
use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use iyes_loopless::prelude::AppLooplessStateExt;
use rand::Rng;

use super::camera::MainCamera;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameStates::Game, spawn_tiles)
            .register_inspectable::<LightSource>()
            .add_system(update_lighting)
            .add_system(move_tiles_outside_view);
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

    // The vertical loop
    for y in -TILE_LIMIT..=TILE_LIMIT {
        // The horizontal loop
        for x in -TILE_LIMIT..=TILE_LIMIT {
            // Spawn tile

            // Choose random tile
            let texture =
                tile_textures_vec[rand::thread_rng().gen_range(0..tile_textures_vec_len)].clone();

            commands
                .spawn_bundle(SpriteBundle {
                    transform: Transform {
                        translation: Vec3 {
                            x: TILE_SIZE.x * TILE_SCALE * x as f32,
                            y: TILE_SIZE.y * TILE_SCALE * y as f32,
                            z: -1.,
                        },
                        scale: Vec3::splat(TILE_SCALE),
                        ..Default::default()
                    },
                    texture,
                    ..Default::default()
                })
                .insert(TileCover);
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
    mut tiles_query: Query<&mut Transform, (Or<(With<TileCover>, With<Tile>)>, Without<MainCamera>)>,
    //mut commands: Commands,
    window: Res<Windows>,
) {
    let window = window.get_primary().unwrap();
    
    let window_right = window.width() / 2.;
    let window_up = window.height() / 2.;

    for camera_tf in player_query.iter() {
        let camera_tl = camera_tf.translation;

        for mut tile_tf in tiles_query.iter_mut() {
            let tile_tl = tile_tf.translation;

            let is_not_viewable =
            tile_tl.x >= camera_tl.x + window_right + TILE_SIZE.x || // Right
            tile_tl.x <= camera_tl.x - window_right - TILE_SIZE.x || // Left
            tile_tl.y >= camera_tl.y + window_up + TILE_SIZE.y || // Up
            tile_tl.y <= camera_tl.y - window_up - TILE_SIZE.y; // Down

            if is_not_viewable {
                // Move the tile to the
                // Opposite position in
                // Relation to the player
                
                //                  New Tile Position
                //                     |                    
                //Tile outside view   \/
                //   |               |_|
                //  \/     Player     |
                // |_| ---------------
                //
                
                tile_tf.translation.x += (camera_tl.x - tile_tl.x) * 2.;
                tile_tf.translation.y += (camera_tl.y - tile_tl.y) * 2.;
            }
        }
    }
}