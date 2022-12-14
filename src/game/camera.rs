use bevy::prelude::*;
use crate::game::player::Player;
use crate::game::map::{TILE_LIMIT, TILE_SCALE, TILE_SIZE};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_camera_system)
            .add_system(move_camera_system);
    }
}

#[derive(Component)]
struct MainCamera;

fn setup_camera_system(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        transform: Transform::from_xyz(0.,0.,0.),
        ..Default::default()
    })
        .insert(MainCamera);
}

fn move_camera_system(
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    window: Res<Windows>
) {
    for player_tf in player_query.iter() {
        for mut camera_tf in camera_query.iter_mut() {
            let window = window.get_primary().unwrap();
            let player_trans = player_tf.translation;

            let outside_x_bounds = player_trans.x.abs() + window.width()/2. >= (TILE_LIMIT as f32) * TILE_SIZE.x * TILE_SCALE;
            let outside_y_bounds = player_trans.y.abs() + window.height()/2. >= (TILE_LIMIT as f32) * TILE_SIZE.x * TILE_SCALE;
            // Don't let the camera leave the tiled map
            if outside_x_bounds
                && outside_y_bounds {
                return;
            }
            if outside_x_bounds {
                camera_tf.translation.y = player_trans.y;
                return;
            }
            if outside_y_bounds {
                camera_tf.translation.x = player_trans.x;
                return;
            }
            camera_tf.translation = player_trans;
        }
    }
}