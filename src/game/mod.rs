use bevy::prelude::*;

use bevy::prelude::{App, Handle, Image};
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::Actionlike;

mod player;
mod map;
mod camera;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum GameStates {
    AssetLoading,
    Game,
}

#[derive(AssetCollection)]
struct FruitTilesTextures {
    # [asset(path = "textures/ground_tiles/1.png")]
    pub grass1: Handle < Image >,
    # [asset(path = "textures/ground_tiles/2.png")]
    pub grass2: Handle < Image >,
    # [asset(path = "textures/ground_tiles/3.png")]
    pub grass3: Handle < Image >,
    # [asset(path = "textures/ground_tiles/4.png")]
    pub grass4: Handle < Image >,
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

//region MainPlugin
pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(GameStates::AssetLoading)
            .add_loading_state(
                LoadingState::new(GameStates::AssetLoading)
                    .continue_to_state(GameStates::Game)
                    .with_collection::<TextureAssets>()
                    .with_collection::<FruitTilesTextures>(),
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
            .add_plugin(map::MapPlugin)
            .add_plugin(camera::CameraPlugin);
    }
}
//endregion
