#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::prelude::*;
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use bevy_kira_audio::{AudioApp, AudioPlugin};

use audio::{AudioAssets, DrumsChannel, MusicChannel, SoundChannel};
use state::{
    game::GamePlugin, loading::GameAssets, main_menu::MainMenuPlugin, results::ResultsPlugin,
};
use ui::constants::BACKGROUND_COLOR;

mod audio;
mod enemies;
mod gameplay;
mod grid;
mod input;
mod state;
mod tower;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(AudioPlugin)
        .add_state::<state::State>()
        .add_loading_state(
            LoadingState::new(state::State::Loading).continue_to_state(state::State::MainMenu),
        )
        .add_collection_to_loading_state::<_, GameAssets>(state::State::Loading)
        .add_collection_to_loading_state::<_, AudioAssets>(state::State::Loading)
        .add_plugins(MainMenuPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(ResultsPlugin)
        .add_systems(Startup, state::results::create_player)
        .add_systems(OnEnter(state::State::Loading), state::loading::setup)
        .add_systems(OnExit(state::State::Loading), state::loading::cleanup)
        .add_audio_channel::<MusicChannel>()
        .add_audio_channel::<DrumsChannel>()
        .add_audio_channel::<SoundChannel>()
        .insert_resource(audio::VolumeSettings::default())
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .run();
}
