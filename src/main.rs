#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use audio::{AudioAssets, MusicChannel, SoundChannel};
use bevy::prelude::*;
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use bevy_kira_audio::{AudioApp, AudioPlugin};
use debug::DebugPlugin;
use state::{game::GamePlugin, loading::GameAssets, main_menu::MainMenuPlugin};

mod audio;
mod debug;
mod enemies;
mod grid;
mod input;
mod inventory;
mod state;
mod tower;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(AudioPlugin)
        .add_state::<state::State>()
        .add_loading_state(
            LoadingState::new(state::State::Loading).continue_to_state(state::State::MainMenu),
        )
        .add_collection_to_loading_state::<_, GameAssets>(state::State::Loading)
        .add_collection_to_loading_state::<_, AudioAssets>(state::State::Loading)
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(DebugPlugin)
        .add_system(state::loading::setup.in_schedule(OnEnter(state::State::Loading)))
        .add_system(state::loading::cleanup.in_schedule(OnExit(state::State::Loading)))
        .add_audio_channel::<MusicChannel>()
        .add_audio_channel::<SoundChannel>()
        .insert_resource(audio::VolumeSettings::default())
        .run();
}
