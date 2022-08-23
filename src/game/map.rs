use crate::game::{FruitTilesTextures, GameStates};
use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use iyes_loopless::prelude::AppLooplessStateExt;
use rand::Rng;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameStates::Game, spawn_tiles)
            .register_inspectable::<LightSource>()
            .add_system(update_lighting);
    }
}

// The square root of the number of tiles the map has (i.e. the map area)
pub const TILE_LIMIT: usize = 30;
// The size of a tile in pixels
pub const TILE_SIZE: Vec2 = Vec2::splat(32.);
pub const TILE_SCALE: f32 = 1.;
const TILES_Z: f32 = -1.;

#[derive(Component, Inspectable)]
pub struct LightSource {
    pub range: f32,
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
        .insert(LightSource { range: 250. });

    // Where the tile will spawn
    // the first tile spawns in the top right corner
    let mut spawn: Vec2 = (TILE_LIMIT as f32) * TILE_SIZE * TILE_SCALE;

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
    for _ in 0..TILE_LIMIT * 2 {
        // The horizontal loop
        for _ in 0..TILE_LIMIT * 2 {
            // Spawn tile

            // Choose random tile
            let texture =
                tile_textures_vec[rand::thread_rng().gen_range(0..tile_textures_vec_len)].clone();
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::Rgba {
                            red: 0.0,
                            green: 0.0,
                            blue: 0.0,
                            alpha: 1.,
                        },
                        custom_size: Option::from(TILE_SIZE * TILE_SCALE),
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3 {
                            x: spawn.x,
                            y: spawn.y,
                            z: TILES_Z,
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(TileCover);

            commands.spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3 {
                        x: spawn.x,
                        y: spawn.y,
                        z: -20.,
                    },
                    scale: Vec3::splat(TILE_SCALE),
                    ..Default::default()
                },
                texture,
                ..Default::default()
            });

            // Change spawn
            spawn.x -= TILE_SIZE.x * TILE_SCALE;
        }

        // Adjust spawn
        spawn.x = (TILE_LIMIT as f32) * TILE_SIZE.x * TILE_SCALE;
        spawn.y -= TILE_SIZE.y * TILE_SCALE;
    }
}

fn update_lighting(
    mut query: Query<(&mut Sprite, &Transform), With<TileCover>>,
    light_source_query: Query<(&LightSource, &Transform)>,
) {
    for (mut sprite, tf) in query.iter_mut() {
        let mut top_brightness: f32 = 1.;
        for (light_source, light_tf) in light_source_query.iter() {
            top_brightness = top_brightness
                .min(light_tf.translation.distance(tf.translation) / light_source.range);
        }

        sprite.color = *Color::BLACK.clone().set_a(top_brightness);
    }
}
