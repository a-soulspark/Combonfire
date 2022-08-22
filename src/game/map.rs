use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;
use rand::Rng;
use crate::game::{FruitTilesTextures, GameStates};
use bevy_prototype_lyon::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(ShapePlugin)
            .add_enter_system(GameStates::Game, spawn_tiles);
    }
}

// The square root of the number of tiles the map has (i.e. the map area)
pub const TILE_LIMIT: usize = 30;
// The size of a tile in pixels
pub const TILE_SIZE: Vec2 = Vec2::splat(32.);
pub const TILE_SCALE: f32 = 1.;
const TILES_Z: f32 = -1.;

#[derive(Component)]
struct Tile;

#[derive(Component)]
struct TileCover;

fn spawn_tiles(
    mut commands: Commands,
    tile_textures: Res<FruitTilesTextures>,
) {
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
    for _ in 0..TILE_LIMIT*2 {
        // The horizontal loop
        for _ in 0..TILE_LIMIT*2 {


            // Spawn tile

            // Choose random tile
            let texture = tile_textures_vec[rand::thread_rng().gen_range(0..tile_textures_vec_len)].clone();
            commands
                .spawn_bundle(
                    GeometryBuilder::build_as(
                        &shapes::RegularPolygon {
                            sides: 4,
                            feature: shapes::RegularPolygonFeature::SideLength(TILE_SIZE.x),
                            ..shapes::RegularPolygon::default()
                        },
                        DrawMode::Outlined {
                            fill_mode: FillMode::color(Color::Rgba {
                                red: 0.0,
                                green: 0.0,
                                blue: 0.0,
                                alpha: 0.8
                            }),
                            outline_mode: StrokeMode::new(Color::BLACK, 0.),
                        },
                        Transform {
                            translation: Vec3{
                                x: spawn.x,
                                y: spawn.y,
                                z: TILES_Z - 1.,
                            },
                            ..Default::default()
                        }
                    )
                )
                .insert(TileCover);

            commands
                .spawn_bundle(
                SpriteBundle {
                    transform: Transform {
                        translation: Vec3{
                            x: spawn.x,
                            y: spawn.y,
                            z: -20.,
                        },
                        scale: Vec3::splat(TILE_SCALE),
                        ..Default::default()
                    },
                    texture,
                    ..Default::default()
                }
            );

            // Change spawn
            spawn.x -= TILE_SIZE.x * TILE_SCALE;
        }

        // Adjust spawn
        spawn.x = (TILE_LIMIT as f32) * TILE_SIZE.x * TILE_SCALE;
        spawn.y -= TILE_SIZE.y * TILE_SCALE;
    }
}