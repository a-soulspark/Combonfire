use bevy::prelude::*;

use bevy::prelude::{App, Handle, Image};
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::Actionlike;

mod player;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum GameStates {
    AssetLoading,
    Game,
}

#[derive(AssetCollection)]
struct TextureAssets {
    #[asset(path = "textures/player.png")]
    player: Handle<Image>,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum InputAction {
    Move,
    MoveY,
}

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(GameStates::AssetLoading)
            .add_loading_state(
                LoadingState::new(GameStates::AssetLoading)
                    .continue_to_state(GameStates::Game)
                    .with_collection::<TextureAssets>(),
            )
            .add_plugin(WorldInspectorPlugin::new())
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(InputManagerPlugin::<InputAction>::default())
            .insert_resource(RapierConfiguration {
                gravity: Vec2::ZERO,
                ..Default::default()
            })
            .add_plugin(player::PlayerPlugin)
            .add_startup_system(setup_camera_system);
    }
}
//endregion

fn setup_camera_system(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
