use bevy::prelude::App;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;
use leafwing_input_manager::{
    prelude::{ActionState, InputMap, VirtualDPad},
    InputManagerBundle,
};

const PLAYER_SPEED: f32 = 400.;
/// Controls how tight the player controls feel; Higher the value, the more responsive it feels
const PLAYER_TIGHTNESS: f32 = 5.;

use super::{GameStates, InputAction, TextureAssets};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_move_system)
            .add_enter_system(GameStates::Game, spawn_player);
    }
}

fn spawn_player(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    /* Create the ground. */
    commands
        .spawn_bundle(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)))
        .insert(Collider::cuboid(500.0, 50.0));

    commands
        .spawn_bundle(SpriteBundle {
            texture: texture_assets.player.clone(),
            transform: Transform {
                scale: Vec3::splat(0.1),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle((
            ExternalForce::default(),
            RigidBody::Dynamic,
            Friction {
                coefficient: 0.,
                combine_rule: CoefficientCombineRule::Multiply,
            },
            Velocity::zero(),
            Collider::cuboid(250., 250.),
            LockedAxes::ROTATION_LOCKED,
        ))
        .insert_bundle(InputManagerBundle::<InputAction> {
            input_map: InputMap::default()
                .insert(VirtualDPad::arrow_keys(), InputAction::Move)
                .insert(VirtualDPad::wasd(), InputAction::Move)
                .build(),
            ..Default::default()
        });
}

fn player_move_system(
    mut query: Query<(&mut ExternalForce, &Velocity, &ActionState<InputAction>)>,
) {
    for (mut force, velocity, action_state) in query.iter_mut() {
        let mut target_velocity = Vec2::ZERO;

        if action_state.pressed(InputAction::Move) {
            let dual_axis_move = action_state.axis_pair(InputAction::Move).unwrap();

            // Returns None if the axis is neutral, aka (x,y) = (0,0)
            if let Some(direction) = dual_axis_move.direction() {
                target_velocity = direction.unit_vector() * PLAYER_SPEED;
            }
        }

        force.force = (target_velocity - velocity.linvel) * PLAYER_TIGHTNESS;
    }
}
