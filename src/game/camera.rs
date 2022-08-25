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
pub struct MainCamera;

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
) {
    for player_tf in player_query.iter() {
        for mut camera_tf in camera_query.iter_mut() {
            let player_trans = player_tf.translation;

            camera_tf.translation = player_trans;
        }
    }
}